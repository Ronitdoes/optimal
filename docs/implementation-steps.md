# Implementation Steps

Follow the [development phases](phases.md) for milestone exit criteria. The [architecture](architecture.md) describes the target interfaces and the [ADR log](adr.md) records the design constraints.

## Phase 0 — Prove the foundation

See the [Phase 0 probes](phase-0.md) for build commands and the evidence by platform.

- [x] Create the initial Cargo workspace and pin the Rust toolchain and tested GPUI artifact. Generate the lockfile and defer crate extraction until the boundaries are needed.
- [x] Commit the lockfile.
- [x] Define typed commands and sequenced events for the CLI and desktop. Prove that a Windows GPUI window can consume Tokio events without blocking rendering.
- [x] Build small probes for streaming HTTP and SQLite writes. Exercise candidate tool-runner isolation on Windows, including filesystem and network restrictions and child-process cleanup. Record which platform guarantees were demonstrated and fail closed when required isolation is unavailable.
- [x] Probe create-if-absent and protected replacement on the initial target filesystem. Record the commit mechanism, how it handles in-place writes and save-by-rename, and which targets require the unapplied-patch fallback.

## Phase 1 — Build the headless harness

- [ ] Define normalized messages, model capabilities, and streaming events. Implement the OpenRouter Chat Completions adapter using Reqwest over HTTPS and bearer API-key authentication.
- [ ] Load the OpenRouter model catalog and use model capabilities to enable tools. Test against fixtures for authorization, streaming tool calls, interrupted streams, and access or quota errors.
- [ ] Implement the runtime state machine, context preparation, streaming accumulation, schema validation, cancellation, steering, follow-ups and runtime budgets. Preserve the terminal finish reason or failure before routing a call to authorization.
- [ ] Inject a fake provider and an in-memory `SessionStore` into the minimal CLI. Verify a deterministic multi-turn task without SQLite, GPUI or network access. Keep sanitized fixtures for malformed JSON, duplicate events and rate limits.
- [ ] **Tool dispatch after stream completion.** Send complete, valid tool arguments followed by a provider error, premature EOF or cancellation. Assert zero dispatches for each, and for filtered, length-limited and unknown finishes. A clean `stop` returns text. A clean `tool_calls` finish with one valid, authorized call dispatches exactly once.

## Phase 2 — Execute coding tools safely

- [ ] Implement workspace-scoped file, search and Git operations with consistent path and output limits. Centralize authorization, launch commands through the restricted runner, bound output and propagate cancellation.
- [ ] **Protected-file access.** Put a unique marker in a protected tracked file and its Git history. Exercise read, search, diff and shell under the same grant. Check the model input, client output and persisted results for the marker. An ordinary file remains readable. An unenforceable scope denies the operation.
- [ ] **Concurrent save and create.** Pause an edit just before commit and race an independent writer, including an editor saving by rename. If the writer succeeds first, its bytes survive and the tool cannot report an applied edit. Race a create against a competing create and verify that only one succeeds. Check BOMs, line endings, links and junctions. Record the guarantees for each supported platform/filesystem and verify the unapplied-patch fallback where protected replacement is unavailable.
- [ ] **Workspace mutation lock.** Open the same workspace through two clients using path aliases. Hold the first run open with a write-capable build or test. Confirm the second run can read but cannot mutate until the first run ends and its runner is cleaned up, including on cancellation.
- [ ] **Change attribution.** Seed a temporary Git repository with staged, unstaged and untracked changes, including an unrelated edit in the file the agent will touch. Fix a failing test and run it in isolation. Compare the recorded tool diff with the agent's edit, and verify the Git view and index still account for all original user changes.

## Phase 3 — Preserve and recover runs

- [ ] Implement the SQLite `SessionStore` with versioned migrations and a bounded storage worker. Persist branch heads, immutable events with unique per-session sequences, runs, scoped permissions, usage and artifact references.
- [ ] **Approval resolution.** Persist each pending approval and its bound operation. Race two approvals, race approval against denial, and submit a stale reply after cancellation or an argument change. Verify one recorded decision per resolved approval, zero launches for denied or invalidated calls, and at most one launch for an approved call.
- [ ] Store the OpenRouter API key in the OS keychain and redact authorization headers from diagnostics. Build context budgets and persist inspectable compaction checkpoints while retaining original events.
- [ ] **Crash recovery.** Force restarts before and after the final message transaction and around approval, dispatch claim, execution and result recording. Every completed run replays its answer exactly once. Restore pending approvals, surface claimed calls with unknown outcomes, and verify that recovery never relaunches them automatically.

## Phase 4 — Connect GPUI to the service

- [ ] Connect GPUI to the versioned local protocol over per-user named pipes on Windows or Unix domain sockets on macOS and Linux. Resume after the last applied durable sequence and restore an active preview from a snapshot and offset.
- [ ] Build workspace and session navigation, streaming conversation, command output, approval controls, diffs, and provider settings. Wire keyboard controls, stop, retry, continue, and steering to service commands.
- [ ] **Reconnect and replay.** Disconnect mid-response and immediately after final commit but before delivery. Complete a turn during the replay-to-live handoff and redeliver an event. Compare the restored transcript with an uninterrupted client's transcript, including exact text, message IDs and order. After a service crash mid-stream, verify the interrupted status and cleared preview.
- [ ] Restart the GUI with a pending approval, then answer it from two windows. Both must show the same recorded decision. Verify end-to-end workflows and check rendering and memory use with long conversations and high-volume output.

## Phase 5 — Qualify the release

- [ ] Run the provider, runtime, tool, isolation, recovery, integration, UI, and performance test matrix. Evaluate real coding tasks in temporary repositories and record success, unnecessary changes, and resource usage.
- [ ] Add redacted diagnostics, package and sign supported builds, and verify installer, update, and recovery behavior on each target platform.
- [ ] For every advertised platform, record the build, platform/filesystem, pass/fail result and evidence for required isolation, safe editing, packaging and recovery checks. Repeat the successful existing-file edit and competing-save tests on each filesystem advertised for editing. Qualify the platform only after all required checks pass and release blockers are resolved.

## Phase 6 — Expand after the first release

- [ ] Add providers through the existing adapter boundary and introduce Markdown skills before MCP.
- [ ] Add interactive terminals, Git worktrees, and symbol-aware exploration as those workflows mature. Use explicit snapshots or worktrees when a workflow needs filesystem restoration.
- [ ] Introduce executable extensions only with versioned interfaces, isolation, and explicit capabilities. Run the existing stability and security checks against each addition.
