use anyhow::{Result, ensure};
use std::{
    fs,
    io::{ErrorKind, Write},
    path::Path,
    sync::Barrier,
    thread,
};

pub fn run(root: &Path) -> Result<()> {
    let scratch = tempfile::tempdir_in(root)?;
    create_if_absent(scratch.path())?;
    #[cfg(windows)]
    windows::protected_replacement(scratch.path())?;
    #[cfg(not(windows))]
    println!("UNQUALIFIED: existing-file replacement requires the unapplied-patch fallback.");
    Ok(())
}

fn create_if_absent(dir: &Path) -> Result<()> {
    let stage = |bytes: &[u8]| -> Result<tempfile::NamedTempFile> {
        let mut file = tempfile::NamedTempFile::new_in(dir)?;
        file.write_all(bytes)?;
        file.as_file().sync_all()?;
        Ok(file)
    };
    let first = stage(b"first creator\n")?;
    let second = stage(b"second creator\n")?;
    let destination = dir.join("creation.txt");
    let barrier = Barrier::new(2);
    let publish = |file: &tempfile::NamedTempFile| {
        barrier.wait();
        fs::hard_link(file.path(), &destination)
    };
    let (left, right) = thread::scope(|threads| {
        let other = threads.spawn(|| publish(&first));
        let right = publish(&second);
        (other.join().expect("creator did not panic"), right)
    });
    let winner = match (left, right) {
        (Ok(()), Err(e)) if e.kind() == ErrorKind::AlreadyExists => b"first creator\n".as_slice(),
        (Err(e), Ok(())) if e.kind() == ErrorKind::AlreadyExists => b"second creator\n".as_slice(),
        (left, right) => anyhow::bail!("create-if-absent is unqualified: {left:?}, {right:?}"),
    };
    drop((first, second));
    ensure!(
        fs::read(destination)? == winner,
        "create race lost the winner's bytes"
    );
    println!(
        "PASS: one hard-link publication succeeded, and the losing creator preserved the winner's bytes."
    );
    Ok(())
}

#[cfg(windows)]
mod windows {
    use super::*;
    use std::{
        fs::{File, OpenOptions},
        io::{self, Read},
        os::windows::{fs::OpenOptionsExt, io::AsRawHandle},
    };
    use windows_sys::Win32::{
        Foundation::{ERROR_ACCESS_DENIED, ERROR_SHARING_VIOLATION},
        Storage::FileSystem::{
            BY_HANDLE_FILE_INFORMATION, FILE_SHARE_READ, GetFileInformationByHandle,
        },
    };

    #[derive(Debug, PartialEq, Eq)]
    struct Base {
        identity: (u32, u32, u32),
        bytes: Vec<u8>,
    }

    fn observe(mut file: &File) -> Result<Base> {
        let mut info = BY_HANDLE_FILE_INFORMATION::default();
        // The borrowed file keeps the handle valid for the call.
        if unsafe { GetFileInformationByHandle(file.as_raw_handle(), &mut info) } == 0 {
            return Err(io::Error::last_os_error().into());
        }
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)?;
        Ok(Base {
            identity: (
                info.dwVolumeSerialNumber,
                info.nFileIndexHigh,
                info.nFileIndexLow,
            ),
            bytes,
        })
    }

    fn guard(target: &Path) -> io::Result<File> {
        OpenOptions::new()
            .read(true)
            .share_mode(FILE_SHARE_READ)
            .open(target)
    }

    fn assert_blocked(result: io::Result<()>) -> Result<()> {
        match result {
            Err(error)
                if error.raw_os_error() == Some(ERROR_SHARING_VIOLATION as i32)
                    || error.raw_os_error() == Some(ERROR_ACCESS_DENIED as i32) =>
            {
                Ok(())
            }
            other => anyhow::bail!("expected Windows to refuse the attempted write: {other:?}"),
        }
    }

    pub(super) fn protected_replacement(dir: &Path) -> Result<()> {
        let target = dir.join("existing.txt");
        let staged = dir.join("replacement.txt");
        let competing = dir.join("editor-save.txt");
        let original = b"original\n";
        fs::write(&target, original)?;
        fs::write(&staged, b"proposed edit\n")?;
        let base = observe(&File::open(&target)?)?;

        // Positive controls. If the writer goes first, a fresh guarded read must
        // detect a stale base, even if a save-by-rename retains the same bytes.
        fs::write(&target, b"editor in-place save\n")?;
        ensure!(
            observe(&guard(&target)?)? != base,
            "in-place save escaped base check"
        );
        fs::write(&target, original)?;
        fs::write(&competing, original)?;
        fs::rename(&competing, &target)?;
        let renamed = observe(&File::open(&target)?)?;
        ensure!(
            renamed.bytes == base.bytes && renamed.identity != base.identity,
            "same-bytes editor rename escaped the identity check"
        );

        let held = guard(&target)?;
        ensure!(observe(&held)? == renamed, "guarded base differs");
        let writer = thread::scope(|threads| {
            threads
                .spawn(|| -> Result<()> {
                    assert_blocked(fs::write(&target, b"competing write\n"))?;
                    fs::write(&competing, b"competing rename\n")?;
                    assert_blocked(fs::rename(&competing, &target))
                })
                .join()
                .expect("writer did not panic")
        });
        writer?;
        assert_blocked(fs::rename(&staged, &target))?;
        drop(held);
        ensure!(
            fs::read(&target)? == original,
            "fallback changed the destination"
        );
        ensure!(
            fs::read(&staged)? == b"proposed edit\n",
            "staged proposal was lost"
        );
        println!(
            "PASS: the share-mode guard detected earlier saves and blocked both competing writers."
        );
        println!("UNAPPLIED: the share-mode guard also blocked the proposed replacement.");
        print!("--- a/existing.txt\n+++ b/existing.txt\n@@ -1 +1 @@\n-original\n+proposed edit\n");
        Ok(())
    }
}
