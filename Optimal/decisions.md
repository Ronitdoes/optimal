# Optimal — Architecture Decision Records
### Windows desktop / GPUI track · 2026-11-18

> Format: **Context → Decision → Rejected alternatives → Consequences.** An ADR is only changed by a new ADR that supersedes it. Every decision cites the research it came from (`../docs/AGENT-RESEARCH-*`, `BEST-OF-ADOPTION.md`, `BLIND-SPOTS-INDEX.md`).

---

### ADR-001 — GPUI, pinned, with a standalone fork fallback · **Accepted**

**Context.** GPUI is Zed's Rust UI framework: Apache-2.0, pre-1.0 (`gpui` 0.2.2, published 2025-10-22, updated 2026-10-30), breaking changes between versions; 511k downloads. On Windows it needs **no** `gpui_platform` features — Win32 windowing, DirectWrite text. A community fork `gpui-ce` (imported as `gpui`) is maintained specifically for standalone apps and adds `screen-capture`, `profiler`, `inspector`, `hot-patching`; `gpui-unofficial` 1.2.7 is a third option.

**Decision.** Use canonical `gpui` + `gpui_platform` from Zed, **pinned in `Cargo.lock` and to a commit in CI**. At P0 spike, evaluate `gpui-ce` for `screen-capture`/`profiler`; switch via ADR if the fork proves necessary for our screen-region capture feature.

**Rejected.** Tauri/Electron (extra runtime, web view, larger binary) · egui (different rendering model, less mature text) · Win32 direct (too much UI code).

**Consequences.** API churn is a real risk — no `gpui` version bumps without a UI regression run (`tests.md`).

---

### ADR-002 — Windows-only first · **Accepted**

**Context.** Research covers Windows UIA, COM, restricted tokens, MXC, Credential Manager, and Windows Sandbox; macOS/Linux need AT-SPI/AXUIElement and different sandbox stacks.

**Decision.** Ship Windows 10/11 x64 first. GPUI keeps the door open but no other OS is tested or promised in this track.

**Rejected.** "Cross-platform from day one" (triples the test matrix before trust is established).

**Consequences.** All adapters behind a `windows` feature gate; non-Windows builds are not supported here.

---

### ADR-003 — Rust core with `windows-rs`, not TS core + C# helper · **Accepted**

**Context.** The original plan used TypeScript + a .NET helper (FlaUI, Restart Manager, IFileOperation). Codex (139 Rust crates) and OMP (Rust N-API natives) show Rust can own the whole stack; `windows-rs` 0.61 gives typed UIA/Win32/COM without a managed sidecar. `gpui` is Rust.

**Decision.** One Rust workspace; `windows-rs` for UIA, Win32, DPAPI, WFP, Credential Manager. Python only as an *optional later* sidecar for Whisper/OCR (Windows.Media.Ocr first).

**Rejected.** TS+Bun core (two runtimes, IPC boundary) · C# helper (extra binary, COM interop debt).

**Consequences.** We must write more low-level code; we gain one binary, no GC pauses, and `gpui` integration without an FFI bridge.

---

### ADR-004 — Single process, local JSON-RPC, never elevated · **Accepted**

**Context.** OpenCode/Codex/T3 all converge on one local service behind every surface. Codex's daemon explicitly refuses to start elevated. T3 now ships a Windows service with remote update.

**Decision.** One Rust core owns loop, policy, state, effects. GPUI talks to it over local JSON-RPC/WS with a capabilities handshake. Per-verb elevation only via gsudo; the daemon runs as the normal user.

**Rejected.** Elevated agent (UIPI bypass + full-disk blast radius) · per-surface cores (state divergence).

**Consequences.** Cross-surface edits serialize through the core's lock service (phase P2).

---

### ADR-005 — Event store with atomic outbox, receipts, and a hash chain · **Accepted**

**Context.** T3's `EventSink`/`EffectWorker`/`CommandReceiptStore` is the working durability pattern; our old plan only named "event-sourced from day 1". OpenCode's `bus.ts` shows the durable envelope + atomic projection commit.

**Decision.** `events(seq, stream_id, type, payload, causation_id, correlation_id, ts)` in SQLite WAL; one transaction commits event + projection + outbox; singleton worker with leases + fencing; `command_receipts` unique index; append-only hash-chained audit; JSONL transcripts. No 12k-line monolith.

**Rejected.** Plain append-only log without receipts (duplicate effects) · ORM migrations without version table.

**Consequences.** Replay and "why did you do that" are first-class; migrations need expand→migrate→contract tests.

---

### ADR-006 — Deny-by-default policy with Z3 validation and TOCTOU recheck · **Accepted**

**Context.** T3 defaults new threads to `full-access`; Grok's classifier is "best-effort"; most vendors fail open. Codex uses most-restrictive-wins and refuses policies it can't enforce.

**Decision.** Tiers `readonly|auto|confirm|never`; default `never`. Validate `policy.toml` with Z3 at load (conflicts, overbroad rules). Split commands with tree-sitter; most restrictive decision wins. Recheck authorization **after awaits, under the same lock that commits the effect** (T3 pattern). Any hook failure = deny.

**Rejected.** Regex-only matching · first-match-wins without canonicalization · fail-open hooks.

**Consequences.** Slightly slower prompts; materially smaller attack surface.

---

### ADR-007 — Capability isolation is the boundary; Reader/Actor is a utility filter · **Accepted**

**Context.** Adaptive attacks exceed 90% ASR against in-band defenses (BLIND-SPOTS-07 B-01). Multi-CaMeL proves provenance-preserving channels can hit 0% ASR. Every competitor calls its auto-review "not a security boundary".

**Decision.** Planner/reader split with capability labels and a build-time destination allowlist. Retrieved/page content is `untrusted-data` and never reaches a send/write tool without a receipt. Guardian, classifiers, and hooks are advisory only.

**Rejected.** Treating Reader/Actor or an LLM classifier as the gate.

**Consequences.** More explicit plumbing; meets the actual threat model.

---

### ADR-008 — Code Mode via `rhai` with host-owned budgets · **Accepted**

**Context.** OpenCode's confined JS VM and Kilo's host-owned budgets solve tool-catalog bloat (10–100× fewer round-trips); Code Mode's "37%" marketing figure is borrowed and must not be quoted (research 04).

**Decision.** Expose many tools as a small `rhai` namespace inside one `code_execute` tool. Host owns `timeout_ms`, `max_tool_calls`, `max_output_bytes` — no permissive defaults. Re-check permission for every inner call with nested attribution; ban nested Code Mode; cap concurrency at 8.

**Rejected.** `eval` of arbitrary JS/Python · compiled bytecode VM (too much surface for MVP).

**Consequences.** Need a safe host API; synthetic benchmark claims are banned in docs/tests.

---

### ADR-009 — TTSR stream-time rule injection · **Accepted**

**Context.** OMP's Time-Traveling Stream Rules are the strongest realization of "hooks-as-runtime-policy": abort mid-token, inject a rule, retry from the same point, survive compaction.

**Decision.** Implement a small Rust coordinator over the model token stream: regex/AST conditions, tool-scoped abort, injected `<system-interrupt>`, resume from the same token; AST conditions checked in the awaited pre-tool hook. Port the design, not the 25 KB TS coordinator.

**Rejected.** Post-hoc only checking (too late to prevent the tool call).

**Consequences.** Model client must expose streaming deltas and support a mid-stream continuation.

---

### ADR-010 — OTel with a pinned internal attribute adapter · **Accepted**

**Context.** GenAI semconv is still "Development" and moved repos in v1.42.0; "OTel-compatible" is not a stable wire contract (BLIND-SPOTS-03 #1). OMP ships a working `OmpGenAIAttr` adapter; Grok added an emit-time fail-closed validator after its exfil scandal.

**Decision.** Emit `invoke_agent`/`chat`/`execute_tool` spans with stable *internal* names mapped to `gen_ai.*` by a pinned adapter; content capture **OFF** by default; W3C `traceparent` carried in `correlationId`; export-time schema validator that drops invalid spans.

**Rejected.** Emitting raw `gen_ai.*` without an adapter.

**Consequences.** Dashboards survive semconv renames; telemetry change requires a regression test.

---

### ADR-011 — Signed, blue-green updates with a real trust root · **Accepted**

**Context.** T3 v0.1.10 ships active/trial Windows-service updates with DB backup, durable commit marker, rollback, migration preflight — but verifies only `SHA256SUMS` from the same endpoint. grok-build's server-side-only telemetry switch is the anti-pattern.

**Decision.** TUF roles (offline Root threshold ≥2), Sigstore keyless + SLSA provenance, SBOM, signature verified **before apply**. Blue-green active/trial service with DB backup and rollback. In-app "verify current install"; client-side kill switch for telemetry.

**Rejected.** Checksum-only updates · remote-only kill switches.

**Consequences.** Release pipeline complexity; justified by the supply-chain threat record.

---

### ADR-012 — Apache-2.0 core, `ee/` carve-out, MIT SDKs · **Accepted**

**Context.** All major self-hosted agents ship permissive licenses; BSL reversals and surprise licenses (Crush FSL-1.1-MIT, Zed GPL crates) damaged trust (research 04/05). GPUI is Apache-2.0, compatible.

**Decision.** `LICENSE` = Apache-2.0 for core; `ee/` enterprise features under a commercial license; TS/Python SDKs and the tool contract MIT; `LICENSING.md` before the first external PR; verify SPDX of every dependency in CI.

**Rejected.** BSL/source-available · no license until "later".

**Consequences.** Contributions require a CLA-free DCO; enterprise features stay clearly separated.

---

### ADR-013 — Three-axis rewind in a separate snapshot repo · **Accepted**

**Context.** Cline/Claude/OpenCode implement `code|conversation|both` rewind; OpenCode stores snapshots in a dedicated repo so the user's git index stays clean; T3's restore-safety gate rejects before touching FS if the conversation can't roll back.

**Decision.** `refs/optimal/checkpoints/*` in a **separate** git repo under `~/.optimal/`; three independent restore axes; "can't roll back conversation → reject before changing files"; cap snapshot bytes/count; symlink/escape checks fail closed.

**Rejected.** Checkpoints in the user's repo · conversation-only undo.

**Consequences.** Extra disk use is bounded by resource caps; users keep their git history pristine.

---

### ADR-014 — Meaningful oversight + two-key for irreversible ops · **Accepted**

**Context.** PNAS 2024: HITL raises uptake while lowering accuracy (BLIND-SPOTS-05 #1–2). Dots' guardian is non-deterministic; Grok says "approval does not reverse work already done"; Grok allowed 500 payments under one tap.

**Decision.** `reviewability` field: route to a human only when error cost is high **and** the card lets them independently judge correctness. Consent grants carry scope/TTL/revoke with signed receipts. Track fatigue KPIs and rubber-stamp rate. Irreversible ops require two independent attestors. Every confirm card shows exact scope and what will be reverted.

**Rejected.** "Add a confirm button" as safety · auto-approve to reduce cards.

**Consequences.** Some tasks pause; that is the point.

---

### ADR-015 — BYOK, visible role routing with fallback chains · **Accepted**

**Context.** Dots now routes models in-task; Codex issues #32692/#56819 show silent swaps and ignored config; Grok users cannot pick a model. OMP's nine-role router + fallback chains + path scoping is the best engineering answer.

**Decision.** Provider-agnostic adapters with BYOK; nine roles; per-role fallback on 429/quota with cooldown; path-scoped `enabledModels`/`disabledProviders`; visible model/effort; published matrix; team allowlists honoured. No silent swap, ever.

**Rejected.** Hardcoded single provider · hidden routing.

**Consequences.** Router needs a small, testable config schema.

---

### ADR-016 — Compaction: measured tokens, shrink ladder, `keepID` · **Accepted**

**Context.** OpenCode's compaction implementation is the best available spec; compaction invalidates cached approvals in Codex (Guardian `IncompatibleCompaction`).

**Decision.** Trigger on measured provider tokens at ~75%; background precompute with cache-TTL guard; shrink ladder `[0.85, 0.7, 0.5, 0.35]`; summaries reference kept messages by `keepID` instead of duplicating text; checkpoint at every compaction; invalidate cached approvals.

**Rejected.** Fixed character-count threshold · duplicate text in summaries.

**Consequences.** Model clients must report measured token usage.

---

### ADR-017 — Credentials: OS vault, one-use refs, no card data in context · **Accepted**

**Context.** Grok Bot could see and recite saved card numbers/CVV; T3 uses one-use HMAC secret refs; Codex uses `Redacted` types; Dots/Wallet hides the card from agent and merchant.

**Decision.** Credential Manager/DPAPI; `{{vault:ref}}` placeholders substituted only inside the executor; one-shot merchant-scoped tokens with Windows Hello confirm; secret refs expire and are consumed before disclosure; payments are a later, two-key-gated capability; no card data in model context, logs, or memory artifacts.

**Rejected.** Secrets in env/config/model context · long-lived tokens.

**Consequences.** Executor needs a last-moment substitution step.

---

### ADR-018 — The five Optimal docs are release gates · **Accepted**

**Context.** Complex agent projects drift into "it works on my machine" and unverified claims (Code Mode's borrowed 37%).

**Decision.** `phases.md` steps are not done until `dod.md` is satisfied and `tests.md` manual checklist is executed and recorded. No step ships with an unrun manual test.

**Rejected.** Automatic CI-only gating (misses UX, DPI, UIA, and accessibility issues).

**Consequences.** Slower but honest releases; manual test log is part of the repo.

---

### ADR-019 — Egress and inference routing are one declarative policy object · **Accepted (2026-11-18)**

**Context.** Hermes + NVIDIA OpenShell apply filesystem, L7 network egress, process/syscall, and **inference-routing** policy as one hot-reloadable object with credentials injected from a provider store; its iron-proxy refuses sandbox start when half-configured. Our O1.4 was a build-time destination allowlist only. Cross-reference: `AGENT-RESEARCH-09A §4.7`, `09C §3B`.

**Decision.** Extend O1.4/O8.4: one policy object covers destinations, L7 rules, model/endpoint routing, and proxy controls; half-configuration is a hard refusal; proxy-control environment variables are protected from user override; every outbound event carries a "what left the machine" receipt. Host/Origin validation on the local WS upgrade is mandatory (CVE-2026-53869 DNS-rebinding class).

**Rejected.** Static allowlist with no inference-routing binding · fail-open proxy on config error.

**Consequences.** Router and egress share a schema; tests must cover half-configured states.

---

### ADR-020 — Bounded frozen memory; autonomous writes are staged, never silent · **Accepted (2026-11-18)**

**Context.** Hermes caps and freezes injected memory at session start (2,200/1,375 chars) and stages skill writes — but ships `write_approval: false`, and real users report memory silently not persisting (issue #56461) and "hidden state". OpenClaw injects up to 60,000 chars. Cross-reference: `AGENT-RESEARCH-09A §4.12–4.14`, `09B §4`, `09C §3C–D`.

**Decision.** Adopt hard caps + session-freeze + first-run deferral + pinned-by-scheduler references at O2.4, **plus** our re-validation against current files. All autonomous memory/skill writes are staged, signed, and review-required by default (O2.6/O4.4). A per-skill lock refuses programmatic writes. Never derive consent grants from approval history.

**Rejected.** Silent self-improvement · unbounded memory injection · treating frozen caps as a measured poisoning defense (it is architectural only).

**Consequences.** Slightly slower skill creation; user-visible diffs for every self-change.

---

### ADR-021 — Offline profile gets a verified local runtime and a zero-outbound proof · **Accepted (2026-11-18)**

**Context.** Hermes is the best offline reference in the class: pinned SHA-256-verified llama.cpp, hardware-aware sizing, ≥64K context, ≥4-bit floor, least-harm spill order, 15-min idle unload — but publishes **no** trace proof that nothing left the machine. Cross-reference: `AGENT-RESEARCH-09C §3F`.

**Decision.** O7.3 copies the runtime-sizing/verification design and adds a startup self-check plus a trace assertion of **zero outbound spans** when the offline profile is active. Signed offline update bundle. Resource caps (O7.4) adopt idle unload and placement policy.

**Rejected.** "Local-first" with remote-execution defaults or cloud-portal coupling (Hermes README advertises "$5 VPS / talk to it from Telegram while it works on a cloud VM").

**Consequences.** Offline mode must be testable with the network physically disabled (`tests.md O7.3`).

---

### ADR-022 — Governance as a trust feature: honest boundary docs + migration import · **Accepted (2026-11-18)**

**Context.** Hermes' SECURITY.md is the most honest in the field (OS is the only boundary; in-process controls are heuristics; "profiles are not tenants"; 90-day disclosure; 13-step hardening list), and `hermes claw migrate --dry-run` lowers switching cost. But issue #10232 shows maintainer suppression of good-faith concerns, and a look-alike download domain caused backlash. Cross-reference: `AGENT-RESEARCH-09B §5 D15`, `09C §3G–H`.

**Decision.** Add **O8.5**: publish a load-bearing `SECURITY.md` that names exactly one boundary and declares every heuristic; ship `optimal hermes migrate --dry-run` importing skills/memories/allowlists into our vetted pipeline; published governance, code of conduct, response SLA, and a single signed distribution channel (no `irm…|iex`, no AV-whitelist strategy).

**Rejected.** Feature-only documentation · relying on upstream maintainer goodwill · look-alike/unverified install paths.

**Consequences.** Documentation and importer are release-gated like code (ADR-018).

---

### ADR-023 — No LangGraph (or any graph agent framework) in the core loop · **Accepted (2026-11-18)**

**Context.** LangGraph sells durable execution, checkpoints, human-in-the-loop pauses, cycles, and streaming. `PLAN.md` §15 already rules out LangGraph/CrewAI/AutoGen for the core loop ("plain tool loop + owned abstractions beats framework overhead"), and §31 shows multi-agent frameworks fail **41–86.7%** of actions (MAST). Our Optimal stack is a single Rust binary with GPUI; adding a Python/JS graph runtime would introduce a second runtime, an IPC boundary, and a **second source of truth for session state**.

**Decision.** Build the core loop, subagent scheduling, and checkpoints natively in Rust (O0.3, O2.1–O2.3, O6.1–O6.3). Graph-shaped logic (deliverable DAG, deadline ladders, scheduler triggers, finish gate) is expressed as small deterministic state machines over the event store — never as a framework graph. LangGraph/Microsoft Agent Framework are **interop targets** via MCP/A2A if a customer already uses them, not dependencies.

**Rejected.** LangGraph as orchestrator · LangGraph as checkpoint store · adopting a graph framework to "get" HITL or durability we already own.

**Consequences.** More code to write for scheduling and resume, but one state substrate, one audit chain, and no framework-version churn. Revisit only by a new ADR if a specific workflow cannot be expressed as a small state machine — and then confine it to a sidecar, never the core.

**Note.** License is not the objection (LangGraph is MIT); the objection is architectural. Narrow Python ML sidecars (Whisper/OCR) remain optional and separate (ADR-003).
