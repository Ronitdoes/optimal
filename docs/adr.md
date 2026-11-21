# Architecture Decision Records

See [Architecture](architecture.md) for design details and [Development phases](phases.md) for delivery sequencing.

## 16. Architectural Decision Records

| ADR | Decision |
|---|---|
| ADR-001 | Agent engine independent of GPUI |
| ADR-002 | OpenRouter over direct HTTPS with API keys |
| ADR-003 | Typed event protocol between UI and engine |
| ADR-004 | SQLite immutable session events |
| ADR-005 | Centralized permissions for every tool |
| ADR-006 | Sandbox tool runner outside main process |
| ADR-007 | One mutating run per workspace initially |
| ADR-008 | Conditional file updates and per-tool diffs |
| ADR-009 | Context compaction without history deletion |
| ADR-010 | Skills, then MCP, then executable plugins |

## Decision rationale and implications

### ADR-001 — Keep the agent engine independent of GPUI

The desktop and CLI share a standalone Rust engine. The core consumes the `SessionStore` interface from `sessions`. The Phase 1 harness uses an in-memory store, and the production service injects the SQLite adapter. Core tests can run without GPUI, Rusqlite or Reqwest.

### ADR-002 — Integrate OpenRouter over direct HTTPS

Use one Chat Completions adapter for OpenRouter models. Store the OpenRouter API key in the OS keychain. The adapter owns request encoding and stream decoding, and the runtime continues to consume normalized events. Additional API dialects can be added behind the same interface.

Carry the terminal finish reason or failure into the runtime. Only a clean `tool_calls` finish and a validated batch can advance to authorization. A later stream failure invalidates previously assembled calls from that turn.

### ADR-003 — Define a typed client protocol

Use versioned commands and events so the same engine can serve multiple clients and restore desktop state after reconnecting. Early prototypes may run in-process. The target service uses per-user local IPC.

Clients resume after their last applied per-session sequence. A consistent handoff from durable replay to the live subscription includes the active message preview and its offset.

### ADR-004 — Store immutable session events in SQLite

Store branch heads and parent links so clients can reconstruct history and inspect past runs. Journal tool execution separately from its eventual outcome to support reconciliation after interruption.

Commit the final assistant message together with its branch head and run completion. Token deltas serve live previews. Replay uses the committed message.

### ADR-005 — Centralize tool permissions

Apply the same protected-file policy to direct reads, search, Git and subprocesses before any content reaches a client or model. Deny operations when the runner cannot enforce the scope. Bind each approval ID to the exact call and scope, resolve it atomically, and claim an approved call once before dispatch. Replayed or stale client replies cannot trigger another execution.

### ADR-006 — Isolate command execution in a tool runner

Run approved commands under enforceable platform restrictions. Validate Windows isolation early and fail closed when required guarantees are unavailable.

### ADR-007 — Allow one mutating run per workspace initially

The service identifies the workspace by its resolved directory identity across clients. A run takes the workspace mutation lock before its first write-capable operation, including shell builds and tests with workspace write access. It holds the lock through run termination and runner cleanup. Independent read-only runs can continue.

### ADR-008 — Detect edit conflicts and report Git-aware changes

Create new files only if the destination is still absent at commit. Replace existing files only when the platform and filesystem can protect the expected identity and bytes through commit. Preserve a competing save and report a conflict when detected. Where protected replacement is unavailable, return an unapplied patch for review. Record the committed edit against its observed base. Show the complete workspace status and diff separately so pre-existing user changes retain their attribution.

### ADR-009 — Compact context while retaining history

Store summaries as inspectable checkpoints and preserve original events. This bounds request context while retaining the evidence needed for review and recovery.

### ADR-010 — Introduce extensibility in stages

Start with Markdown skills and templates. Add MCP after built-in tools are stable, then consider executable extensions with isolation, explicit capabilities, and versioned interfaces.

### Non-negotiable scope boundaries

- **No unnecessary cloud backend:** Local-first operation does not require Supabase, Redis, remote queues, or a hosted HTTP backend.
- **No heavyweight AI planning framework:** Add separate planning state only when evaluations show a need.
- **No fake rollback:** Conversation branches do not revert a repository; use Git worktrees or explicit snapshots when necessary.
- **No default unsandboxed autonomy:** Higher autonomy requires stronger boundaries, not weaker safeguards.
- **No full IDE in v1:** Diffs, snippets, and tool outputs provide sufficient initial functionality.
