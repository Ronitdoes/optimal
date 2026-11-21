mod probes;

use anyhow::{Result, anyhow, bail};
use optimal::protocol::{Command, connect};
use std::{path::Path, time::Duration};

fn main() -> Result<()> {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    #[cfg(windows)]
    if let Some(code) = probes::isolation::fixture_exit_code(&args) {
        std::process::exit(code);
    }
    run(args)
}

#[tokio::main]
async fn run(args: Vec<std::ffi::OsString>) -> Result<()> {
    let command = args.first().and_then(|arg| arg.to_str());
    match command {
        Some("events") if args.len() == 1 => {
            let (commands, mut events) = connect(&tokio::runtime::Handle::current());
            commands
                .send(Command::start_demo(1))
                .map_err(|_| anyhow!("the Tokio command receiver closed"))?;
            while let Some(event) = events.recv().await {
                println!("{}", serde_json::to_string(&event)?);
            }
        }
        Some("http") if args.len() == 1 => {
            tokio::time::timeout(Duration::from_secs(5), probes::http::run()).await??;
        }
        Some("sqlite" | "files" | "isolation") if args.len() <= 2 => {
            let current = std::env::current_dir()?;
            let root = args.get(1).map(Path::new).unwrap_or(&current);
            match command {
                Some("sqlite") => probes::sqlite::run(root).await?,
                Some("files") => probes::files::run(root)?,
                _ => probes::isolation::run(root)?,
            }
        }
        None if args.is_empty() => print_usage(),
        Some("--help" | "-h") if args.len() == 1 => print_usage(),
        _ => bail!("Unknown command or argument count. See optimal --help."),
    }
    Ok(())
}

fn print_usage() {
    println!("Usage: optimal <events|http|sqlite|files|isolation> [scratch-parent]");
    println!(
        "sqlite, files, and isolation use a temporary directory under the current directory or scratch-parent."
    );
}
