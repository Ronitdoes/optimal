#[cfg(windows)]
use anyhow::Context;
use anyhow::Result;
use std::path::Path;

#[cfg(windows)]
mod windows;
#[cfg(windows)]
pub use windows::fixture_exit_code;

#[cfg(windows)]
pub fn run(root: &Path) -> Result<()> {
    windows::run(root)
        .context("restricted tool execution is unavailable because the isolation probe failed")
}

#[cfg(not(windows))]
pub fn run(_: &Path) -> Result<()> {
    anyhow::bail!("restricted tool execution is unavailable on this platform")
}
