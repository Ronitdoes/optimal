# Optimal — Phases & discrete steps
### Windows desktop / GPUI · 2026-11-18

> **Rule:** safety spine before capabilities. A step is not done until `dod.md` passes and `tests.md` is executed and recorded. Step IDs are stable; manual tests are keyed to them.
> **Research lineage:** `[BEST §n]` = `../docs/BEST-OF-ADOPTION.md`; `[BS §n]` = `../docs/BLIND-SPOTS-INDEX.md`; `[R0n]` = `../docs/AGENT-RESEARCH-0n-*.md`.

---

## Phase O0 — Skeleton & safety spine (weeks 1–2)

**Goal:** a GPUI window that runs a governed multi-step local task; nothing mutates outside policy.

| Step | Work | Done when |
|---|---|---|
| **O0.1** | Rust workspace, pinned deps (`gpui`, `gpui_platform`, `windows-rs`, `tokio`, `rusqlite`); GPUI `Application` + root view; DPI awareness at startup; single-instance lock `[ADR-001/002/003]` | Clean clone builds; window opens on Win11 at 100%/125%/175% DPI; `cargo build --release` produces one exe |
| **O0.2** | Tool registry: uniform envelope `{status, stdout, stderr, exit, artifacts[], durationMs, correlationId}`, branded risk declarations, `probe.cmd` gating, `tools.search` `[BEST §3 #5]` | Registering a tool without a branded declaration fails to compile; unavailable tool is unreachable |
| **O0.3** | Agent loop: PERCEIVE→PLAN→ACT→REFLECT, interrupt queue checked between tools, Esc/Ctrl+Alt+Space kill, plan-as-file `plan.json` + `todos.md`, bounded retries + idempotency keys | 5-step read-only task completes; kill switch stops in-flight work with completed effects logged |
| **O0.4** | Policy spine: canonicalize (PATH resolve, case/slash/8.3/UNC/symlink), `readonly|auto|confirm|never` tiers, default `never`, Z3 validation at load, tree-sitter command split `[ADR-006]` | 60+ rule table tests green incl. `npm` hijack, `Invoke-WebRequest | iex`, compound commands |
| **O0.5** | Hash-chained audit + secret redaction before persistence + DPAPI vault refs `[BEST §3 #25]` | Injected fake secret never in `events.jsonl`; chain tamper test fails as expected |
| **O0.6** | Local JSON-RPC/WS gateway with capabilities handshake and `harness.v1.*` namespacing; GPUI renders streamed tokens + tool badges `[BEST §3 #9]` | "Hello" task streams end-to-end from core to window |
| **O0.7** | Eval harness v0: rule tables, canary files, transcript scrubber in CI | CI fails on leaked secret / canonicalization regression / canary touched |

> **O0 Gate:** multi-step task end-to-end; zero secrets; kill switch from UI and hotkey; audit chain verifies.

---

## Phase O1 — Code agent, git, rewind & Windows boundary (month 1)

**Goal:** safe coding on a real repo with rollback and an OS-enforced boundary.

| Step | Work | Done when |
|---|---|---|
| **O1.1** | Git adapter (`status --porcelain=v2 -z`, diff, commit) + snapshots in a **separate** repo `refs/optimal/checkpoints/*`; capture before mutation; `FSUtil.contains` equivalent `[BEST §3 #12; R01]` | Delete file → restore → clean tree; user's git index untouched |
| **O1.2** | Three-axis rewind (`code|conversation|both`) + T3 restore-safety gate ("can't roll back conversation → reject before touching FS") | Rewind conversation without file change, then files only, then both; each verified |
| **O1.3** | Confirm UX with unified diff preview, matched rule + tier, exact scope, "what will be reverted"; ordered decision menus; batch attribution `[BEST §3 #6]` | Every destructive eval case required confirm; no scope overreach (`rm one-file` ≠ any `rm`) |
| **O1.4** | Egress DLP: deny-by-default, private ranges + 169.254.169.254 hard-denied, build-time destination allowlist, "what left the machine" receipt `[BEST §4]` | SSRF/metadata tests blocked and logged; no outbound call without a receipt |
| **O1.5** | Windows boundary: MXC → restricted token + capability SIDs → refuse; never elevated; gsudo per-verb; WFP offline rule where applicable `[BEST §3 #1/#2; R03]` | Daemon refuses elevated start; unenforceable policy is refused, not downgraded |
| **O1.6** | Finish gate + loop breaker + TTSR v0 (abort mid-token → inject rule → retry from same point, survives compaction) `[BEST §3 #8]` | Deliberate loop broken in ≤3 reps; stream rule catches a forbidden write before the tool call |
| **O1.7** | Budgets: per-task token/$ ceiling, max tool calls/min, max bytes/write; breach → pause + ask | Budget breach pauses rather than degrading silently |

> **O1 Gate:** 100+ rule tests; rollback drill; three-axis rewind; OS boundary fail-closed; no unconfirmed destructive op; adaptive corpus v0 green.

---

## Phase O2 — Durability, context & telemetry (month 2)

| Step | Work | Done when |
|---|---|---|
| **O2.1** | Event store: `events(seq, stream_id, type, payload, causation_id, correlation_id, ts)`; event+projection+outbox in one transaction; singleton worker with leases/fencing; dead-letter `[BEST §3 #3]` | Kill mid-command → restart → receipt prevents duplicate effect; projection rebuild checksum matches |
| **O2.2** | Command receipts unique index + property "no effect without receipt"; schema_migrations table expand→migrate→contract, fixtures from last 3 versions | 10k fuzzed permission matrices pass; v-1 DB migrates forward |
| **O2.3** | Compaction: measured-token trigger (~75%), background precompute with cache TTL guard, shrink ladder `[0.85,0.7,0.5,0.35]`, `keepID` references, checkpoint + invalidate cached approvals `[ADR-016]` | Compaction round-trips with no loss; summary references kept messages, never duplicates |
| **O2.4** | File-first memory (`AGENTS.md` ≤30 lines, `plan.json`, `todos.md`, `env.md`) with TTL/staleness + re-validation | Stale recalled fact is re-checked against current files before acting |
| **O2.5** | OTel spans with pinned internal adapter + OW3C `traceparent` in `correlationId`; export-time fail-closed validator; content capture OFF `[ADR-010]` | One task traced core→tool→effect with correlated IDs; invalid span dropped |
| **O2.6** | Guardian client (advisory) + consent ledger (scope/TTL/revoke + signed receipts) + fatigue KPIs + ambient state plumbing `[ADR-014]` | Rubber-stamp rate (<1s approvals) measured; consent grant expires on schedule |

> **O2 Gate:** replay after kill; no duplicate effects; compaction lossless; telemetry correlates; consent expiry verified.

---

## Phase O3 — GPUI UX surfaces (month 2–3)

| Step | Work | Done when |
|---|---|---|
| **O3.1** | Task tree with statuses, streaming tokens, tool-call badges, elapsed/cost footer, budget bar | User sees a 5-step plan with live status without opening logs |
| **O3.2** | Diff cards with accept/reject blocks; permission cards with full context + ordered decisions; counterfactual "why / what would change this" | A file edit round-trips accept/reject; card shows matched rule, tier, scope, revert |
| **O3.3** | Ambient 4-state channel (idle/progressing/needs-you/stuck) with non-color coding; notifications/tray pause-resume; deep link "open in window" | Ambient state matches core state across pause/resume |
| **O3.4** | Global hotkey palette, screen-region capture (GPUI `screen-capture` or Win32 fallback), voice bar stub | Hotkey opens palette; captured region attaches to a prompt |
| **O3.5** | Takeover protocol: graduated yellow→orange→red, pre-staged context packet (diff, last 5 actions, screenshot, open files), Takeover Time Budget | Takeover under budget with context packet; time-to-correct-intervention recorded |
| **O3.6** | Own-UI accessibility: keyboard-only path through every confirm, visible focus, screen-reader announcements, ≥18pt option, low-stimulation mode | NVDA/JAWS user completes a confirm flow unaided; axe/keyboard CI green |

> **O3 Gate:** all six surfaces render from one event stream; reviewable confirms; accessibility audit green.

---

## Phase O4 — Tools, MCP & extensibility (month 3)

| Step | Work | Done when |
|---|---|---|
| **O4.1** | Windows adapters: fs (recycle-bin deletes, OneDrive placeholders, zip-slip guard), PowerShell 7 `-EncodedCommand`, env.probe, Everything/git search | 20 adapter unit tests; destructive delete goes to Recycle Bin |
| **O4.2** | LSP-in-the-loop pattern (diagnostics after every edit; semantic rename as a rule) — copy the **pattern**, not OpenCode v2 files `[BEST §5]` | `publishDiagnostics` fed back within one turn; semantic rename preferred over text edit |
| **O4.3** | MCP client: stdio + Streamable HTTP, 2026-07-28 revision, namespaced, server pinning + description scanning, fail-closed required server | An unrelated third-party MCP server works with zero core edits |
| **O4.4** | Skills/Plugins: `SKILL.md`, Agent Plugins 1.0, vetted install, hash-trusted hooks, managed-only channel, scoped revision + auto-disable `[BEST §3 #10]` | Hostile plugin (exfil tool, over-broad manifest, poisoned hook hash) blocked |
| **O4.5** | Code Mode `rhai` with host budgets, nested permission attribution, no nested Code Mode, concurrency cap 8 | 8-step batch replays with zero extra model turns; every inner call re-checked |
| **O4.6** | OSV install gate inverted to fail-closed (unknown → confirm/deny) | `npx` of a package with a MAL advisory is blocked before install |
| **O4.7** | TTSR full: regex + AST conditions, provenance in receipts, stream rules survive compaction | 40-case injection corpus + 3-step chained attack green |

> **O4 Gate:** third-party MCP works; hostile plugin suite green; Code Mode budgeted; OSV fail-closed; injection corpus green.

---

## Phase O5 — Windows GUI automation (month 4)

| Step | Work | Done when |
|---|---|---|
| **O5.1** | UIA tree digest via `windows-rs` (ControlType, Name, AutomationId, patterns, rect), cached per window, `StructureChangedEvent` invalidation, per-monitor DPI | A legacy .NET app's "Export" button found by a11y tree with zero pixels |
| **O5.2** | Input ladder: app COM scripting > UIA > Windows.Media.Ocr + vision fallback > SendInput last; never elevated | Each rung exercised on a test app; lowest sufficient rung chosen |
| **O5.3** | Post-action verification (a11y delta vs screenshot), abort on mismatch; per-app rules (notepad auto, Settings confirm, regedit never) | Mismatch aborts cleanly; regedit never runs without hard block |
| **O5.4** | Worktree/subagent isolation for parallel desktop tasks is **disabled** — one user's desktop is serial; document the lock `[BLIND §C3]` | Single desktop lock enforced; no two agents actuate simultaneously |
| **O5.5** | Grounding-distrust scoreboard + zoom + a11y cross-check before clicking | Low-confidence click is zoom-verified before commit |
| **O5.6** | 10 daily-driver task eval set | ≥5/10 pass with default approvals; every act verifiable/reversible |

> **O5 Gate:** 5/10 desktop tasks; per-app rules; verification+abort proven; desktop lock enforced.

---

## Phase O6 — Guardian, scheduling & background (month 4–5)

| Step | Work | Done when |
|---|---|---|
| **O6.1** | Durable scheduler: cron/event/file-change triggers, command receipts, retry + dead-letter, one-use HMAC webhook ingress with dedupe/rate-limit | A scheduled job that hits `confirm` skips + notifies; no duplicate runs |
| **O6.2** | Guardian auto-review (separate reviewer, ordered decisions, policy-revision-scoped scores, 3-of-50 circuit breaker) — **advisory**, with Z3 + two-key for irreversible `[BEST §3 #6]` | Classifier `unavailable` falls back to deterministic rules; two-key required for disbursement |
| **O6.3** | Background jobs + notify-on-finish; sessions survive GPUI window close and reattach | Close window mid-job → reopen → converge |
| **O6.4** | Morning-brief loop (calendar/file/mail *draft only*) + presence-awareness (skip when locked) | 2-week unattended simulation, zero destructive actions, zero missed briefs |
| **O6.5** | Event-driven subscription agents draft-only (PR green / file change / webhook) | Trigger produces a draft + receipt, never an auto-send |

> **O6 Gate:** unattended loop with zero destructive actions; guardian advisory; two-key verified; jobs survive restart.

---

## Phase O7 — Hardening & release (continuous, formalized month 5)

| Step | Work | Done when |
|---|---|---|
| **O7.1** | TUF + Sigstore + SLSA updates, SBOM, verify-before-apply, blue-green active/trial service with DB backup + rollback + migration preflight `[ADR-011]` | Tampered/rolled-back update rejected; rollback drill restores DB |
| **O7.2** | Key lifecycle: Ed25519 AID keys in DPAPI/TPM, JWKS rotate/revoke, cryptoperiods, secret rotation + one-use refs | Rotated key invalidates old sig; revoked secret cannot be used |
| **O7.3** | Offline/air-gapped profile: local model option, signed offline bundle, zero outbound spans proof | Full task works with network disabled; trace shows zero outbound calls |
| **O7.4** | Resource envelope: caps on transcript bytes, checkpoints, sidecar RSS; GC/archive with user-visible dashboard; low-resource mode | 30-day soak shows bounded disk/RSS |
| **O7.5** | Eval CI: benchmark version pin (OSWorld v2.1 etc.), contamination audit, Pass^k + CIs, property/fuzz/cut-point replay, response cache | No borrowed savings number; releases gated on stable replay suite |
| **O7.6** | Latency budgets p95 (permission ≤200ms, GUI act ≤500ms), nightly perf gate | Regression >10% fails CI |

> **O7 Gate:** signed update + rollback + key rotation + restore + offline + a11y all green.

---

## Phase O8 — Productization (month 6, minimum viable)

| Step | Work | Done when |
|---|---|---|
| **O8.1** | Distribution: winget + scoop manifests + signed `.exe/.msi`, CI manifest PRs on release | Clean Win11 machine installs from winget and runs a task |
| **O8.2** | `LICENSE` Apache-2.0 + `ee/` carve-out + `LICENSING.md`, DCO, `CONTRIBUTING.md`, `OWNERS.md` | External contributor can ship a skill through the vetted pipeline |
| **O8.3** | Pricing experiment (Free BYOK / Pro ~$12 / Team ~$20) + support tiers + "why did you do that" export | Price page A/B live; audit-chain export reconstructs a task |
| **O8.4** | Public supported-model matrix + deprecation policy + changelog with wire-verifiable telemetry opt-out | No silent model swap or telemetry change ships |
| **O8.5** | Hermes-derived trust artifacts: load-bearing `SECURITY.md` (OS is the only boundary; in-process controls are heuristics), `optimal hermes migrate --dry-run` importer for skills/memories/allowlists, governance + CoC + response SLA, single signed distribution path `[ADR-022]` | Auditor can read what our controls do **not** do; importer brings a skill into the vetted pipeline with a receipt; no look-alike install path |

> **O8 Gate:** installable, licensed, governable, explainable, honestly priced.

---

## Hermes-derived refinements (2026-11-18)

From `../docs/AGENT-RESEARCH-09-HERMES.md` (264k★ local-first peer). These refine existing steps; IDs stay stable.

| Step | Refinement adopted | Hermes evidence |
|---|---|---|
| **O0.1** | Refuse reparse points in tree walks; detect elevation at startup; verify expected owner before Windows ACL grants | `hermes_dir_acl.py:57-129,328-381` |
| **O0.2** | Probes: hard timeout + dedupe + 30s TTL + 60s last-good flapping grace; built-in tools not plugin-overridable without opt-in | `tools/registry.py:260-289,368-445` |
| **O0.4** | Unconditional floors before any bypass; user `deny` never bypassable; block deletion of the runtime's own interpreter/venv | `tools/approval.py:1299-1369` |
| **O0.5** | Force-redaction at the persistence sink independent of the user toggle | `agent/conversation_loop.py:1879-1917` |
| **O0.6** | WS upgrade validates Host/Origin (CVE-2026-53869 DNS-rebinding class); loopback bind; fail-closed auth before PTY/event surfaces | 09B D1 |
| **O1.3** | Normalize consent vocabulary **before** allow-list check; every affirmative alias collapses to `once`; denial text carries explicit anti-retry / anti-route-around instruction to the model | `tools/approval.py:878-978` |
| **O1.4** | One declarative policy object for destinations + **L7 egress + inference routing**, hot-reloadable; half-config refuses sandbox start; proxy-control env protected; WS Host/Origin validation | `docker_egress.py:20-136,214-275`; 09C §3B |
| **O1.5** | Execution-target taxonomy (`local|Docker|SSH|…`, MXC strongest rung) as blast-radius selection; sandbox handles stamped with agent identity, cross-agent reuse refused | `sandbox_scope.py:33-140`; 09C §3A |
| **O1.6** | Exact loop ladder: warn 2 / block 5 identical failures; same-tool 3/8; no-progress 2/5; hard stops default-on unattended; canonical arg hash → stub duplicate results | `agent/tool_guardrails.py:93-195` |
| **O2.4** | Hard caps + session-freeze for injected memory; curator defers one interval on first tick; pinned/cron-referenced skills never touched; keep our re-validation | `agent/curator.py:158-217`; 09C §3D |
| **O4.3** | Apply OSV/scan gate to **catalog sources** too, not just packages; `--force` cannot override a `dangerous` verdict | `skills_guard.py:788-1081`; CVE-2026-82021 (secondary-source) |
| **O4.4** | Stage all skill writes (diff + approve) by default; per-skill lock; hash + source + scanner version in a lock file; write-origin provenance; invert Hermes' `write_approval: false` | 09C §3C, 09A §4.12 |
| **O4.5** | Constant-time RPC token; refusals do not consume budget; strip blocked params before dispatch; log every inner call | `code_execution_rpc.py:85-123` |
| **O6.1** | Scan the **assembled** job prompt (including runtime-loaded skills) at execution time; scheduler toolset denylist that per-job config cannot widen; fail closed on lookup failure; **invert** Hermes' fail-open skew probe | `cron/scheduler.py:167-236,407-476` |
| **O6.2** | Model-as-approver is forbidden; guardian remains advisory; unattended contexts resolve deterministically (deny), never auto-approve | `tools/approval.py:981-1010,1228-1273` |
| **O7.1** | Update receipts with `running|success|partial|failed` and verified-vs-attempted vocabulary; keep our TUF/Sigstore (Hermes verifies structure only) | `update_receipt.py:144-226` |
| **O7.3** | Managed local runtime sizing (SHA-256 verified engine, ≥64K context, ≥4-bit floor, 15-min idle unload) **plus** trace-proven zero outbound | 09C §3F |
| **O8.4** | Split collection/upload consent, whole-period consent windows, pinned non-overridable HTTPS endpoint; no trajectory harvesting | `config_defaults.py:2274-2287` |

---

## Phase dependencies

```
O0 ──► O1 ──► O2 ──► O3 ──► O4 ──► O5 ──► O6 ──► O7 ──► O8
 │       │       │       │       │       │       │
 └── safety/eval/audit/telemetry are cross-cutting from O0 ──┘
```
No capability phase starts until the previous gate passes. `O7` work begins as a cross-cutting track at O1 (signed binaries, caps, telemetry) and is formalized in O7.
