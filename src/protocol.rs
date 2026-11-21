use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::sync::{mpsc, oneshot};

pub const VERSION: u16 = 1;
pub const DEMO_STEPS: u8 = 5;

#[derive(Debug, Serialize, Deserialize)]
pub struct Command {
    pub version: u16,
    pub run_id: u64,
    pub kind: CommandKind,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum CommandKind {
    StartDemo,
}

impl Command {
    pub fn start_demo(run_id: u64) -> Self {
        Self {
            version: VERSION,
            run_id,
            kind: CommandKind::StartDemo,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Event {
    pub version: u16,
    pub run_id: u64,
    pub sequence: u64,
    pub kind: EventKind,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum EventKind {
    Started,
    Progress { step: u8 },
    Completed,
    Rejected { supported_version: u16 },
}

/// An in-process client connection with one command and a bounded event queue.
pub fn connect(
    runtime: &tokio::runtime::Handle,
) -> (oneshot::Sender<Command>, mpsc::Receiver<Event>) {
    let (commands, incoming) = oneshot::channel();
    let (outgoing, events) = mpsc::channel(2);
    runtime.spawn(async move {
        if let Ok(command) = incoming.await {
            serve(command, outgoing).await;
        }
    });
    (commands, events)
}

async fn serve(command: Command, outgoing: mpsc::Sender<Event>) {
    let event = |sequence, kind| Event {
        version: VERSION,
        run_id: command.run_id,
        sequence,
        kind,
    };
    if command.version != VERSION {
        let kind = EventKind::Rejected {
            supported_version: VERSION,
        };
        let _ = outgoing.send(event(1, kind)).await;
        return;
    }

    match command.kind {
        CommandKind::StartDemo => {
            if outgoing.send(event(1, EventKind::Started)).await.is_err() {
                return;
            }
            for step in 1..=DEMO_STEPS {
                tokio::select! {
                    () = outgoing.closed() => return,
                    () = tokio::time::sleep(Duration::from_millis(400)) => {}
                }
                let progress = event(u64::from(step) + 1, EventKind::Progress { step });
                if outgoing.send(progress).await.is_err() {
                    return;
                }
            }
            let completed = event(u64::from(DEMO_STEPS) + 2, EventKind::Completed);
            let _ = outgoing.send(completed).await;
        }
    }
}
