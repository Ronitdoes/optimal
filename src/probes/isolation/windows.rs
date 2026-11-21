use anyhow::{Context, Result, bail, ensure};
use std::{
    env,
    ffi::{OsStr, OsString},
    fs,
    io::{self, ErrorKind},
    net::{SocketAddr, TcpListener, TcpStream},
    os::windows::{
        ffi::{OsStrExt, OsStringExt},
        io::{AsRawHandle, FromRawHandle, OwnedHandle},
    },
    path::{Path, PathBuf},
    process::Command,
    ptr::{null, null_mut},
    thread,
    time::Duration,
};
use windows_sys::Win32::{
    Foundation::{HANDLE, LocalFree, WAIT_OBJECT_0, WAIT_TIMEOUT},
    Networking::WinSock::WSAEACCES,
    Security::{
        Authorization::ConvertSidToStringSidW,
        FreeSid, GetTokenInformation,
        Isolation::{CreateAppContainerProfile, DeleteAppContainerProfile},
        PSID, SECURITY_CAPABILITIES, TOKEN_QUERY, TokenIsAppContainer,
    },
    System::{
        JobObjects::{
            CreateJobObjectW, IsProcessInJob, JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE,
            JOBOBJECT_EXTENDED_LIMIT_INFORMATION, JobObjectExtendedLimitInformation,
            SetInformationJobObject,
        },
        Threading::{
            CREATE_NO_WINDOW, CREATE_UNICODE_ENVIRONMENT, CreateProcessW,
            DeleteProcThreadAttributeList, EXTENDED_STARTUPINFO_PRESENT, GetCurrentProcess,
            GetExitCodeProcess, InitializeProcThreadAttributeList, LPPROC_THREAD_ATTRIBUTE_LIST,
            OpenProcess, OpenProcessToken, PROC_THREAD_ATTRIBUTE_JOB_LIST,
            PROC_THREAD_ATTRIBUTE_SECURITY_CAPABILITIES, PROCESS_INFORMATION,
            PROCESS_QUERY_LIMITED_INFORMATION, PROCESS_SYNCHRONIZE, STARTUPINFOEXW,
            UpdateProcThreadAttribute, WaitForSingleObject,
        },
    },
};

const FIXTURE: &str = "__phase0_isolation_fixture";
const DESCENDANT: &str = "__phase0_isolation_descendant";
const ALLOWED: &[u8] = b"allowed phase 0 data";
const OUTSIDE: &[u8] = b"protected phase 0 data";
// Windows uses these to construct the restricted process's environment.
const WINDOWS_ENV_KEYS: [&str; 2] = ["LOCALAPPDATA", "SystemRoot"];
// The fixture reports its descendant PID in the low bits of a successful exit code.
const PID_MASK: u32 = 0x0fff_ffff;
const CONNECTION_DENIED: u32 = 0x2000_0000;
const CONNECTION_TIMED_OUT: u32 = 0x3000_0000;

pub(super) fn run(root: &Path) -> Result<()> {
    let scratch = tempfile::tempdir_in(root)?;
    let allowed = scratch.path().join("staged");
    fs::create_dir(&allowed)?;
    let outside = scratch.path().join("protected.txt");
    fs::write(&outside, OUTSIDE)?;
    fs::write(allowed.join("allowed.txt"), ALLOWED)?;
    let executable = allowed.join("probe.exe");
    fs::copy(env::current_exe()?, &executable)?;

    let name = format!(
        "Optimal.Phase0.{}.{}",
        std::process::id(),
        scratch
            .path()
            .file_name()
            .context("scratch directory has no name")?
            .to_string_lossy()
    );
    let profile = Profile::create(&name)?;
    profile.grant_read_execute(&allowed)?;

    let listener = TcpListener::bind("127.0.0.1:0").context("bind loopback control")?;
    let address = listener.local_addr()?;
    listener.set_nonblocking(true)?;
    verify_host_connection(&listener, address)?;

    let job = create_job()?;
    let process = launch(&profile, &job, &executable, &allowed, address)?;
    wait(&process, "restricted fixture")?;
    let mut code = 0;
    // The child handle remains valid while the exit status is retrieved.
    check(unsafe { GetExitCodeProcess(process.as_raw_handle(), &mut code) })?;
    let network_result = match code & !PID_MASK {
        CONNECTION_DENIED => "Windows returned WSAEACCES",
        CONNECTION_TIMED_OUT => "the restricted connection timed out",
        _ => match code {
            10 => bail!("fixture token did not establish AppContainer isolation"),
            11 => bail!("fixture could not read the granted file"),
            12 => bail!("fixture did not receive access denied reading the protected file"),
            13 => bail!("fixture did not receive access denied writing to the read-only tree"),
            14 => bail!("fixture did not receive access denied writing the protected file"),
            15 => bail!("the restricted fixture connected to the reachable listener"),
            16 => bail!("restricted fixture could not launch and report its descendant"),
            18 => bail!("the restricted fixture had no usable loopback endpoint or socket error"),
            10000..=11999 => {
                bail!("the restricted fixture received unexpected Winsock error {code}")
            }
            _ => bail!("restricted fixture exited with status {code:#010x}"),
        },
    };
    ensure!(fs::read(outside)? == OUTSIDE, "the protected file changed");
    ensure!(
        matches!(listener.accept(), Err(error) if error.kind() == ErrorKind::WouldBlock),
        "the listener received an unexpected connection during the restricted attempt"
    );
    verify_host_connection(&listener, address)?;
    println!(
        "PASS: AppContainer permitted the staged read, denied protected access and denied writes."
    );
    println!(
        "PASS: the reachable loopback listener received no restricted connection ({network_result})."
    );

    let pid = code & PID_MASK;
    ensure!(pid != 0, "fixture did not report a descendant PID");
    let descendant = owned(unsafe {
        OpenProcess(
            PROCESS_SYNCHRONIZE | PROCESS_QUERY_LIMITED_INFORMATION,
            0,
            pid,
        )
    })
    .with_context(|| format!("open reported descendant {pid}"))?;
    let mut in_job = 0;
    check(unsafe { IsProcessInJob(descendant.as_raw_handle(), job.as_raw_handle(), &mut in_job) })?;
    ensure!(
        in_job != 0,
        "the reported descendant was outside the Job Object"
    );
    ensure!(
        unsafe { WaitForSingleObject(descendant.as_raw_handle(), 0) } == WAIT_TIMEOUT,
        "the descendant exited before the job was closed"
    );
    drop(job);
    wait(&descendant, "job descendant")?;
    println!("PASS: closing the Job Object terminated the surviving descendant.");
    Ok(())
}

pub fn fixture_exit_code(args: &[OsString]) -> Option<i32> {
    match args {
        [flag, address] if flag == FIXTURE => Some(run_fixture(address)),
        [flag] if flag == DESCENDANT => {
            if !matches!(is_app_container(), Ok(true)) {
                return Some(10);
            }
            thread::sleep(Duration::from_secs(30));
            Some(17)
        }
        _ => None,
    }
}

fn run_fixture(address: &OsStr) -> i32 {
    if !matches!(is_app_container(), Ok(true)) {
        return 10;
    }
    if !matches!(fs::read("allowed.txt"), Ok(bytes) if bytes == ALLOWED) {
        return 11;
    }
    if !denied(fs::read("../protected.txt")) {
        return 12;
    }
    if !denied(fs::write("attempt.txt", b"blocked")) {
        return 13;
    }
    if !denied(fs::OpenOptions::new().write(true).open("../protected.txt")) {
        return 14;
    }
    let Some(address) = address
        .to_str()
        .and_then(|address| address.parse::<SocketAddr>().ok())
        .filter(|address| address.ip().is_loopback())
    else {
        return 18;
    };
    let network_result = match TcpStream::connect_timeout(&address, Duration::from_secs(2)) {
        Ok(_) => return 15,
        Err(error) if error.raw_os_error() == Some(WSAEACCES) => CONNECTION_DENIED,
        Err(error) if error.kind() == ErrorKind::TimedOut => CONNECTION_TIMED_OUT,
        Err(error) => {
            return error
                .raw_os_error()
                .filter(|code| (10000..=11999).contains(code))
                .unwrap_or(18);
        }
    };
    // The fixture only runs a second copy of its own executable.
    let child = (|| -> Result<u32> {
        let child = Command::new(env::current_exe()?)
            .arg(DESCENDANT)
            .env_clear()
            .envs(required_environment()?)
            .spawn()?;
        Ok(child.id())
    })();
    match child {
        Ok(pid) if pid != 0 && pid <= PID_MASK => (network_result | pid) as i32,
        _ => 16,
    }
}

fn verify_host_connection(listener: &TcpListener, address: SocketAddr) -> Result<()> {
    let control = TcpStream::connect_timeout(&address, Duration::from_secs(2))
        .context("the unrestricted host must be able to reach the listener")?;
    let (_, peer) = listener
        .accept()
        .context("accept the unrestricted control connection")?;
    ensure!(
        peer == control.local_addr()?,
        "unexpected control connection"
    );
    Ok(())
}

fn denied<T>(result: io::Result<T>) -> bool {
    matches!(result, Err(error) if error.kind() == ErrorKind::PermissionDenied)
}

fn is_app_container() -> Result<bool> {
    let mut token = null_mut();
    check(unsafe { OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token) })?;
    let token = owned(token)?;
    let mut is_container: u32 = 0;
    let mut returned = 0;
    check(unsafe {
        GetTokenInformation(
            token.as_raw_handle(),
            TokenIsAppContainer,
            (&mut is_container as *mut u32).cast(),
            size_of::<u32>() as u32,
            &mut returned,
        )
    })?;
    Ok(is_container != 0)
}

struct Profile {
    name: Vec<u16>,
    sid: PSID,
}

impl Profile {
    fn create(name: &str) -> Result<Self> {
        let name = wide(name);
        let label = wide("Optimal Phase 0 isolation probe");
        let mut sid = null_mut();
        let status = unsafe {
            CreateAppContainerProfile(
                name.as_ptr(),
                label.as_ptr(),
                label.as_ptr(),
                null(),
                0,
                &mut sid,
            )
        };
        ensure!(
            status == 0,
            "CreateAppContainerProfile failed with HRESULT {:#010x}",
            status as u32
        );
        Ok(Self { name, sid })
    }

    fn grant_read_execute(&self, directory: &Path) -> Result<()> {
        let mut text = null_mut();
        check(unsafe { ConvertSidToStringSidW(self.sid, &mut text) })?;
        // Windows supplies a NUL-terminated SID string allocated using LocalAlloc.
        let sid = unsafe {
            let len = (0..).find(|&index| *text.add(index) == 0).unwrap();
            let string = OsString::from_wide(std::slice::from_raw_parts(text, len));
            LocalFree(text.cast());
            string
        };
        let mut grant = OsString::from("*");
        grant.push(sid);
        grant.push(":(OI)(CI)(RX)");
        let icacls = PathBuf::from(env::var_os("SystemRoot").context("SystemRoot is unset")?)
            .join("System32/icacls.exe");
        let output = Command::new(icacls)
            .arg(directory)
            .arg("/grant")
            .arg(grant)
            .output()?;
        ensure!(
            output.status.success(),
            "icacls failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        Ok(())
    }
}

impl Drop for Profile {
    fn drop(&mut self) {
        unsafe {
            FreeSid(self.sid);
            let status = DeleteAppContainerProfile(self.name.as_ptr());
            if status != 0 {
                eprintln!(
                    "Deleting the temporary AppContainer profile failed with HRESULT {:#010x}",
                    status as u32
                );
            }
        }
    }
}

fn create_job() -> Result<OwnedHandle> {
    let job = owned(unsafe { CreateJobObjectW(null(), null()) })?;
    let mut limits = JOBOBJECT_EXTENDED_LIMIT_INFORMATION::default();
    limits.BasicLimitInformation.LimitFlags = JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE;
    check(unsafe {
        SetInformationJobObject(
            job.as_raw_handle(),
            JobObjectExtendedLimitInformation,
            (&limits as *const JOBOBJECT_EXTENDED_LIMIT_INFORMATION).cast(),
            size_of::<JOBOBJECT_EXTENDED_LIMIT_INFORMATION>() as u32,
        )
    })?;
    Ok(job)
}

struct Attributes {
    storage: Vec<usize>,
}

impl Attributes {
    fn new() -> Result<Self> {
        let mut bytes = 0;
        // The sizing call deliberately supplies no buffer.
        unsafe { InitializeProcThreadAttributeList(null_mut(), 2, 0, &mut bytes) };
        ensure!(bytes > 0, "process attribute-list sizing failed");
        let mut storage = vec![0; bytes.div_ceil(size_of::<usize>())];
        check(unsafe {
            InitializeProcThreadAttributeList(storage.as_mut_ptr().cast(), 2, 0, &mut bytes)
        })?;
        Ok(Self { storage })
    }

    fn raw(&mut self) -> LPPROC_THREAD_ATTRIBUTE_LIST {
        self.storage.as_mut_ptr().cast()
    }

    // Attribute values are borrowed by the Windows list. The caller keeps them alive through launch.
    unsafe fn add<T>(&mut self, attribute: u32, value: &T) -> io::Result<()> {
        check(unsafe {
            UpdateProcThreadAttribute(
                self.raw(),
                0,
                attribute as usize,
                (value as *const T).cast(),
                size_of::<T>(),
                null_mut(),
                null(),
            )
        })
    }
}

impl Drop for Attributes {
    fn drop(&mut self) {
        unsafe { DeleteProcThreadAttributeList(self.raw()) };
    }
}

fn launch(
    profile: &Profile,
    job: &OwnedHandle,
    exe: &Path,
    cwd: &Path,
    address: SocketAddr,
) -> Result<OwnedHandle> {
    let capabilities = SECURITY_CAPABILITIES {
        AppContainerSid: profile.sid,
        ..SECURITY_CAPABILITIES::default()
    };
    let jobs = [job.as_raw_handle()];
    let mut attributes = Attributes::new()?;
    unsafe {
        attributes.add(PROC_THREAD_ATTRIBUTE_SECURITY_CAPABILITIES, &capabilities)?;
        attributes.add(PROC_THREAD_ATTRIBUTE_JOB_LIST, &jobs)?;
    }
    let mut startup = STARTUPINFOEXW::default();
    startup.StartupInfo.cb = size_of::<STARTUPINFOEXW>() as u32;
    startup.lpAttributeList = attributes.raw();
    let mut process = PROCESS_INFORMATION::default();
    let exe = wide(exe.as_os_str());
    let cwd = wide(cwd.as_os_str());
    // argv[0] is only a label because lpApplicationName selects the executable.
    let mut arguments = wide(format!("probe.exe {FIXTURE} {address}"));
    let mut environment = Vec::new();
    for (key, value) in required_environment()? {
        let mut assignment = OsString::from(key);
        assignment.push("=");
        assignment.push(value);
        environment.extend(wide(assignment));
    }
    environment.push(0);
    check(unsafe {
        CreateProcessW(
            exe.as_ptr(),
            arguments.as_mut_ptr(),
            null(),
            null(),
            0,
            EXTENDED_STARTUPINFO_PRESENT | CREATE_NO_WINDOW | CREATE_UNICODE_ENVIRONMENT,
            environment.as_ptr().cast(),
            cwd.as_ptr(),
            &startup.StartupInfo,
            &mut process,
        )
    })
    .context("launch AppContainer process inside Job Object")?;
    let _thread = owned(process.hThread)?;
    owned(process.hProcess).context("obtain the restricted process handle")
}

fn wait(process: &OwnedHandle, name: &str) -> Result<()> {
    match unsafe { WaitForSingleObject(process.as_raw_handle(), 5_000) } {
        WAIT_OBJECT_0 => Ok(()),
        WAIT_TIMEOUT => bail!("timed out waiting for {name}"),
        _ => Err(io::Error::last_os_error()).context(format!("wait for {name}")),
    }
}

fn wide(value: impl AsRef<OsStr>) -> Vec<u16> {
    value.as_ref().encode_wide().chain([0]).collect()
}

fn required_environment() -> Result<Vec<(&'static str, OsString)>> {
    WINDOWS_ENV_KEYS
        .into_iter()
        .map(|key| {
            let value = env::var_os(key).with_context(|| format!("{key} is unset"))?;
            Ok((key, value))
        })
        .collect()
}

fn check(result: i32) -> io::Result<()> {
    if result == 0 {
        return Err(io::Error::last_os_error());
    }
    Ok(())
}

fn owned(handle: HANDLE) -> io::Result<OwnedHandle> {
    if handle.is_null() {
        return Err(io::Error::last_os_error());
    }
    // Called only with fresh handles returned by Win32 process, token and job APIs.
    Ok(unsafe { OwnedHandle::from_raw_handle(handle) })
}
