//! Optimal O0 — single Windows binary. Thin GPUI surface over a local JSON-RPC
//! gateway and a governed Rust core. Never elevated (ADR-004).

mod win32;

use std::sync::Arc;

use optimal_common::CorrelationId;
use optimal_gateway::{constant_time_eq, Capabilities, CoreApi, Gateway, StreamEvent};
use optimal_runtime::KillSwitch;
use optimal_store::EventStore;
use tokio::sync::mpsc;

struct Core {
    token: String,
    kill: KillSwitch,
    store: Arc<EventStore>,
}

impl CoreApi for Core {
    fn capabilities(&self) -> Capabilities { Capabilities::default() }

    fn authenticate(&self, token: &str) -> bool {
        constant_time_eq(token.as_bytes(), self.token.as_bytes())
    }

    fn run_task(&self, goal: &str, sink: mpsc::UnboundedSender<StreamEvent>) -> String {
        let task_id = CorrelationId::new().to_string();
        let goal = goal.to_string();
        let kill = self.kill.clone();
        let id = task_id.clone();
        tokio::spawn(async move {
            let _ = sink.send(StreamEvent::Token { text: format!("Hello — {goal}") });
            for tool in ["env.probe", "fs.read"] {
                if kill.is_killed() {
                    let _ = sink.send(StreamEvent::Killed { task_id: id.clone() });
                    return;
                }
                let _ = sink.send(StreamEvent::ToolBadge { id: tool.into(), tier: "readonly".into() });
            }
            let _ = sink.send(StreamEvent::Done { task_id: id });
        });
        task_id
    }

    fn interrupt(&self, _message: &str) { /* soft steering wired in O1 */ }

    fn kill(&self) { self.kill.kill(); }

    fn audit_verifies(&self) -> bool { self.store.verify_chain().unwrap_or(false) }
}

#[tokio::main]
async fn main() {
    optimal_telemetry::init_subscriber();

    // Ordering matters: DPI awareness and the instance lock precede any window.
    win32::set_per_monitor_dpi_awareness();

    let _single_instance = match win32::acquire_single_instance() {
        Ok(Some(guard)) => guard,
        Ok(None) => {
            eprintln!("Optimal is already running; exiting.");
            return;
        }
        Err(e) => {
            eprintln!("REFUSED: could not acquire single-instance lock: {e}");
            return;
        }
    };

    if win32::is_elevated() {
        eprintln!("REFUSED: Optimal must not run elevated (ADR-004).");
        return;
    }

    let data_dir = std::path::PathBuf::from(std::env::var("LOCALAPPDATA").unwrap_or_else(|_| ".".into()))
        .join("optimal");
    let store = match EventStore::open(data_dir.join("events.db")) {
        Ok(s) => Arc::new(s),
        Err(e) => {
            eprintln!("REFUSED: event store could not open: {e}");
            return;
        }
    };

    let token = std::env::var("OPTIMAL_RPC_TOKEN")
        .unwrap_or_else(|_| CorrelationId::new().to_string());
    let core = Arc::new(Core { token: token.clone(), kill: KillSwitch::default(), store });

    let gateway = Gateway::new(core, "127.0.0.1:0");
    let addr = match gateway.bind().await {
        Ok(a) => a,
        Err(e) => {
            eprintln!("REFUSED: loopback gateway could not bind: {e}");
            return;
        }
    };
    eprintln!("optimal v{} gateway={addr} token={token}", env!("CARGO_PKG_VERSION"));

    #[cfg(feature = "ui")]
    {
        optimal_ui::run_ui();
    }

    #[cfg(not(feature = "ui"))]
    {
        eprintln!("headless build (ui feature off); press Ctrl+C to exit");
        let _ = tokio::signal::ctrl_c().await;
    }
}
