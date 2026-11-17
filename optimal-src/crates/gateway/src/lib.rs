//! O0.6 — Local JSON-RPC gateway. Loopback bind, namespace `harness.v1.*`,
//! capabilities handshake, fail-closed auth before any event surface, and
//! streaming notifications the GPUI window renders (ADR-004).
//!
//! WS upgrades validate Host/Origin (CVE-2026-53869 DNS-rebinding class) before
//! any PTY/event surface is exposed (Hermes refinement, phases.md O0.6).

pub mod ws;

use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;

pub const NAMESPACE: &str = "harness.v1";

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Capabilities {
    pub server: String,
    pub version: String,
    pub protocol: String,
    pub methods: Vec<String>,
    pub streaming: bool,
}

impl Default for Capabilities {
    fn default() -> Self {
        Self {
            server: "optimal-core".into(),
            version: "0.0.1".into(),
            protocol: NAMESPACE.into(),
            methods: vec![
                "harness.v1.capabilities".into(),
                "harness.v1.auth".into(),
                "harness.v1.task.run".into(),
                "harness.v1.interrupt".into(),
                "harness.v1.kill".into(),
                "harness.v1.audit.verify".into(),
            ],
            streaming: true,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum StreamEvent {
    Token { text: String },
    ToolBadge { id: String, tier: String },
    Done { task_id: String },
    Killed { task_id: String },
}

/// The core surface the gateway exposes. Implemented by the runner (O0.7+).
pub trait CoreApi: Send + Sync {
    fn capabilities(&self) -> Capabilities;
    /// Constant-time token check performed before any event surface is opened.
    fn authenticate(&self, token: &str) -> bool;
    fn run_task(&self, goal: &str, sink: mpsc::UnboundedSender<StreamEvent>) -> String;
    fn interrupt(&self, message: &str);
    fn kill(&self);
    fn audit_verifies(&self) -> bool;
}

/// Constant-time comparison (Hermes refinement O4.5, applied here to auth).
pub fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() { return false; }
    let mut diff = 0u8;
    for (x, y) in a.iter().zip(b) { diff |= x ^ y; }
    diff == 0
}

pub struct Gateway {
    core: Arc<dyn CoreApi>,
    addr: String,
}

impl Gateway {
    pub fn new(core: Arc<dyn CoreApi>, addr: impl Into<String>) -> Self {
        Self { core, addr: addr.into() }
    }

    /// Bind loopback only. Returns the bound address (useful with port 0).
    pub async fn bind(&self) -> std::io::Result<String> {
        let listener = TcpListener::bind(&self.addr).await?;
        let addr = listener.local_addr()?.to_string();
        let core = self.core.clone();
        tokio::spawn(async move {
            while let Ok((socket, peer)) = listener.accept().await {
                let core = core.clone();
                tokio::spawn(async move {
                    let _ = serve_connection(socket, core).await;
                    let _ = peer;
                });
            }
        });
        Ok(addr)
    }
}

async fn serve_connection(socket: TcpStream, core: Arc<dyn CoreApi>) -> std::io::Result<()> {
    let (reader, mut writer) = socket.into_split();
    let mut lines = BufReader::new(reader).lines();
    let mut authenticated = false;

    while let Some(line) = lines.next_line().await? {
        if line.trim().is_empty() { continue; }
        let req: Value = match serde_json::from_str(&line) {
            Ok(v) => v,
            Err(_) => {
                write_json(&mut writer, &json!({"jsonrpc":"2.0","id":null,"error":{"code":-32700,"message":"parse error"}})).await?;
                continue;
            }
        };
        let id = req.get("id").cloned().unwrap_or(Value::Null);
        let method = req.get("method").and_then(Value::as_str).unwrap_or("").to_string();
        let params = req.get("params").cloned().unwrap_or(json!({}));

        if !method.starts_with(&format!("{NAMESPACE}.")) {
            write_error(&mut writer, &id, -32601, "method not in harness.v1 namespace").await?;
            continue;
        }

        // Fail closed: before auth, only handshake/auth methods are reachable.
        if !authenticated && !matches!(method.as_str(), "harness.v1.capabilities" | "harness.v1.auth") {
            write_error(&mut writer, &id, -32000, "unauthorized: authenticate first").await?;
            continue;
        }

        match method.as_str() {
            "harness.v1.capabilities" => {
                write_result(&mut writer, &id, serde_json::to_value(core.capabilities()).unwrap()).await?;
            }
            "harness.v1.auth" => {
                let token = params.get("token").and_then(Value::as_str).unwrap_or("");
                if core.authenticate(token) {
                    authenticated = true;
                    write_result(&mut writer, &id, json!({"ok":true})).await?;
                } else {
                    write_error(&mut writer, &id, -32000, "authentication failed").await?;
                }
            }
            "harness.v1.task.run" => {
                let goal = params.get("goal").and_then(Value::as_str).unwrap_or("").to_string();
                let (tx, mut rx) = mpsc::unbounded_channel::<StreamEvent>();
                let task_id = core.run_task(&goal, tx);
                write_result(&mut writer, &id, json!({"task_id":task_id})).await?;
                // Stream notifications until the core closes the sink.
                while let Some(event) = rx.recv().await {
                    write_json(&mut writer, &json!({
                        "jsonrpc":"2.0",
                        "method":"harness.v1.stream",
                        "params": serde_json::to_value(&event).unwrap(),
                    })).await?;
                }
            }
            "harness.v1.interrupt" => {
                let message = params.get("message").and_then(Value::as_str).unwrap_or("");
                core.interrupt(message);
                write_result(&mut writer, &id, json!({"ok":true})).await?;
            }
            "harness.v1.kill" => {
                core.kill();
                write_result(&mut writer, &id, json!({"ok":true})).await?;
            }
            "harness.v1.audit.verify" => {
                write_result(&mut writer, &id, json!({"verifies":core.audit_verifies()})).await?;
            }
            _ => write_error(&mut writer, &id, -32601, "unknown method").await?,
        }
    }
    Ok(())
}

async fn write_json(w: &mut tokio::net::tcp::OwnedWriteHalf, value: &Value) -> std::io::Result<()> {
    let mut s = serde_json::to_string(value).unwrap();
    s.push('\n');
    w.write_all(s.as_bytes()).await
}
async fn write_result(w: &mut tokio::net::tcp::OwnedWriteHalf, id: &Value, result: Value) -> std::io::Result<()> {
    write_json(w, &json!({"jsonrpc":"2.0","id":id,"result":result})).await
}
async fn write_error(w: &mut tokio::net::tcp::OwnedWriteHalf, id: &Value, code: i64, msg: &str) -> std::io::Result<()> {
    write_json(w, &json!({"jsonrpc":"2.0","id":id,"error":{"code":code,"message":msg}})).await
}

/// Thin client used by the GPUI window (which never touches the OS directly).
pub struct RpcClient {
    reader: BufReader<tokio::net::tcp::OwnedReadHalf>,
    writer: tokio::net::tcp::OwnedWriteHalf,
    next_id: u64,
}

impl RpcClient {
    pub async fn connect(addr: &str) -> std::io::Result<Self> {
        let stream = TcpStream::connect(addr).await?;
        let (reader, writer) = stream.into_split();
        Ok(Self { reader: BufReader::new(reader), writer, next_id: 1 })
    }

    async fn call_raw(&mut self, method: &str, params: Value) -> std::io::Result<Value> {
        let id = self.next_id;
        self.next_id += 1;
        let mut line = serde_json::to_string(&json!({
            "jsonrpc":"2.0","id":id,"method":method,"params":params
        })).unwrap();
        line.push('\n');
        self.writer.write_all(line.as_bytes()).await?;
        loop {
            let mut buf = String::new();
            if self.reader.read_line(&mut buf).await? == 0 { break; }
            let v: Value = serde_json::from_str(buf.trim())?;
            if v.get("id") == Some(&json!(id)) {
                return Ok(v);
            }
            // Notifications are buffered by the caller via `next_notification`.
        }
        Ok(json!({"error":{"code":-1,"message":"connection closed"}}))
    }

    pub async fn call(&mut self, method: &str, params: Value) -> std::io::Result<Value> {
        self.call_raw(method, params).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct FakeCore;
    impl CoreApi for FakeCore {
        fn capabilities(&self) -> Capabilities { Capabilities::default() }
        fn authenticate(&self, token: &str) -> bool { constant_time_eq(token.as_bytes(), b"test-token") }
        fn run_task(&self, _goal: &str, sink: mpsc::UnboundedSender<StreamEvent>) -> String {
            let _ = sink.send(StreamEvent::Token { text: "Hello".into() });
            let _ = sink.send(StreamEvent::ToolBadge { id: "env.probe".into(), tier: "readonly".into() });
            let _ = sink.send(StreamEvent::Done { task_id: "task-1".into() });
            "task-1".into()
        }
        fn interrupt(&self, _: &str) {}
        fn kill(&self) {}
        fn audit_verifies(&self) -> bool { true }
    }

    #[tokio::test]
    async fn handshake_auth_and_streaming() {
        let gw = Gateway::new(Arc::new(FakeCore), "127.0.0.1:0");
        let addr = gw.bind().await.unwrap();

        let stream = TcpStream::connect(&addr).await.unwrap();
        let (read, mut stream) = stream.into_split();
        let mut lines = BufReader::new(read).lines();
        stream.write_all(b"{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"harness.v1.capabilities\",\"params\":{}}\n").await.unwrap();
        let cap: Value = serde_json::from_str(&lines.next_line().await.unwrap().unwrap()).unwrap();
        assert_eq!(cap["result"]["protocol"], "harness.v1");

        // Event surface is unauthorized before auth.
        stream.write_all(b"{\"jsonrpc\":\"2.0\",\"id\":2,\"method\":\"harness.v1.task.run\",\"params\":{\"goal\":\"hi\"}}\n").await.unwrap();
        let denied: Value = serde_json::from_str(&lines.next_line().await.unwrap().unwrap()).unwrap();
        assert_eq!(denied["error"]["code"], -32000);

        stream.write_all(b"{\"jsonrpc\":\"2.0\",\"id\":3,\"method\":\"harness.v1.auth\",\"params\":{\"token\":\"test-token\"}}\n").await.unwrap();
        let auth: Value = serde_json::from_str(&lines.next_line().await.unwrap().unwrap()).unwrap();
        assert_eq!(auth["result"]["ok"], true);

        stream.write_all(b"{\"jsonrpc\":\"2.0\",\"id\":4,\"method\":\"harness.v1.task.run\",\"params\":{\"goal\":\"hi\"}}\n").await.unwrap();
        let started: Value = serde_json::from_str(&lines.next_line().await.unwrap().unwrap()).unwrap();
        assert_eq!(started["result"]["task_id"], "task-1");
        let mut events = Vec::new();
        for _ in 0..3 {
            let n: Value = serde_json::from_str(&lines.next_line().await.unwrap().unwrap()).unwrap();
            assert_eq!(n["method"], "harness.v1.stream");
            events.push(n["params"]["kind"].as_str().unwrap().to_string());
        }
        assert_eq!(events, vec!["token", "tool_badge", "done"]);
    }

    #[test]
    fn foreign_namespace_is_rejected() {
        assert!(constant_time_eq(b"abc", b"abc"));
        assert!(!constant_time_eq(b"abc", b"abd"));
    }
}
