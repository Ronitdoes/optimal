//! Win32 startup guards (ADR-002/003/004).
//! * Per-monitor DPI awareness must be set before any HWND is created.
//! * A named mutex gives single-instance behaviour.
//! * An elevated token is refused: the agent never runs elevated.

use windows::core::w;
use windows::Win32::Foundation::{
    GetLastError, SetLastError, ERROR_ALREADY_EXISTS, HANDLE, WIN32_ERROR,
};
use windows::Win32::System::Threading::CreateMutexW;
use windows::Win32::UI::HiDpi::{
    SetProcessDpiAwarenessContext, DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2,
};
use windows::Win32::UI::Shell::IsUserAnAdmin;

/// Keep alive for the process lifetime; dropping releases the reference.
/// The field is intentionally unread: its Drop/RAII lifetime is the point.
#[allow(dead_code)]
pub struct SingleInstance(pub HANDLE);

pub fn set_per_monitor_dpi_awareness() {
    // A failure here usually means a compatibility layer already fixed the
    // context; never abort startup, but surface it.
    if let Err(e) = unsafe { SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2) } {
        eprintln!("WARN: SetProcessDpiAwarenessContext failed: {e}");
    }
}

pub fn acquire_single_instance() -> windows::core::Result<Option<SingleInstance>> {
    unsafe { SetLastError(WIN32_ERROR(0)) };
    let handle = unsafe {
        CreateMutexW(
            None,
            true,
            w!("Local\\Optimal-Single-InstanceMutex-7c2f9a1e"),
        )
    }?;
    if unsafe { GetLastError() } == ERROR_ALREADY_EXISTS {
        return Ok(None);
    }
    Ok(Some(SingleInstance(handle)))
}

pub fn is_elevated() -> bool {
    unsafe { IsUserAnAdmin() }.as_bool()
}
