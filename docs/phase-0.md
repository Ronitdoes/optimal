# Phase 0 foundation probes

The workspace has one package with a GPUI-independent protocol library, a CLI, and an optional Windows GPUI binary. Both clients send the same typed, versioned command to a Tokio worker. Each demonstration emits a start event, five progress events and a completion event over a bounded channel with sequences 1 through 7.

## Reproduce on Windows x64

Install Rustup and put the 64-bit compiler before any older MinGW installations on `PATH`. The Windows reference build uses w64devkit 2.11.0 for the C dependencies and native build tools. The Rust toolchain is selected by [rust-toolchain.toml](../rust-toolchain.toml). [Cargo.lock](../Cargo.lock) pins the GPUI 0.2.2 registry artifact by checksum. The published package records Zed commit `69e2130295c2649963eb639fc70b4f2ee8ea1624` and a dirty source tree, so the registry artifact is the reproducible pin. [w64devkit 2.11.0](https://github.com/skeeto/w64devkit/releases/tag/v2.11.0)

```powershell
# Select the runtime libraries shipped with Rust for the GNU linker.
$env:RUSTFLAGS = '-C link-self-contained=yes'
$env:CARGO_PROFILE_DEV_DEBUG = '0'

# w64devkit 2.11.0 omits the import library for this Windows system DLL.
if ((gcc.exe -print-file-name=libktmw32.a) -eq 'libktmw32.a') {
    $imports = New-Item -ItemType Directory -Force -Path target/phase0-imports
    Push-Location $imports.FullName
    try {
        gendef.exe "$env:SystemRoot/System32/ktmw32.dll"
        if ($LASTEXITCODE -ne 0) { throw 'gendef failed' }
        dlltool.exe --dllname ktmw32.dll --input-def ktmw32.def --output-lib libktmw32.a
        if ($LASTEXITCODE -ne 0) { throw 'dlltool failed' }
    } finally {
        Pop-Location
    }
    $env:LIBRARY_PATH = $imports.FullName
}
cargo build --locked --features desktop

cargo run --locked -- events
cargo run --locked -- http
cargo run --locked -- sqlite
cargo run --locked -- files
cargo run --locked -- isolation
cargo run --locked --features desktop --bin optimal-desktop -- --smoke
```

Run the desktop command without `--smoke` to keep the window open. Click **Check responsiveness** while progress is streaming. Smoke mode verifies that the window renders multiple times while receiving events, checks the final sequence and exits automatically. Use a Windows session with a display and a working Direct3D driver.

The SQLite, filesystem and isolation probes accept a scratch parent as an optional argument. They create temporary files below that directory, which defaults to the current working directory. Choose a parent on the filesystem under evaluation. Use the debug build for the desktop check. GPUI's release build requires the Windows SDK shader compiler.

## What each probe checks

| Command | Evidence produced |
|---|---|
| `events` | JSON events with protocol version, run ID, sequence and payload. |
| `http` | Reqwest reads the first part of a local chunked response while the server withholds the second part. The fixture validates the complete body and has a five-second timeout. |
| `sqlite` | A blocking worker writes events in WAL transactions. A duplicate sequence is rejected and a reopened connection replays the exact event stream. |
| `files` | Two creators race to publish synced, staged bytes. Exactly one wins. The Windows check tests an in-place save and a same-bytes save by rename before acquiring the guard, then checks the effect of both saves with the guard held. |
| `isolation` | An AppContainer with zero capabilities reads a staged fixture and receives access denied on protected paths and writes. The TCP attempt must be denied or time out without reaching the host listener, which accepts unrestricted connections before and after the attempt. Closing the Job Object must terminate the reported descendant. |
| Desktop `--smoke` | A native GPUI window renders multiple times during the Tokio run and observes all sequenced events through completion. |

## Filesystem strategy

Create-if-absent stages completed bytes in the target directory and publishes with `std::fs::hard_link`. Linking introduces the final name in one operation and fails if that name already exists. It requires same-volume hard-link support. The probe tests the commit primitive in a fresh scratch directory. Workspace path authorization, link and junction handling, and crash durability remain separate qualification work. [CreateHardLinkW](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-createhardlinkw)

For an existing file, a Windows read handle opened with only `FILE_SHARE_READ` supplies the observed file identity and bytes. Holding it blocks the tested in-place write, save by rename and proposed replacement. Closing it before publishing introduces a race. Existing-file edits on the Windows filesystems covered by this prototype therefore use the **unapplied patch** fallback, and a mismatch from the observed base is a conflict. No existing-file replacement mechanism is qualified. [CreateFileW](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfilew)

## Isolation strategy

The Windows candidate uses a temporary, capability-free AppContainer with an ACL grant on a staged read-only directory. The probe launches its own fixture executable in a kill-on-close Job Object via process creation attributes. It supplies only the Windows-required `LOCALAPPDATA` and `SystemRoot` environment entries and inherits no handles. Setup or verification failure returns an error and leaves tool execution unavailable. The runner only launches the probe fixture. [AppContainer isolation](https://learn.microsoft.com/en-us/windows/win32/secauthz/appcontainer-isolation), [Windows Job Objects](https://learn.microsoft.com/en-us/windows/win32/procthread/job-objects)

The evidence covers the staged ACL, a protected sibling file, a reachable loopback endpoint and process-tree cleanup. Release qualification still needs a sanitized filesystem view for real repositories, testing of external network restrictions and the release checks in the roadmap.

## Verification record

Ran on November 22, 2026 on Windows x64, build 26300, with a local NTFS scratch directory. The locked debug build, formatting check, and Clippy with `-D warnings` passed under Rust 1.100.0. Cargo reported a future-compatibility warning in the transitive `proc-macro-error2` dependency.

| Check | Observed result |
|---|---|
| CLI and GPUI | CLI emitted sequences 1–7 in order. The native window rendered multiple times before consuming the completion event. |
| Streaming HTTP | The client consumed the first body part while the server held the second. |
| SQLite | Reopening the database reproduced all seven events. SQLite rejected a duplicate sequence. |
| NTFS create and replace | Exactly one hard link published. A write and a same-bytes rename before the guard were both detected. While guarded, both competing saves and the proposed replacement were blocked. The original bytes were preserved and the patch was printed as unapplied. |
| AppContainer and Job Object | The staged read succeeded. The protected read, protected write and staged write received access denied. The restricted loopback connection timed out while both host controls succeeded. The reported descendant was live in the job and terminated when its handle closed. |
