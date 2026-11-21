# Rust and GPUI Coding Agent Architecture

See [Architecture decisions](adr.md), [Development phases](phases.md), and [Implementation steps](implementation-steps.md).

## 1. What We're Building

Build a native desktop coding agent inspired by [Pi Coding Agent](https://github.com/earendil-works/pi), with an entirely Rust-based architecture.

The application should combine Pi's simplicity, the reliability of a production desktop application, and extensibility for future capabilities.

**Requirements:** No LLM SDKs, no agent frameworks, direct HTTP integrations, GPUI for the desktop interface, and Rusqlite for persistence.

### Core architectural decisions

| Layer | Technology |
|---|---|
| Language | Rust |
| Desktop UI | GPUI |
| UI components | GPUI Kit, selectively |
| Async runtime | Tokio |
| LLM integration | Reqwest + direct HTTPS |
| Agent harness | Custom Rust implementation |
| Persistence | SQLite via Rusqlite |
| Credentials | OS-native keychain |
| Tool execution | Sandboxed subprocesses |
| Code search | ripgrep |
| Code intelligence | Tree-sitter, later LSP |
| Event transport | Typed local IPC |
| Extensions | Skills first, MCP later |
| Distribution | Native signed installers |

**Key architectural rule:** The coding agent must be a standalone Rust engine. GPUI only provides the interface. This allows the same engine to power the desktop application, CLI, automation, and eventually remote development environments without rewriting the agent.

## 2. Research References: Pi, Zed, Codex, and Goose

### Reference implementations

1. **[Pi](https://github.com/earendil-works/pi)** — Primary reference for the minimal agent loop, provider abstraction, tools, steering, session trees, and context compaction. The original `badlogic/pi-mono` repository now redirects to `earendil-works/pi`.
2. **[Zed](https://github.com/zed-industries/zed)** — GPUI architecture, rendering, application state, editor and terminal integration, and native desktop performance.
3. **[OpenAI Codex](https://github.com/openai/codex)** — Separation of Rust business logic and interfaces, sandboxing, permissions, and safe command execution.
4. **[Goose](https://github.com/aaif-goose/goose)** — Rust agent with desktop, CLI, API, MCP, and multiple model providers; useful architectural reference but broader than the initial target.

### Pi ideas to adopt

1. **Minimal agent loop:** LLM generates text or tool calls; tools execute; results return to the model.
2. **Provider independence:** Agent loop is not tied to one API.
3. **Small tool surface:** File operations and shell execution provide most basic coding capability.
4. **Tree-structured sessions:** Conversations branch from earlier points.
5. **Context compaction:** Long conversations continue without resending all history.
6. **Steering:** Users can add instructions while the agent is working.
7. **Extensibility:** Add capabilities without complicating the core.

Pi's current implementation does not provide built-in filesystem, process, network, or credential permission isolation. A production desktop agent should **not** inherit that default.

### What not to copy

Avoid building all of Pi's features at once or treating the desktop application as a terminal wrapper. The initial product should not require LangChain, LangGraph, CrewAI, cloud servers, an HTTP backend, external databases, a plugin marketplace, multi-agent orchestration, or a complete IDE.

**Priority:** A reliable execution loop, correctly implemented model protocols, dependable file tools, and recoverable state.

## 3. High-Level Architecture

```text
                    +-----------------------------------+
                    |       GPUI Desktop App            |
                    | Chat, streaming, diffs, terminal  |
                    | sessions, settings, approvals     |
                    +----------------+------------------+
                                     |
                        Typed local IPC / events
                                     |
                    +----------------v------------------+
                    |         Rust Agent Service        |
                    |-----------------------------------|
                    | Agent Loop       Provider Registry|
                    | Tool Registry    Context Builder  |
                    | Session Manager  Permission Engine|
                    | Event Dispatcher Workspace Manager|
                    +--------+------------+-------------+
                             |            |
          +------------------+            +-------------------+
          |                   |                              |
    +-----v-------+    +------v-------+            +---------v--------+
    | LLM APIs    |    | SQLite       |            | Tool Runner      |
    | Direct HTTPS|    | Sessions     |            | Files, shell, Git|
    | Streaming   |    | Events       |            | Sandbox          |
    +-------------+    +--------------+            +------------------+
```

### Component responsibilities

- **GPUI application:** Renders state and sends user commands. No provider-specific model logic or direct execution of model-requested shell commands.
- **Agent service:** Owns sessions, requests, context, tools, authorization, persistence, and task lifecycle.
- **Tool runner:** Executes approved operations with appropriate OS restrictions; separate process where isolation is supported.

### Why a separate service?

A headless service allows the GUI to restart without necessarily killing an active agent session. A CLI can reuse the engine. Long-running commands need not be owned by a GPUI window. Multiple windows can subscribe to the same session.

Use **Unix domain sockets on macOS/Linux** and **named pipes on Windows**, with restricted per-user permissions and a versioned IPC protocol. Do not expose an unauthenticated localhost HTTP server.

Start with crates in one process if it simplifies early prototyping, but define the IPC command/event boundary immediately so extracting the service is straightforward.

## 4. Rust Workspace and Crate Architecture

```text
rust-coding-agent/
|
+-- Cargo.toml
+-- Cargo.lock
+-- rust-toolchain.toml
|
+-- apps/
|   +-- desktop/            # GPUI desktop application
|   +-- agentd/             # Headless agent service
|   +-- cli/                # Headless CLI
|   +-- tool-runner/        # Restricted command executor
|
+-- crates/
|   +-- agent-core/         # Agent loop and state machine
|   +-- llm/                # Provider-independent types
|   +-- llm-providers/      # Direct REST integrations
|   +-- tools/              # Tool definitions and dispatch
|   +-- execution/          # Shell, subprocess, sandboxing
|   +-- context/            # Prompts, history, compaction
|   +-- sessions/           # Sessions, branches, event journal
|   +-- storage/            # Rusqlite and migrations
|   +-- workspace/          # Filesystem, search, Git
|   +-- security/           # Trust, permissions, secrets
|   +-- protocol/           # Typed IPC commands/events
|   +-- desktop-ui/         # GPUI views and state
|
+-- migrations/
+-- tests/
|   +-- fixtures/
|   +-- provider-contracts/
|   +-- agent-scenarios/
|   +-- sandbox-tests/
|   +-- crash-recovery/
|
+-- packaging/
    +-- windows/
    +-- macos/
    +-- linux/
```

This is a **target layout**, not a requirement to create every crate immediately. Start with agent core, LLM abstraction, tools, persistence, protocol, and desktop UI. Extract other crates once their roles justify it.

### Dependency direction

```text
desktop-ui -------> protocol
agentd ----------> agent-core, llm-providers, storage
agent-core ------> llm, tools, context, sessions
llm-providers ---> llm
tools -----------> execution, workspace, security
storage ---------> sessions
```

`sessions` defines the `SessionStore` interface. The Phase 1 CLI and harness tests inject an in-memory implementation. In production, `agentd` supplies the SQLite implementation from `storage` and the HTTPS adapter from `llm-providers`. The core can run a complete turn without linking GPUI, Rusqlite, or Reqwest.

## 5. The Custom Agent Harness

The harness orchestrates the agent and must be relatively small but thoroughly tested.

### 5.1 Agent loop

```text
User submits task
       |
Persist user message
       |
Build model context: instructions + history + tools
       |
Send LLM request; stream and normalize response
       |
Inspect completed response (text, tool calls, finish reason)
       |
  +----+------------------------------+
  |                                   |
Finished?                          Tool calls?
  |                                   |
Complete run                        Validate calls
                                      |
                               Authorize operations
                                      |
                               Execute approved tools
                                      |
                               Persist tool results
                                      |
                               Return results to LLM
                                      |
                               Loop until completed
```

See [Pi's agent loop](https://github.com/earendil-works/pi/blob/main/packages/agent/src/agent-loop.ts).

### 5.2 Explicit state machine

Avoid one huge async function with deeply nested conditionals.

```rust
enum RunState {
    Idle,
    PreparingContext,
    WaitingForModel,
    StreamingResponse,
    ValidatingTools,
    AwaitingApproval,
    ExecutingTools,
    RecordingResults,
    CompactingContext,
    Completed,
    Cancelled,
    Failed,
    Interrupted,
}
```

Transitions should be explicit and validated. Incomplete streamed tool JSON may not enter `ExecutingTools`. Pending approval must survive GUI restart without becoming an authorization.

### 5.3 Agent runtime object

```text
AgentRun
+-- run_id
+-- session_id
+-- workspace_id
+-- model_configuration
+-- context_builder
+-- provider
+-- tool_registry
+-- permission_policy
+-- cancellation_token
+-- event_sink
+-- runtime_limits
```

Each workspace owns a session controller for runs, steering, cancellation, and tool coordination.

The service assigns one workspace ID per resolved directory identity. Clients opening the same directory through path aliases share its ID and lock. Before the first operation with workspace write access, the run must acquire that workspace's mutation lock. It holds the lock through run termination and runner cleanup. A second run may continue read-only work while waiting. On service restart, reconcile surviving runners before granting the lock again.

### 5.4 Tool scheduling

| Category | Examples | Scheduling |
|---|---|---|
| Read-only | Read, search, list, commands confined to read-only workspace access | Parallel when independent |
| Can write the workspace | Write, edit, Git mutation, shell with write access (including builds and tests) | Authorize, acquire the workspace mutation lock, execute sequentially |
| Other external effects | Network access | Require the applicable grant in addition to filesystem permissions |

Do not blindly execute all model-issued tool calls in parallel; maintain dependency order.

### 5.5 Steering, follow-ups, and cancellation

Support two queues:

- **Steering:** Change the current task at the next safe execution boundary.
- **Follow-up:** Process new instructions after the current run ends.

Propagate cancellation through model HTTP requests, tools, and subprocess trees where supported. Use Tokio cancellation primitives such as `tokio_util::sync::CancellationToken`. Record cancelled shell commands as interrupted, not successful.

### 5.6 Runtime limits

Configure limits for model turns, number of tool calls, accumulated output bytes, wall-clock time, and optionally estimated spending. Preserve state and make a limit-reached run resumable.

## 6. Multi-Provider LLM Architecture — No SDKs

Use Reqwest with TLS, API-dialect serializers, streaming parsers, and a common internal representation. OpenRouter is the first-release model gateway. The Rust client calls OpenRouter directly over HTTPS without an LLM SDK.

### 6.1 Provider-independent interface

Conceptual interface (not a compile-ready implementation):

```rust
trait ModelProvider: Send + Sync {
    fn capabilities(&self, model: &ModelId) -> ModelCapabilities;

    fn stream(
        &self,
        request: ModelRequest,
        cancellation: CancellationToken,
    ) -> ModelEventStream;
}
```

The agent core consumes normalized `ModelEvent` values only.

### 6.2 Adapter architecture

```text
Agent Core -> Model Registry -> Provider Interface
     -> OpenRouter adapter
     -> Chat Completions codec -> Reqwest + TLS -> openrouter.ai
```

Use one adapter and streaming codec across the supported OpenRouter models. Keep the endpoint and credential reference in provider configuration and model capabilities in the registry. Add API dialects independently as needed.

### 6.3 OpenRouter endpoints and first-release scope

| Operation | HTTPS endpoint |
|---|---|
| Streaming inference | `POST https://openrouter.ai/api/v1/chat/completions` |
| Model catalog | `GET https://openrouter.ai/api/v1/models` |

Send the OpenRouter catalog model ID as `model` and set `stream: true`. Check that the catalog entry includes `tools` in `supported_parameters` before enabling coding tools, and verify tool-call behavior on supported models. [Chat Completions API](https://openrouter.ai/docs/api/api-reference/chat/send-chat-completion-request) · [Model catalog API](https://openrouter.ai/docs/api/api-reference/models/list-all-models-and-their-properties)

### 6.4 Normalized messages

```text
Message
+-- role
|   +-- system
|   +-- developer
|   +-- user
|   +-- assistant
|   +-- tool
+-- content blocks
|   +-- text
|   +-- image
|   +-- reasoning
|   +-- tool_call
|   +-- tool_result
+-- metadata
    +-- provider
    +-- model
    +-- request_id
    +-- usage
    +-- provider_opaque_data
```

Retain provider-specific continuation information such as reasoning items, signatures, and identifiers. Normalization must not discard data that a provider requires for follow-up requests.

### 6.5 Streaming

Separate **request encoding** (normalized messages and schemas to native HTTP requests) from **response decoding** (native streaming events to normalized events).

```rust
enum ModelEvent {
    MessageStarted,
    TextDelta(String),
    ReasoningDelta(String),
    ToolCallStarted { id: String, name: String },
    ToolArgumentsDelta { id: String, delta: String },
    ToolCallCompleted { id: String },
    UsageUpdated(TokenUsage),
    MessageCompleted { finish_reason: FinishReason },
    MessageFailed { error: ModelError },
}
```

```text
HTTP response bytes
        |
Streaming frame decoder
        |
Provider-specific parser
        |
Partial response accumulator
        |
Normalized ModelEvent
        |
Agent event dispatcher
        |
GPUI renderer
```

`ToolCallCompleted` marks assembled arguments. The adapter emits one terminal event for the turn after checking its finish reason and stream termination. An explicit provider error, premature EOF, missing finish reason, or inconsistent response produces `MessageFailed`, even after valid tool arguments and an HTTP success status. [OpenRouter streaming documentation](https://openrouter.ai/docs/api/reference/streaming)

Only a clean `tool_calls` finish with a complete, schema-valid batch can advance to authorization. A clean `stop` with no calls finishes the answer. Surface `length` as truncated and `content_filter` as filtered. Failure, cancellation, and unsupported finish reasons discard pending dispatches. Retain the reason for the UI and recovery. [OpenRouter API overview](https://openrouter.ai/docs/api/reference/overview)

### 6.6 Model registry

Store provider/model identifiers, supported modalities, tool support, reasoning support, context/output limits, native API dialect, optional pricing metadata, and provider-specific parameters. Keep model metadata versionable and user-overridable rather than baking prices and limits into compiled Rust code.

### 6.7 Networking reliability

Implement connection pooling, TLS verification, timeouts, retry classification, exponential backoff with jitter, `Retry-After`, proxies, cancellation, interrupted-stream detection, bounded buffers, request IDs, and structured errors.

Never automatically retry a completed or uncertain side-effecting tool operation because an LLM request failed.

### 6.8 Authentication

Let users enter an OpenRouter API key and send it as `Authorization: Bearer <api-key>` on inference requests. Keep the key in Windows Credential Manager, macOS Keychain, or a supported Linux secret service, and redact authorization headers from logs and diagnostics. [OpenRouter API overview](https://openrouter.ai/docs/api/reference/overview)

## 7. Tool System and Filesystem Operations

### 7.1 Initial tool set

| Tool | Responsibility |
|---|---|
| `read_file` | Read files with line numbers and output bounds |
| `write_file` | Create or replace a file safely |
| `edit_file` | Apply precise changes |
| `list_files` | List directories or paths |
| `search` | Search via ripgrep |
| `execute_command` | Execute an approved command |
| `git_status` | Inspect repository status |
| `git_diff` | Show changes |

Prefer one search tool with filename/content modes over several redundant tools.

### 7.2 Tool contracts

```text
ToolDefinition
+-- name
+-- description
+-- JSON input schema
+-- capability requirements

ToolExecution
+-- validated arguments
+-- workspace context
+-- cancellation
+-- output limits

ToolOutcome
+-- status
+-- structured result
+-- display result
+-- side-effect metadata

ToolPolicy
+-- read/write scope
+-- approval requirement
+-- execution environment
```

Keep tool execution and display distinct so GPUI does not implement tool internals.

### 7.3 Safe editing sequence

1. Resolve and authorize the target under the workspace policy. Bind the operation to that directory and target through commit, including when links or junctions change.
2. For an edit or replacement, verify the expected file identity and content against the observed base. For creation, require an absent destination. Build the proposed bytes and per-tool diff in memory, preserving BOM, line endings and applicable metadata.
3. Publish a new file with an atomic create-if-absent operation. A competing creation returns an edit conflict and preserves the winner's bytes.
4. For an existing file, the commit mechanism must protect the expected identity and bytes through replacement against independent writers, including editors that save by rename. A hash recheck followed by an unconditional rename leaves a race. Qualify the mechanism per platform and filesystem. If the guarantee is unavailable, return the reviewable patch as unapplied. Report an observed mismatch as a conflict.
5. Record the outcome and base-to-result diff for this invocation. Label previews as unapplied. The separate Git workspace view includes pre-existing staged, unstaged and untracked changes. Only attribute a diff to a file tool after its write commits.

### 7.4 Search and code intelligence

Start with `ripgrep`, the `ignore` crate, Tree-sitter (when syntax awareness helps), and Git status/diff. **Do not build a vector database or full repository index for v1.** Add semantic retrieval only after benchmarks demonstrate value.

### 7.5 Shell execution

```text
Agent requests command
       |
Tool validation
       |
Permission decision
       |
Execution environment selection
       |
Spawn isolated process
       |
Stream stdout/stderr
       |
Enforce limits/cancellation
       |
Persist exit code and result
```

Use `tokio::process` for noninteractive commands and a cross-platform PTY abstraction for interactive terminals. Windows uses ConPTY; Unix systems use appropriate PTY mechanisms.

A PTY **is not** a sandbox. Handle environment, current working directory, process-group cleanup, exit codes, signals, timeouts, output truncation, and child processes.

## 8. Security and Sandboxing

**Permission prompts do not equal process isolation.** A command executed with full user privileges may access data outside the chosen workspace regardless of checks in your file-edit tool.

### 8.1 Security pipeline

```text
Tool call
   |
Schema validation
   |
Policy engine
   |
Approval (if required)
   |
Restricted execution environment
   |
Tool result + audit record
```

### 8.2 Default permissions

| Operation | Default |
|---|---|
| Read ordinary workspace file | Allow |
| Read protected secrets or outside workspace | Deny or explicitly approve |
| Search workspace | Allow within the authorized read scope |
| Write workspace file | Scoped user authorization |
| Delete/overwrite important files | Explicit approval |
| Run shell | Approval or narrowly defined sandbox policy |
| Command network access | Restricted by default |
| Write outside workspace | Deny unless separately authorized |

Approvals may be once, session, or workspace scoped. Never broaden an approval implicitly.

Apply the same read policy to file tools, search, Git output and shell commands. Scope inputs before inspection, including Git index and history content for protected paths. Give subprocesses a filesystem view that enforces the policy, including restrictions on access to the underlying Git object store where required. If a backend cannot enforce the requested scope, deny the operation. Apply these rules before sending output to the model, UI, journal or artifacts.

Issue a stable approval ID bound to the tool call, run, workspace, validated arguments and requested capabilities. The service atomically resolves a pending decision once. Only an approved, current call may be claimed for dispatch, and only one worker may claim it. Duplicate replies return the recorded state. Reject stale replies after cancellation or a change of operation. A changed operation requires fresh authorization.

### 8.3 OS isolation

| OS | Candidate mechanisms |
|---|---|
| Linux | Bubblewrap, namespaces, Landlock when applicable |
| macOS | Seatbelt profiles or isolated container execution |
| Windows | AppContainer/native isolation or isolated WSL2/container environment |

The [Codex Rust source](https://github.com/openai/codex/tree/main/codex-rs) is a useful reference for sandbox-policy implementation.

A Windows Job Object can manage resource limits and child-process termination, but is **not** a security sandbox by itself.

Prove Windows sandbox feasibility early. If required isolation cannot be enforced, fail closed and make that clear to the user.

### 8.4 Threat model

Test malicious repository instructions, prompt injection, API-key exfiltration, symlink/junction traversal, destructive commands, malicious install scripts, untrusted MCP responses, ANSI terminal attacks, unbounded tool output, subprocess environment leakage, and approval/execution races.

Treat model output and repository contents as untrusted data. Opening a repository must not automatically execute scripts or extensions.

## 9. SQLite Persistence and Durable Sessions

Adopt Pi-style tree-structured sessions while storing metadata and events in SQLite instead of JSONL. See [Pi session format](https://docs.pi.dev/latest/session-format).

### 9.1 Core tables

| Table | Responsibility |
|---|---|
| `projects` | Workspace roots and trust |
| `sessions` | Conversation identities |
| `branches` | Active history pointers |
| `events` | Immutable history |
| `runs` | Agent task lifecycle |
| `tool_calls` | Execution and approval state |
| `compactions` | Context checkpoints |
| `permissions` | Scoped grants and denials |
| `usage` | Token and cost accounting |
| `artifacts` | Diffs and output references |

Minimal sketch:

```sql
CREATE TABLE events (
    id TEXT PRIMARY KEY,
    session_id TEXT NOT NULL,
    parent_event_id TEXT,
    sequence_number INTEGER NOT NULL,
    kind TEXT NOT NULL,
    payload_json TEXT NOT NULL,
    created_at TEXT NOT NULL,
    UNIQUE (session_id, sequence_number)
);
```

Production migrations should add foreign keys, indexes, constraints, and payload versioning.

### 9.2 Event-sourced sessions

```text
User message
     |
Assistant response
     |
Tool call
     |
Tool result
     |
Assistant response
     |
     +----- Branch A
     |
     +----- Branch B
```

Immutable events allow session branching, auditing, replay, crash recovery, tool inspection, and durable links between model requests and tool results. A branch points to its head event; reconstruct history by following parent pointers. Use materialized projections/snapshots for fast UI rendering.

**Important:** Conversation branching does not undo filesystem changes. Repository snapshots and conversation history are different systems.

### 9.3 Rusqlite concurrency

Use a dedicated storage worker/actor with a bounded command channel. Enable WAL mode, foreign keys, busy timeout, versioned migrations, coordinated writes, transactions, and separate read access where appropriate. Select durability/synchronous settings appropriate for the importance of individual writes.

Do not hold SQLite transactions open during model requests or subprocess execution.

### 9.4 Recoverability

Persist the pending call and approval binding before dispatch. Store the decision and its audit event in one transaction, then durably claim the authorized call before launching it. Record the observed start and eventual outcome. On restart, restore pending decisions and treat a claimed call without a recorded outcome as `OutcomeUnknown` until reconciled. A crash between claim and launch can also produce this state. Never automatically rerun uncertain side effects.

Store large logs and binary files as content-addressed artifacts with retention and cleanup policies. Keep secrets in the OS keychain.

## 10. Context Engineering and Compaction

### 10.1 Context assembly

```text
System instructions
       +
Project instructions
       +
Tool definitions
       +
Relevant active session history
       +
Compaction summaries
       +
Current user task
       |
Token budget + validation
       |
Provider adapter
```

Support `AGENTS.md`-style project instructions with explicit precedence and workspace trust. Avoid putting entire repositories, all past tool output, or all previous files into context.

### 10.2 Budgets

Reserve context space separately for system/project instructions, tool schemas, conversation, relevant files, expected output, and safety headroom. Use real provider usage when available and conservative estimation otherwise.

### 10.3 Compaction

1. Keep immutable original history.
2. Identify a safe boundary.
3. Summarize finished work and remaining tasks.
4. Record changed/relevant files.
5. Preserve unresolved errors, user constraints, and decisions.
6. Save a compaction event.
7. Continue with the summary and recent messages.

Compaction is **not deletion**. Make the summary inspectable and history recoverable.

### 10.4 Retrieval scope

Start with targeted source reads, search, and explicit working files. Introduce embeddings/symbol retrieval after measurement demonstrates a real improvement.

## 11. GPUI Desktop Architecture

The app should be a focused agent workstation, not a complete VS Code replacement.

### 11.1 Conceptual layout

```text
+-------------------------------------------------------------------+
| Coding Agent                                        Search Settings|
+----------------+----------------------------------+---------------+
| WORKSPACE      | AGENT CONVERSATION               | CHANGES       |
|                |                                  |               |
| my-project     | User: Refactor auth handler      | auth.rs       |
|                |                                  | auth_test.rs  |
| SESSIONS       | Read src/auth.rs                 |               |
| Auth refactor  | Edited 2 files                   | RUNTIME       |
| Fix payment    | Running cargo test...            | Model         |
| Write tests    |                                  | Tools used    |
|                | [ Ask the agent...             ] | Status        |
+----------------+----------------------------------+---------------+
```

### 11.2 UI layout modules

```text
desktop-ui/
+-- app/
|   +-- app_state.rs
|   +-- windows.rs
|   +-- commands.rs
+-- views/
|   +-- workspace_view.rs
|   +-- session_sidebar.rs
|   +-- conversation_view.rs
|   +-- message_view.rs
|   +-- tool_call_view.rs
|   +-- diff_view.rs
|   +-- terminal_view.rs
|   +-- settings_view.rs
+-- components/
|   +-- composer.rs
|   +-- markdown.rs
|   +-- code_block.rs
|   +-- approval_dialog.rs
+-- state/
    +-- session_state.rs
    +-- run_state.rs
    +-- workspace_state.rs
```

### 11.3 GPUI state and performance

Use GPUI entities for retained state and views for rendering. Incoming service events update entities and notify GPUI. Model HTTP and tools never block the UI thread.

Coalesce high-frequency token updates, virtualize long conversations, lazy-render historical code blocks, and bound terminal output buffers.

[GPUI](https://github.com/zed-industries/zed/tree/main/crates/gpui) is still pre-1.0; pin a known-compatible revision and commit `Cargo.lock`. Consider [GPUI Kit](https://github.com/longbridge/gpui-kit) for standard controls, but verify exact version compatibility before integrating.

### 11.4 First-release interaction requirements

- Workspace selection and recent sessions
- Streaming Markdown responses
- Expandable tool calls and command output
- Approval/rejection dialogs
- Stop, retry, continue, steering
- Provider/model selection
- File change preview and diffs
- Session persistence/restoration
- Shortcuts and command search
- Clear errors and recovery status

A full IDE, LSP client, debugger, and terminal multiplexer can come later.

## 12. Extensibility

Support three levels:

1. **Skills and templates:** Markdown instructions and reusable workflows; no executable plugins. Build first.
2. **MCP tools:** External tool servers behind capability checks. Add after built-in tools are stable; see the [Rust MCP SDK](https://github.com/modelcontextprotocol/rust-sdk) as a reference.
3. **Executable plugins:** Isolated processes or sandboxed WASM with versioned APIs and explicit capabilities. Build only after core maturity.

Plugin APIs may expose tool registration, model registration, lifecycle events, commands, and constrained UI contributions. Do not give plugins raw database connections or unrestricted internal Rust objects. Avoid in-process native libraries for untrusted extensions.

## 13. Reliability, Observability, and Testing

### 13.1 Event journal / live stream

```text
RunStarted
ContextPrepared
ModelRequestStarted
AssistantMessageCommitted
ToolCallProposed
ApprovalRequested
ApprovalResolved
ToolExecutionStarted
ToolExecutionFinished
UsageRecorded
RunCompleted
RunCancelled
RunInterrupted
RunFailed
```

Assign each durable event a strictly increasing per-session sequence number. Include run, turn and message IDs where applicable. `AssistantMessageCommitted` holds the normalized message, tool calls, continuation data and finish reason. Commit it and advance the branch head before dispatching tools. For a final answer, commit the message, branch head and `RunCompleted` in one transaction. Publish durable events only after commit.

During generation, stream transient deltas tagged with the message ID and an offset. On reconnect, the client supplies its last applied durable sequence number. The service establishes a subscription at a consistent boundary, captures the durable high-water mark and any in-progress message snapshot with its next offset, and buffers subsequent updates. Replay durable events through the mark, replace the partial preview with the snapshot, then drain the buffer in order. Deduplicate durable events by sequence and deltas by offset. The committed message replaces the preview by message ID.

If the service restarts before a message commits, recover the interrupted turn from the journal and clear its transient preview. A completed run must always replay its final answer.

### 13.2 Test matrix

| Test category | Proves |
|---|---|
| Provider contracts | Correct streaming and tool-call parsing |
| Agent loop | Correct ordering, cancellation, stopping |
| Tool tests | Safe reads/writes and conflict detection |
| Security | Restricted actions are actually blocked |
| Persistence | Crash recovery and consistent histories |
| Integration | Real tasks in temporary Git repositories |
| UI | Streaming, approvals, focus, large histories |
| Performance | No stalls or unbounded memory growth |

Use sanitized recorded provider fixtures, test malformed JSON/truncated streams/duplicate events/rate limits, and use deterministic fake providers to test the loop.

### 13.3 Coding evaluation suite

- Find and explain a bug.
- Edit a function without unrelated changes.
- Refactor across files.
- Fix a failing test.
- Handle a concurrent edit.
- Recover from a failed command.
- Recover from an interrupted stream.
- Resume after forced restart.

Measure task success, test-pass rate, unnecessary changes, interventions, tool-call count, token usage, and cost.

### 13.4 Privacy-conscious diagnostics

Use `tracing` for structured logs of IDs, state transitions, timings, failure categories, and resources. Avoid storing raw source code, full prompts, secrets, and large outputs in logs by default. Provide explicit diagnostics export with redaction.

## 15. Rust Dependencies and Packaging

| Responsibility | Candidate dependency |
|---|---|
| Async execution | `tokio` |
| HTTP | `reqwest` |
| TLS | `rustls` via HTTP stack |
| Serialization | `serde`, `serde_json` |
| Error handling | `thiserror`, `anyhow` at app boundaries |
| SQLite | `rusqlite` |
| IDs | `uuid` |
| Cancellation | `tokio-util` |
| Logging | `tracing`, `tracing-subscriber` |
| Credentials | `keyring` plus platform validation |
| Search | ripgrep, `ignore` |
| Pattern matching | `regex` |
| Code parsing | `tree-sitter` |
| File watch | `notify` |
| Diff | `similar` or Git-aware implementation |
| Git | Git CLI initially; `gix` selectively |
| Interactive terminal | Cross-platform PTY abstraction |
| UI components | GPUI Kit |

Pin tested versions and `Cargo.lock`, especially where GPUI compatibility is sensitive.

### Packaging

Use native Windows MSI/MSIX or a suitable installer, signed/notarized macOS application bundles, and appropriate Linux packages.

GPUI does not automatically include Tauri's updater. Build or integrate a separately maintained updater with cryptographically verified release metadata.

CI should test Windows/macOS/Linux build targets, formatting, lint, advisories, licenses, integration tests, and release artifacts.

For each advertised platform, attach pass/fail evidence for isolation, safe editing on supported filesystems, packaging and recovery to the release record. Qualifying an editable filesystem requires both a successful edit of an existing file and preservation of a competing save. A failed or unverified required check blocks qualification of that platform.

## 17. Recommended Source Code and Documentation

| Source | Study |
|---|---|
| [Pi agent-loop.ts](https://github.com/earendil-works/pi/blob/main/packages/agent/src/agent-loop.ts) | Turn scheduling and tool lifecycle |
| [Pi agent.ts](https://github.com/earendil-works/pi/blob/main/packages/agent/src/agent.ts) | Stateful runtime |
| [Pi agent-session.ts](https://github.com/earendil-works/pi/blob/main/packages/coding-agent/src/core/agent-session.ts) | Session orchestration and compaction |
| [Pi session format](https://docs.pi.dev/latest/session-format) | Branchable event history |
| [Pi built-in tools](https://github.com/earendil-works/pi/tree/main/packages/coding-agent/src/core/tools) | Minimal coding-tool implementation |
| [GPUI README](https://github.com/zed-industries/zed/blob/main/crates/gpui/README.md) | Entities, rendering, framework model |
| [GPUI Kit](https://github.com/longbridge/gpui-kit) | Native UI components |
| [Codex Rust](https://github.com/openai/codex/tree/main/codex-rs) | Agent engineering and sandboxing |
| [Goose](https://github.com/aaif-goose/goose) | Rust agent ecosystem |
| [Rust MCP SDK](https://github.com/modelcontextprotocol/rust-sdk) | Future MCP integration |
| [OpenRouter Chat Completions](https://openrouter.ai/docs/api/api-reference/chat/send-chat-completion-request) | HTTPS inference, bearer authentication, and streaming |
| [OpenRouter model catalog](https://openrouter.ai/docs/api/api-reference/models/list-all-models-and-their-properties) | Model IDs and supported parameters |

## 18. Final Recommended Interfaces

Keep the architecture focused on five clean interfaces:

1. **Agent Runtime:** Loop, state machine, scheduling, cancellation.
2. **Model Provider:** Native API adapters into normalized events.
3. **Tool Executor:** Validate, authorize, execute, and audit coding tools.
4. **Session Store:** Persist events, branches, runs, and recovery state.
5. **Client Protocol:** Commands and events for GPUI, CLI, and future interfaces.

If these boundaries remain clean, the UI, providers, sandbox, or MCP support can evolve without rewriting the harness.

**First significant milestone:** Build a headless Rust coding agent that can read a repository, apply a safe edit, execute tests within an enforceable permission policy, persist the entire run in SQLite, and recover correctly after a crash.

Once that works, GPUI becomes the presentation layer over a functioning agent engine—preserving Pi's minimalism while providing the engineering foundation for a production desktop coding agent.
