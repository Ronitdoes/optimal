# Development Phases

See [Implementation steps](implementation-steps.md) for the ordered build and verification checklist.

## 14. Development Roadmap

### Phase 0 — Foundation

Validate GPUI on Windows, event channels, Tokio integration, SQLite writes, streaming HTTP, and tool-runner isolation.

**Exit criterion:** Reproducible proof of concept and feasible sandbox strategy.

### Phase 1 — Harness

Build normalized messages, the OpenRouter HTTPS adapter with API-key authentication, streaming parser, state machine, tool-call validation, mock providers, and minimal CLI.

**Exit criterion:** The headless CLI completes a deterministic multi-step task using the in-memory store. The stream fixtures dispatch a validated call once on a clean tool turn and zero times when valid arguments are followed by a stream failure.

### Phase 2 — Tools

Build file tools, ripgrep, shell execution, Git status/diff, permissions, workspace checks, cancellation, and edit conflicts.

**Exit criterion:** Fix a failing test in a dirty temporary repository and show the agent's exact edit alongside the full workspace changes. The concurrent save/create, protected-file access and workspace mutation lock tests in the [implementation checklist](implementation-steps.md) pass.

### Phase 3 — Persistence

Add SQLite migrations, event history, branching, recovery, usage, keychain, context, and compaction.

**Exit criterion:** Forced restart restores completed answers and pending approvals, exposes uncertain operations and never silently reruns them. Racing approval replies yield at most one dispatch.

### Phase 4 — Desktop

Build workspace navigation, streamed conversations, approvals, diffs, settings, session switching, keyboard controls, and IPC reconnection.

**Exit criterion:** Core coding workflows work through GPUI. Reconnect during a streamed answer or just after commit reconstructs one complete answer with no missing or duplicate text. Two windows resolving the same request observe one decision.

### Phase 5 — Hardening

Perform security, resource-limit, compatibility, crash, installer, and platform sandbox tests; add diagnostics and releases.

**Exit criterion:** Each advertised platform has passing evidence for required isolation, safe editing on supported filesystems, packaging and recovery checks. A failed or unverified required check blocks that platform's release.

### Phase 6 — Expansion

Add providers, skills, MCP, interactive terminals, Git worktrees, symbol-aware exploration, and eventually safe executable extensions.

**Exit criterion:** New features preserve stability and modularity.

### Release scope

| Capability | First public release | Later |
|---|:---:|:---:|
| GPUI desktop application | Yes | |
| Native Rust harness | Yes | |
| OpenRouter via HTTPS API key | Yes | |
| File editing and shell | Yes | |
| Permission system | Yes | |
| SQLite sessions | Yes | |
| Context compaction | Yes | |
| Git diff/review | Yes | |
| Enforceable sandboxing | Yes | |
| Additional providers | | Yes |
| MCP/extensions | | Yes |
| Multiple agents | | Yes |
| Semantic code indexing | | Yes |
| Remote execution | | Yes |

Only call a release production-ready on operating systems where sandboxing, packaging, and recovery have actually been validated. Windows-first is reasonable.
