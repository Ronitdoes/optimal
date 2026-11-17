# Optimal — Windows Desktop Agent (GPUI)
### Build plan · 2026-11-18

> **Scope of this folder.** This is the *optimal, narrowed* track derived from `../docs/` (21 research files). We build **one local-first Windows desktop agent** with a **GPUI** UI and a **Rust** core. Everything in `PLAN.md` that is not needed for a Windows desktop MVP is explicitly deferred (§6).
> **Relationship to other docs:** `../docs/BEST-OF-ADOPTION.md` gives the mechanisms; `../docs/BLIND-SPOTS-INDEX.md` gives the must-fix gaps; `../IMPLEMENTATION-PHASES.md` is the broad plan. This folder is the executable subset.

---

## 1. Vision

One GPUI window on Windows 11 that can safely do your computer work: read/write files, drive git, run shell commands, operate other desktop apps through their accessibility tree, remember project context, and ask for permission only when it matters — with receipts for everything.

**Non-negotiable principles (inherited from research)**
1. **The model never touches the OS.** Every effect: `tool call → canonicalize → policy → OS boundary → execute → hash-chained audit`.
2. **Deny-by-default.** Unknown agents/tools/commands = `never`. Auto-review and classifiers are *advisory*, never the boundary.
3. **One binary, one service.** GPUI clients are thin; the Rust core owns loop, policy, state, effects.
4. **Durable and replayable.** SQLite event store + outbox + command receipts + 3-axis rewind.
5. **Capability isolation is the wall.** OS-enforced sandbox + egress DLP + two-key for irreversible ops.
6. **Boring, verifiable, local.** Predictability over autonomy theater.

---

## 2. Tech stack (decided; rationale in `decisions.md`)

| Layer | Choice | Note |
|---|---|---|
| Language | **Rust (stable, latest)** | Single binary, `windows-rs`, no GC pauses, matches Codex/OMP/Zed |
| UI | **GPUI** (`gpui` + `gpui_platform`), **Apache-2.0** | Windows: Win32 + DirectWrite, **no platform features required**; pre-1.0 → pin version + lockfile (ADR-001) |
| Windows APIs | `windows-rs` 0.61 (UIA, Win32, DPAPI, WFP, registry, Credential Manager) | replaces the old C#/FlaUI helper (ADR-003) |
| Event store | `rusqlite` (SQLite WAL) + JSONL transcripts | event + projection + outbox in one transaction (ADR-005) |
| Async | `tokio` + `async-trait` | streaming, cancellation, background jobs |
| Policy validation | `z3` crate at config load + tree-sitter command splitting | temporal/shield rules (ADR-006) |
| Code Mode | `rhai` embedded interpreter with host-owned budgets | no arbitrary `eval` (ADR-008) |
| MCP | `rmcp` (stdio + Streamable HTTP, 2026-07-28 revision) | namespaced, pinned servers |
| Telemetry | `opentelemetry` + pinned internal attribute adapter, OTLP export | content capture OFF (ADR-010) |
| Updates | TUF + Sigstore keyless + SLSA provenance | signed manifests, verified before apply (ADR-011) |
| Shell | PowerShell 7 subprocess, `-EncodedCommand`, structured JSON output | per-command timeout, no elevated daemon |
| Model | BYOK provider adapters (OpenAI/Anthropic/local HTTP), role router + fallback chains | visible model/effort, published matrix |
| License | **Apache-2.0 core + `ee/` carve-out** | MIT SDKs; see ADR-012 |

**Not in the MVP:** TypeScript core, Python sidecars (Windows.Media.Ocr via `windows-rs` first), Tauri/Electron, Node runtime.

---

## 3. Architecture

```
┌──────────────────────── GPUI window (thin) ────────────────────────┐
│ chat · task tree · streaming tokens · tool badges · diff cards     │
│ permission cards · budget meter · ambient 4-state · hotkey palette │
│ screen-region capture · tray · notification centre                 │
└───────────────────────────┬────────────────────────────────────────┘
              local JSON-RPC/WS (capabilities handshake, v1)
┌───────────────────────────▼────────────────────────────────────────┐
│ RUST CORE (single process, no elevated token)                      │
│  loop · planner · compaction · subagent scheduler · guardian client │
│       every effect passes through ▼                                │
│  POLICY: canonicalize → rule engine (Z3-checked) → tier →           │
│          TOCTOU recheck under commit lock → OS boundary → execute   │
└───────────────────────────┬────────────────────────────────────────┘
┌───────────────────────────▼────────────────────────────────────────┐
│ TOOL REGISTRY (branded risk declarations, probes, uniform result)  │
│  fs · shell · git · UIA · COM(Office) · env · mcp/* · code_execute  │
└───────────────────────────┬────────────────────────────────────────┘
┌───────────────────────────▼────────────────────────────────────────┐
│ WINDOWS EFFECT ADAPTERS                                             │
│  UIA tree (windows-rs) · SendInput · Win32 windows/DPI · COM ·      │
│  Credential Manager/DPAPI · restricted token / MXC · egress DLP     │
└───────────────────────────┬────────────────────────────────────────┘
┌───────────────────────────▼────────────────────────────────────────┐
│ STATE: SQLite events + outbox + receipts + hash chain · JSONL       │
│  snapshots in a SEPARATE git repo (refs/optimal/checkpoints/*)      │
└────────────────────────────────────────────────────────────────────┘
```

**Crate layout**
```
optimal-src/
├─ Cargo.toml (workspace, pinned deps)
├─ crates/{core,policy,registry,tools,runtime,store,model,ui,gateway,guardian,telemetry}/
├─ runner/          # Windows service launcher + signed updater (blue-green)
├─ evals/           # rule tables, injection corpus, task suite, manual scripts
└─ docs/            # this folder (plan, decisions, phases, dod, tests)
```

---

## 4. Capability map (Windows desktop subset)

| Domain | In MVP | Mechanism from research |
|---|---|---|
| Files & git | ✅ | content-addressed snapshots in separate repo; three-axis rewind; hashline-style edits |
| Shell / PowerShell | ✅ | tree-sitter command splitting; most-restrictive rule wins; `denyInsteadOfPrompting` |
| Coding loop | ✅ | finish gate + TTSR stream rules; semantic LSP refactor as rule; ranked repo map |
| Windows GUI automation | ✅ (P5) | UIA tree via `windows-rs`, OCR fallback, SendInput last, post-action verification |
| Context / memory | ✅ | measured-token compaction + `keepID`; file-first `AGENTS.md`/`plan.json`; TTL + re-validation |
| Subagents | ✅ (P2) | schema yields + JSON-path reads, worktree isolation, park/revive, async mailbox |
| MCP / skills / plugins | ✅ (P4) | stdio + Streamable HTTP, hash-trusted hooks, branded risk declarations, vetted install |
| Scheduling / background | ✅ (P6) | durable triggers, command receipts, guardian review, notifications |
| Comms / email | ⛔ deferred | draft-first sending designed but not built in this track |
| Browser agent | ⛔ deferred | CDP/Playwright later; extension bridge not in MVP |
| Cloud / DevOps | ⛔ deferred | CLI adapters later |
| Mobile / web / macOS / Linux | ⛔ deferred | GPUI is cross-platform but this track targets Windows only |
| Social OS / tenancy / verticals | ⛔ deferred | receipts substrate supports them later |

---

## 5. Safety baseline (carried in from every report)

- **Two-axis sandbox:** OS boundary (`PermissionProfile`) independent of approval policy (`AskForApproval` × reviewer). Fail closed when the OS cannot enforce.
- **Windows ladder:** MXC (if available) → restricted token + capability SIDs → refuse. Never elevated; gsudo per-verb.
- **Egress DLP:** deny-by-default, cloud metadata (169.254.169.254) hard-denied, build-time destination allowlist, "what left the machine" receipt.
- **Capability isolation:** reader/planner split, capability labels, no retrieved text reaches a send/write tool without a receipt.
- **Meaningful oversight:** `reviewability` field — a human is asked only when they can independently judge correctness; otherwise deterministic guard.
- **Guardian:** separate reviewer, advisory; Z3-validated policy + two-key for irreversible; model cannot edit its own gate.
- **TTSR:** stream-time rules abort mid-token, inject a correction, retry from the same point, survive compaction.
- **No unsafe defaults:** no `yolo`, no fail-open hooks, no full-access default, no unbounded approval, no whole-repo telemetry, no silent model swaps.

---

## 6. Explicitly deferred (out of scope for this folder)

| Deferred | Why |
|---|---|
| Browser agent, Playwright/extension bridge | separate capability; needs its own injection corpus |
| Email/calendar/cloud/DevOps | after the desktop loop is trustworthy |
| VS Code extension, web dashboard, mobile | one surface first |
| macOS/Linux parity | Windows-first ADR-002 |
| i18n, non-Gregorian time, international regimes | global track later |
| Social OS / multi-tenancy / obligation-custody verticals | later products on the same receipts substrate |
| Payments beyond one-shot draft | S5.6 later; two-key gate required |

---

## 7. Success criteria for this track

1. A 5-step local task runs end-to-end in the GPUI window with visible plan, streaming, diffs, and permission cards.
2. Zero secrets in transcripts/logs; audit chain verifies; kill switch works mid-task.
3. Rule-engine table tests + injection corpus + adaptive attack corpus green in CI.
4. Delete-a-file → restore from checkpoint → clean working tree; three-axis rewind verified.
5. 5/10 daily Windows desktop tasks pass with default approvals via UIA.
6. Signed update verified before apply; rollback drill passes; offline profile makes zero outbound calls.
7. Own UI keyboard-operable with visible focus; permission dialog readable by screen reader.

---

## 8. Research lineage

All claims trace to `../docs/`:
- `BEST-OF-ADOPTION.md` §3 (steal list), §4 (do-not-copy), §5 (corrections)
- `BLIND-SPOTS-INDEX.md` §3 (must-fix), §5 (stale facts)
- `AGENT-RESEARCH-01..08-*.md` (source-level evidence with file/line refs)
- `AGENT-RESEARCH-09-HERMES.md` + `09A/09B/09C` (strongest **local-first** peer: identity, source audit, 395 user stories, OpenClaw/MantisClaw comparison)
- `IMPLEMENTATION-PHASES.md` (broad plan; this folder is its Windows/GPUI subset)

**Hermes-specific scope additions (2026-11-18):** L7 egress + inference-routing policy as the same declarative object (O1.4/O8.4); bounded frozen memory with re-validation (O2.4); reviewable self-improvement with staged writes + per-skill lock (O4.4); managed offline local-model runtime with trace-proven zero outbound (O7.3); migration importer + honest SECURITY.md (O8.5). Hermes' `smart` auto-approve and `write_approval: false` defaults are explicitly inverted.
