# AI-Harness — Phased Implementation Plan (Build Steps)
### v1.0 — 2026-11-17 (blind-spot integrated)

> **Purpose.** Turn `PLAN.md` (architecture/strategy) and `USE-CASES-EXPANSION-V2.md` (market wedges) into an executable, sequenced build order with discrete steps and hard exit gates.
> **Inputs.** `docs/PLAN.md` (esp. §12 roadmap, §28 corrections, §31.12 Social OS), `docs/USE-CASES-EXPANSION-V2.md` (esp. §3 primitives, §6 backlog), and **102 findings** from `BLIND-SPOTS-01..07.md`, consolidated in `BLIND-SPOTS-INDEX.md`. Steps added from those reports are marked †; the full mapping is in §14.
> **Rule that never changes:** *safety core ships before capabilities.* No new effect adapter is merged without its eval corpus, strictest-sane confirm defaults, and a rollback drill.

---

## 0. How to read this document

| Element | Meaning |
|---|---|
| **P0–P9** | Phase = a milestone large enough to demo end-to-end. Later phases assume earlier exit gates passed. |
| **Sx.y** | Step = one independently reviewable unit of work (a PR or a small batch of PRs). |
| **Done when** | Objective exit criterion. "Works on my machine" is not a criterion. |
| **Gate** | A phase-level go/no-go review. A failed gate blocks all downstream capability work. |
| **†** | Step added from a blind-spot report; see §14 for the mapping. |

**Sequencing invariants**
1. The policy engine, audit chain, and rollback ledger are built **before** any mutating effect adapter.
2. Every tool adopts the uniform registry envelope on day one; runtime is an implementation detail.
3. Every effect ships with: eval corpus → confirm defaults → dry-run → checkpoint → undo drill.
4. Evals are CI-gated from Phase 0; a regression in safety tests blocks releases even at the cost of features.
5. *Boring, verifiable, local* beats autonomy theater — predictability is the product.

---

## 1. Phase map at a glance

| Phase | Theme | Target window | Hard exit gate |
|---|---|---|---|
| **P0** | Skeleton & safety spine | Weeks 1–2 | Multi-step task end-to-end; zero secrets in transcript; kill switch works |
| **P1** | Code agent + policy core | Month 1 | 100+ rule tests; injection corpus v0; rollback drill; no unconfirmed destructive op |
| **P2** | Extensibility, memory, subagents, Social Alpha | Months 2–3 | 3rd-party MCP works; 3 subagents finish; hostile-plugin suite passes; commitments attested |
| **P3** | Browser agent + action cache | Months 3–4 | 5-site research; form-fill post-confirm; decoy injection refused; 40+ injection cases pass |
| **P4** | GUI/OS automation + interface repo | Months 4–5 | 5/10 desktop tasks pass at default approvals; every GUI act verifiable/reversible |
| **P5** | Comms, cloud, scheduler, desktop app | Months 5–6 | Unattended morning brief, zero destructive actions; desktop app is primary UX |
| **P6** | Obligation-custody lanes + telephony | Post-MVP | Clock ledger, chase ledger, two-key gate; consent-receipt call pilot |
| **P7** | Cross-platform & international | Post-MVP | Linux AT-SPI + macOS AXUIElement parity; i18n; non-US jurisdiction profile |
| **P8** | Hardening: observability, PKI, updates | Continuous from P1 | OTel traces cross surfaces; signed updates; key rotation drill |
| **P9** | Productization: GTM, packaging, community | From P5 | Pricing experiment, distribution channels, governance docs, support SLA |

---

## 2. Phase 0 — Skeleton & safety spine (weeks 1–2)

**Goal.** One governed agent loop that can do a multi-step local task, with tamper-evident logging and a working kill switch. No browser, no GUI, no network effects.

### S0.1 Repo & build foundation
- Monorepo layout per `PLAN.md` Appendix A (`packages/core`, `policy`, `tools`, `registry`, `gateway`, `tui`, `cli`; `runners/`; `evals/`; `docs/`).
- Toolchain: Node 22 + TypeScript strict mode; PowerShell 7 + C#/.NET 8 helper skeleton; SQLite (WAL) + JSONL transcripts.
- **Done when:** `pnpm build` green from clean clone; `harness --version` prints; CI runs lint/typecheck/unit on every PR.

### S0.2 Uniform tool registry envelope
- Implement the envelope from §5 for `fs.read`, `fs.write`, `fs.list`, `env.probe`, `shell.exec`.
- Uniform result envelope `{status, stdout, stderr, exit, artifacts[], durationMs, correlationId}`.
- `probe.cmd` capability gating at startup; `tools.search` meta-tool stub.
- **Done when:** registry serializes to the documented JSON schema; a tool with a failed probe is greyed out and unreachable by the model.

### S0.3 Agent loop (PERCEIVE → PLAN → ACT → REFLECT)
- Steerable interrupt queue checked between tool calls; Esc soft interrupt / Ctrl+C hard kill.
- Bounded retries with `timeout_ms`, `retries`, idempotency keys; failure ladder retry → re-plan → escalate.
- Plan-as-file: `plan.json` + `todos.md` pinned during execution.
- **Done when:** a 5-step task ("summarize this folder, then create and refactor a file") completes with visible plan/todos; kill switch mid-task leaves completed effects logged and in-flight calls stopped cleanly.

### S0.4 Policy spine & audit chain
- Single decision pipeline: `tool call → canonicalize → rule engine → tier → sandbox → execute → hash-chained audit log`.
- Tiers `readonly | auto | confirm | never`; default = `never` for unknown effects.
- Secret redaction regex pass **before** persistence; vault `{{vault:ref}}` placeholders substituted only inside executor.
- **Done when:** a injected fake secret in tool output never appears in `events.jsonl`; audit chain verifies with its hash chain; changing a rule changes the tier in a table test.

### S0.5 Minimal CLI/TUI + config
- CLI one-shots; TUI streaming tokens + tool badges + permission prompt + budget meter; `policy.toml` config.
- **Done when:** a human can complete S0.3's task without reading source; confirm prompt shows exact command/paths + matched rule + tier.

### S0.6 Eval harness v0
- `evals/` skeleton: rule tables, task fixtures, transcript scrubber, canary files.
- **Done when:** CI fails on (a) leaked secret, (b) canonicalization regression, (c) canary file touched.

### Blind-spot additions (†)
- **S0.7 Protocol & schema contracts** — version JSON-RPC method/event names (`harness.v1.session.spawn`), handshake with supported range, `capabilities` object, additive-only within a major version, one-version deprecation policy; version the tool envelope and JSONL transcripts. *(ENG #6)* **Done when:** an v0 SDK client drives the v1 gateway without code changes.
- **S0.8 Training Wheels first-run + Capability Card** — 60s capability-boundary tour, one guided low-stakes task with annotated confirm, staged surface unlock, reset-trust affordance; plain-language can/cannot/will-ask-first card generated from §4. *(HUMAN #3#4)* **Done when:** a first-time user completes a task and can state three things the agent will never do.
- **S0.9 Meaningful Oversight Test** — add a `reviewability` field to every rule; route to a human only when error cost is high **and** the confirm card lets the reviewer independently judge correctness; otherwise use a deterministic guard. *(HUMAN #2)* **Done when:** a human cannot be asked to approve an action whose correctness is not reviewable from the card.

> **P0 Gate:** end-to-end multi-step task demonstrated; zero secrets in transcript; kill switch verified from TUI and CLI; audit chain tamper test fails as expected; protocol handshake and reviewability test in place.

---

## 3. Phase 1 — Code agent + safety core (month 1)

**Goal.** A trustworthy coding agent: git-aware, test-driven, with the full rule engine and rollback.

### S1.1 Git integration & checkpoints
- `status --porcelain=v2 -z`, diff, commit parsing; `refs/harness/checkpoints/*` hidden refs (T3 pattern).
- Checkpoint before every mutating batch; **"can't roll back → must reject before touching filesystem"** (§19 adoption).
- **Done when:** delete-a-file → restore-from-checkpoint drill passes with a clean working tree.

### S1.2 Full rule engine & canonicalization
- Resolve executables against PATH before matching (blocks `.cmd`/`.bat` hijack); normalize case, slashes, 8.3 names, UNC, symlinks/junctions.
- Split compound commands (`&&`, `|`, redirects) into sub-calls; pipelines/redirects escalate to `confirm` by default.
- Rule resolution chain: global → project → session → interactive; first-match-wins.
- **Done when:** 100+ table tests pass incl. PowerShell tricks (`Invoke-WebRequest | iex`, download cradles, path hijacks).

### S1.3 Tiered permission UX
- Confirm prompt shows unified diff for edits, URL + screenshot preview for browser actions, recipient for sends, matched rule + tier; buttons approve / always-in-project / reject / edit.
- Deny-by-default in unattended mode, timeout = deny.
- **Done when:** every destructive op in the eval set required a confirm; no `never` rule was bypassable from any surface.

### S1.4 Budgets, rate limits, egress DLP
- Per-task token ceiling, per-day $ ceiling, max tool calls/min, max bytes/write; breach → pause + ask.
- Deny-by-default network egress; hard-deny private ranges & cloud metadata (169.254.169.254); build-time destination allowlist.
- **Done when:** a task that exceeds budget pauses instead of degrading silently; SSRF test to metadata endpoint is blocked and logged.

### S1.5 Agent-core upgrades from §22.4 (P1 slice)
- **Narration-blind finish gate:** clean context (task + filesystem only), re-verify artifacts, ≤3 repair rounds.
- **Stuck + milestone monitors:** cheap classifier escalates small→frontier only at stall/milestone.
- **Three-tier loop breaker:** same action ×2 no-change → modality switch; same screen ×3 → strategy switch; external judge → blacklist.
- **Done when:** a deliberately looping task is broken out of within 3 repetitions; finish gate catches a falsely-claimed success.

### S1.6 Session resume, secret storage, backups
- SQLite sessions + JSONL replay; DPAPI-encrypted vault; `.harness/.bak` per file (24h retention); global undo ledger.
- **Done when:** close TUI mid-task → reattach → converge; secrets never appear in export.

### Blind-spot additions (†)
- **S1.7 Human oversight & consent layer** — Reliability Calibration (confidence + known-failure tags, reliance-accuracy metric, periodic probes); Consent Ledger (scope/TTL/delegation depth/revoke + signed consent receipts); fatigue KPIs (approval rate, median time-to-approve, <1s rubber-stamp rate) with batching at threshold; 4-state ambient cue channel (idle/progressing/needs-you/stuck, non-color coded); graduated handover with Takeover Time Budget + pre-staged context packet; counterfactual cards + Contest action; risk-triggered cognitive forcing for irreversible ops; coercion-aware confirms + duress mode for senior profile. *(HUMAN #1#5#6#7#8#9#13; TEMPORAL #11)* **Done when:** rubber-stamp approvals are measurable and trigger batching; a consent grant expires on schedule; a takeover includes a context packet and meets its time budget.
- **S1.8 Real security boundary** — demote Reader/Actor to a utility filter and make **capability isolation** the boundary (planner/reader split, capability labels, destination allowlist); SMT/temporal-logic monitor over the hash-chained trace + Z3 validation of `policy.toml` at load; add gVisor (L3.5) and Firecracker/Kata ≥4.3.0 (L5 microVM) tiers with no long-lived credentials inside and egress deny-by-default inside the VM; implement guard evolution as propose → Z3/conflict verify → evidence-quorum + two-key activation (model never edits rules directly); add "agent cannot rewrite its own tooling/crew" guard. *(SAFETY B-01#02#03#07; TEMPORAL #9)* **Done when:** an adaptive-attack corpus (GCG, phantom continuation, architecture-aware) fails to cross the capability boundary; a multi-step standing obligation is enforced; a policy downgrade requires two keys.
- **S1.9 Eval integrity** — pin benchmark versions (OSWorld v2.1, Terminal-Bench 4.x); adversarial audit of our own eval harness (gold answers outside agent-accessible FS, network-off default, grader integrity tests); 5-field disclosure schema; report Pass^k with confidence intervals, not single-run ASR. *(SAFETY B-09; TEMPORAL #3#4)* **Done when:** a BenchJack-style planted conftest cannot inflate our scores; all reported numbers carry benchmark version + harness disclosure.
- **S1.10 Calibrated autonomy (P1 slice)** — conformal/LTT deferral thresholds per task family; optional local activation probe as a *soft* signal with explicit false-positive budget and sequential accumulation (never a sole hard gate). *(SAFETY B-04#05)* **Done when:** deferral meets a stated risk target at stated coverage; probe score enters the receipt structure.
- **S1.11 Model-role routing & fallback chains** — operationalize `PLAN.md` §14 with nine roles (`default|smol|slow|plan|commit|vision|task|advisor|tiny`), per-role/per-model fallback chains on 429/quota with cooldown restore, path-scoped `enabledModels`/`disabledProviders`, round-robin credentials with session affinity and per-key backoff. Show model/effort to the user, publish a supported-model matrix, honour team allowlists. Distinct from S3.7 (provider failover mid-task) and S8.9 (benchmark CI). *(OMP research — gap with no prior `Sx.y`; Dots multi-model validation)* **Done when:** a 429 fails over to the fallback model within the cooldown, per-role cost is visible, and no model swap is silent.

> **P1 Gate:** 100+ rule tests green; injection corpus v0 + adaptive corpus green; rollback drill verified; every destructive op confirmed; egress DLP default-deny verified; capability boundary and temporal monitor enforced; eval harness contamination audit passed.

---

## 4. Phase 2 — Extensibility, memory, subagents, Social Alpha (months 2–3)

**Goal.** Plugins, MCP, subagents with narrow allowlists, inspectable memory — and the commitment/receipt substrate for multi-agent work.

### S2.1 MCP client (dual transport)
- MCP `2026-07-28`: stdio + Streamable HTTP; stateless requests, MRTR, CIMD auth, header routing. **Do not build on deprecated Sampling/Roots/Logging/SSE.**
- Handle `resultType: "task"` + `input_required` as scheduler primitives; namespaced `mcp/server.tool`.
- **Done when:** an unrelated third-party MCP server works with zero code edits to the harness.

### S2.2 Plugin API + permission manifests
- `plugin.toml` / Agent Plugins 1.0 (`plugin.json` + `skills/` + `mcp.json`); plugins can only claim declared permissions; over-broad claims denied.
- Server pinning + tool-description scanning for hostile MCP tools (Deadbugz/Postmark/RufRoot defenses).
- **Done when:** hostile-plugin suite (exfiltrating tool, over-broad manifest, poisoned description) passes.
- † **P8.i** (PKI) begins here: signing keys for plugin manifests.

### S2.3 Subagent orchestration
- `task` tool: isolated context, narrowed allowlist, final report + artifact paths only; attributable elevation requests.
- Orchestrator–worker default; async mailbox (deliverable DAG), not peer chat; global concurrency cap; per-scope locks.
- **Done when:** 3 subagents finish a parallel task; a subagent's permission prompt is attributed to it in the TUI.

### S2.4 Memory tiers
- Working context compaction at 75% with artifact flush + rolling session brief + checkpoint.
- File-first project memory (`AGENTS.md` ≤30 lines, `todos.md`, `plan.json`, `env.md`); long-term store with TTL + staleness class + re-validation (Momento lesson).
- **Done when:** compaction round-trips with no loss; a stale recalled fact is re-checked against current state before acting.

### S2.5 T3-style durable state
- Event-sourced orchestrator + transactional outbox + command receipts (skip T3's 12.7k-line single file); bounded stream budgets; one-use HMAC secret refs.
- **Done when:** kill mid-command → restart → command receipt prevents duplicate effect.

### S2.6 Social Alpha (P2.5 — M0–M5)
Per `SOCIAL-ALPHA.md` (referenced in PLAN.md §31.12) and §31:
- **M0** Identity: `agent.toml` social extension (AID record, RFC 9421 endpoint proof, owner-tethering, model pin).
- **M1** Commitment Ledger: promises as protocol objects with criteria/deadline/budget/deps/attestors; deterministic reputation = f(ledger), never f(LLM opinion).
- **M2** Receipts: every inter-agent claim `{tool ref, artifact hash, verifier verdict, TTL}`; unreceipted claims may not be built upon.
- **M3** Standup-as-file: fixed-shape blocks; minutes commit to `plan.json`/`todos.md`; 4 small-model calls per standup; freeform chat denied by default.
- **M4** N-of-M dual control for irreversible ops via MCP MRTR `input_required` / A2A `TASK_STATE_AUTH_REQUIRED`.
- **M5** TeamWork-Bench v0: commitment-kept rate, handoff survival, deception-propagation, $/outcome.
- **Done when:** two teammates hand off a task with receipts; a missed promise is renegotiated before deadline and reputation updates deterministically; verifier rejects an unreceipted claim.

### S2.7 VS Code extension
- WorkspaceEdit diffs with accept/reject blocks; `languages.getDiagnostics`; Shell Integration API for terminal output; `vscode://file/path:line:col`.
- **Done when:** an edit round-trips accept/reject and diagnostics appear in the agent loop within one turn.

### Blind-spot additions (†)
- **S2.8 Telemetry** — OTel GenAI spans (`invoke_agent`, `chat`, `execute_tool`, `plan`, `retrieval`, `memory`) with a pinned internal adapter layer; W3C `traceparent` in the JSON-RPC `correlationId`; stable internal metric names mapped to unstable `gen_ai.*`; content capture stays OFF. *(ENG #1; SAFETY B-06)* **Done when:** one task traced end-to-end across TUI → gateway → subagent → tool with correlated IDs.
- **S2.9 Durability contracts** — event schema (`seq, stream_id, type, payload, causation_id, correlation_id, ts`), one transaction for event+projection+outbox, singleton EffectWorker + `command_receipts` unique index, projection-rebuild command + checksum test, snapshots, dead-letter poison events; `schema_migrations` with expand→migrate→contract, fixtures from last 3 versions, backup before destructive ALTER. *(ENG #4#5#6)* **Done when:** a projection rebuild matches the checksum; a v-1 DB migrates forward and back without data loss.
- **S2.10 Concurrency** — sessions/subagents as actors with mailboxes; one actor owns each mutable scope; cross-surface lock service with lease + fencing tokens and stale-lease reclaim; ordered acquisition, global wait graph. *(ENG #7)* **Done when:** two surfaces editing the same `plan.json` serialize without lost updates or deadlock.
- **S2.11 Testing infrastructure** — property suite over policy + envelope (authorization, idempotency, fail-closed, "no effect without receipt"), grammar-based fuzz of canonicalizers, record/replay with cut-point CI replay, deterministic simulation with seeded clocks/randomness + fault injection. *(ENG #8)* **Done when:** 10k fuzzed permission matrices pass; a recorded incident replays bit-stable in CI with zero model calls.
- **S2.12 Identity & IAM lifecycle** — `KEY-MANAGEMENT.md`: Ed25519 keys in DPAPI/TPM, JWKS at `/.well-known/http-message-signatures-directory`, `kid` = JWK thumbprint, overlap rotation, signed revocation list + short cache TTL (standards have no revocation); ANS-compatible DNS records + public-log receipt alongside AID; SPIFFE-style workload identity bound to code/model hash, RFC 8693 token exchange with scope narrowing per hop, DPoP-bound short-lived tokens, zero standing privileges. *(ENG #2; SAFETY B-12; TEMPORAL #6)* **Done when:** a rotated key invalidates old signatures within the cache TTL; a delegated token's scope narrows at each hop.
- **S2.13 RAG retrieval security** — tag chunks `untrusted-data` at ingest, Reader/Actor separation at the retrieval boundary, Unicode/HTML normalization, reliability/rank + decision gate before chunks enter context, traceback + quarantine on confirmed poison, no retrieved text reaches a send/write tool without a receipt. *(SAFETY B-08)* **Done when:** a single poisoned document with a 10-token trigger cannot cause an exfil path.
- **S2.14 Guard/provenance evolution** — three-phase policy mutation propose→verify→activate; sign + verify model artifacts and bind each receipt to the exact tool-call/response hash (never free text); quarantine on mismatch. *(SAFETY B-03#14)* **Done when:** a tampered provenance field invalidates all downstream receipts.

> **P2 Gate:** third-party MCP works; 3-subagent parallel task done; hostile-plugin suite green; Social Alpha M0–M5 demo passes; event-sourced restart has no duplicate effects; traces correlate; identity rotation + RAG poisoning tests green.

---

## 5. Phase 3 — Browser agent + action cache (months 3–4)

**Goal.** Operate the user's real logged-in web life, with injection defense stronger than the state of the art.

### S3.1 Playwright + persistent profile
- Headed persistent Chrome profile, `channel:'chrome'`; CDP attach; MV3 extension bridge via Native Messaging (profile-copy fallback for Chrome ≥136).
- `ariaSnapshot()` as structured page-state feed; click by `ref`, not pixels; downloads to harness inbox.
- **Done when:** agent stays logged in across restarts and can act on an SSO site under confirm.

### S3.2 Reader/Actor split
- Small cheap Reader (zero mutating tools) digests page → structured facts + DOM-node citations; Actor sees only digest + goal, never raw HTML/DOM.
- Hidden/zero-contrast/overlay text rendered as `[hidden: …]`; hostile-data signal flags; sensitivity tiering (type=auto on allowlist, purchase/send/pay/login=always confirm, downloads=confirm+scan).
- Reputation gate: history allowlist, denylist, phishing feeds, lookalike detection (`paypa1.com`).
- **Done when:** decoy page with hidden instructions is refused; denylist honored; 40+ injection corpus cases pass in CI.

### S3.3 Action cache (Stagehand pattern)
- observe once → cache selector → deterministic replay → LLM only on miss; teach-and-repeat capture under confirm tier with step-verified replay.
- **Done when:** a recorded 8-step form fill replays 10× with zero LLM calls and zero drift.

### S3.4 Sequence-aware injection defense
- Multi-turn page-chained attack detection (StepJack 41.7→72.9% at 3-step depth): track hostile signals across the session, not per page.
- **Done when:** a 3-step chained injection corpus is refused where a per-page checker would fail.

### S3.5 P3 use-case slices (from §25/§31 of the expansion doc)
- Subscription sweep (cancel-and-evidence with screenshots), car/major-purchase negotiation, travel dispute/refund tracking, consular slot sentinel (confirm-gated, polite single attempt), utility/IRA rebate radar.
- Long-horizon scheduler emphasis: re-check deadlines over weeks/months, escalate with citations, archive proof.
- **Done when:** a 4-week simulated dispute timeline is tracked with no missed window and every action receipted.

### Blind-spot additions (†)
- **S3.6 Manipulation resistance** — Reader digest emits a manipulation signal (pre-ticked consent, false urgency, confirmshaming); never accept a site's dark-pattern consent on the user's behalf; Brignull-taxonomy self-audit of our own prompts in CI. *(HUMAN #11)* **Done when:** a decoy consent flow is surfaced as hostile data and routed to the human.
- **S3.7 Provider-agnostic model supply** — hot failover across model adapters, published supported-model matrix, no critical feature bound to one provider's hosted model (canonical case: OpenAI→Cursor shutoff 11/12/2026). *(TEMPORAL #14)* **Done when:** killing the primary provider fails over mid-task without losing state.

> **P3 Gate:** 5-site research task; form fill post-confirm; decoy injection refused; denylist honored; 40+ injection cases + adaptive corpus green; action cache replay verified; model failover proven.

---

## 6. Phase 4 — GUI/OS automation + interface repository (months 4–5)

**Goal.** Drive the real Windows desktop via a11y-first observation with vision fallback, verified after every terminal action.

### S4.1 UIA observation plane
- Windows UIA tree digest per step (ControlType, Name, AutomationId, patterns, rect), cached per window, invalidated on `StructureChangedEvent`; per-monitor DPI awareness at startup.
- OCR: Windows.Media.Ocr first, PaddleOCR/RapidOCR fallback.
- **Done when:** a legacy .NET app's "Export" button is found by a11y tree with zero pixels used.

### S4.2 Input ladder & post-action verification
- app-native scripting (COM) > UIA > OCR+vision > SendInput last resort; never run agent elevated — gsudo per-verb.
- Post-action expectation check (TVAE-style): a11y delta vs screenshot; abort on mismatch.
- **Done when:** 5/10 daily-driver desktop tasks pass at default approvals; every GUI act verifiable and reversible.

### S4.3 Interface repository (Agent S4 thesis)
- Seed verified adapters for local apps (Office COM, LibreOffice UNO, browser sessions) before generic GUI; repo-relative adapters beat model-swapping ~3.8×.
- **Done when:** a seeded adapter handles a task that pure vision fails; held-out task regression green.

### S4.4 GUI safety & small grounding models
- Per-app rules (notepad auto, Settings confirm, regedit never); grounding-distrust scoreboard + uncertainty-driven zoom + a11y cross-check; local Holo3.1/Fara-class grounding where available.
- Visual-history folding (keep 20 recent screenshots, fold older in blocks of 10, never mutate prefix → KV-cache reuse); latency gates not token gates for interactive steps.
- **Done when:** low-confidence clicks are zoom-verified before committing; latency budget met for interactive steps.

### S4.5 P4 use-case slices
- ERP/AP re-keyer (NetSuite/SAP GUI), Sabre/green-screen copilot via keystroke→macro recorder (confirm-before-submit), funeral-doc chain, kennel vet-cert chase, screen-reader agent hybrid (SR-native gists, staged confirm).
- **Done when:** a recorded legacy-GUI macro replays with parameterization and a confirm gate before submit.

### Blind-spot additions (†)
- **S4.6 Latency budget engineering** — p95/p99 targets per interaction class (permission decision ≤200ms, GUI act ≤500ms to early decode, tool dispatch <20ms); nightly pinned-model perf job recording p50/p95/p99 + token/cost per step; fail on >10% regression. *(ENG #11)* **Done when:** latency budget met across 10 desktop tasks with a regression gate in CI.

> **P4 Gate:** 5/10 desktop tasks pass; per-app rules enforced; post-action verification + abort proven; interface repo seeded; latency budget met with CI regression gate.

---

## 7. Phase 5 — Comms, cloud, scheduler, desktop app (months 5–6)

**Goal.** Unattended-but-safe follow-through and a primary desktop UX.

### S5.1 Communication agents
- Graph API / IMAP; thread-state model; tone-mirrored drafting; **external_send never auto-run** — dry-run preview of exact message + confirm.
- Free/busy conflict solver; chase/escalate durable loops.
- **Done when:** an email draft requires confirm before send; canary recipient never receives a test message.

### S5.2 Cloud/DevOps
- `gh`, aws, az scoped CLIs; plan-review gate for Terraform/Pulumi; repo/org-gated rules; Docker Desktop detect-never-autostart; WSL2 path translation.
- **Done when:** a deploy plan is reviewed before apply; a rollback is rehearsed in eval.

### S5.3 Durable scheduler engine
- Event-sourced job store + outbox + command receipts; triggers cron/event/webhook/file-change/price; retry + dead-letter; idempotency keys.
- HMAC webhooks + dedupe + rate-limit + header redaction; scheduled jobs skip on any confirm hit; unattended mode = strictest profile.
- **Done when:** a scheduled job that hits a confirm tier skips + notifies instead of denying-then-continuing.

### S5.4 Tauri desktop app + tray + overlay
- System tray (status/budget, quick-capture, notification center, pause/resume), global-hotkey palette, screen-region capture, voice bar stub.
- Cross-cutting surfaces built once: streaming, task tree, permission prompts, steering, background notifications, session resume/share, scheduled-jobs UI, cost transparency.
- **Done when:** desktop app is the primary daily driver; tray pause/resume and deep-link "open in TUI" work.

### S5.5 Morning-brief loop
- Calendar/mail/weather/tasks digest; passive presence-awareness (skip if workstation locked).
- **Done when:** 2-week unattended simulation with zero destructive actions and zero missed briefs.

### Blind-spot additions (†)
- **S5.6 Global payment rails + substitute credentials** — `PaymentRail` adapter interface (identifier type, precision, irrevocability, dispute semantics, confirm tier, regulatory flags) with UPI, Pix, SEPA Instant/FedNow/Faster Payments, M-Pesa/USSD, and x402 as one adapter; one-shot merchant-scoped tokens minted by the DPAPI vault, Windows Hello/biometric confirm; value never enters model context. *(INTL #4#12; TEMPORAL #7; HUMAN — capability isolation)* **Done when:** a one-shot token pays a test merchant and cannot be replayed; no card value appears in transcript.

> **P5 Gate:** morning brief runs unattended with zero destructive actions; desktop app primary UX; all external sends confirmed; one-shot payment token cannot be replayed.

---

## 8. Phase 6 — Obligation custody + telephony (post-MVP)

**Goal.** Productize the surviving wedge from `USE-CASES-EXPANSION-V2.md §0`: *persistence + clocks + receipts*, not drafting.

### S6.1 Clock Ledger / ObligationRegistry / Deadline Ladder
- Ingest governing text (CBA, permits, plan PDFs) → derive personal countdown clocks with source citations; re-derive when facts change.
- **Done when:** a 6-month simulated deadline timeline fires every escalation with a citation and a receipt.

### S6.2 Relay Baton + Chase Ledger + Evidence Locket
- First-class document-hop object with per-hop SLA/delivery receipt/escalation; per-counterparty chase ledger; hash-receipted evidence binder (photos/GPS/texts/screens) exportable as a case pack.
- **Done when:** a wage-claim/appeal case pack is assembled end-to-end from evidence to escalation.

### S6.3 WindowGraph + Shot-Clock Executor + Release-Map Inference
- Event-conditional statutory window graph; pre-staged forms, single-tap confirm, auto-release; infer portal release schedules into `release.ics`; ONE armed confirm-gated attempt (politeness as a primitive).
- **Done when:** a release-window attempt fires once, confirms, and never spams.

### S6.4 Telephony surface †
- Softphone/SIP + IVR navigation + hold-forest (transcript-on-human-answer, warm bridge); consent-receipt pairing + LiveMic bridge; spoken disclosure, hash-receipted provenance, never clone the user's voice.
- Compliance codified as a **skill**, not guidance: FCC TCPA "artificial voice", NPRM 24-84, all-party-consent states, CA B.O.T./Utah AIPA/CO/TX.
- **Done when:** a disclosed automated-assistant call is placed with a receipt; a hold call bridges to the human on answer.

### S6.5 Multi-tenancy + fiduciary two-key
- Household tenancy spec (roles, delegation graphs, expiry); rules minted by the person they govern ("with, not about"); Fiduciary Two-Key Gate for disbursements/filings (intent vs governing doc + second verified human).
- **Done when:** a two-key disbursement fails with one key and succeeds with two, both receipted.

### S6.6 Registry watchers + Clause→Cron
- Nightly registry diff (IRS TEOS, CMS, NHTSA VIN, state boards); compile signed agreements (commission TOS, sponsor brief) into watch trees with receipted firings.
- **Done when:** a simulated status flip produces a diff alarm within 24h.

### S6.7 P6 vertical slices (top of the expansion shortlist)
- Hold-forest + phone status racing, consular slot sentinel, executor/probate + trustee console, QDRO chase, PCA/EVV family-employer ops, recall-remedy claim runner, wallet-loss reissue, Form 990 watchdog, SBA/disaster appeal, IEP clock steward, seat-war, vehicle recall + days-out-of-service.
- **Done when:** 3 lanes run for 4 weeks in simulation with zero missed clocks and full receipts.

### Blind-spot additions (†)
- **S6.8 International rule-change sources** — add EUR-Lex/OJ, EC Digital Strategy, EDPB, ICO, MSIT Korea, NIA, PIB India, NPCI, BCB, ITU/ISO feeds to the Registry-Diff Tripwire with effective-date + jurisdiction metadata feeding Clause→Cron. *(INTL #13)*
- **S6.9 Age & household boundaries** — Age-Signal Abstraction Layer (zero-knowledge minor/adult attestation, no DOB stored; unknown = minor-safe profile); Household Boundary Map (per-profile DPAPI encryption, explicit reachable scope, guest/child profiles with no cross-read, no cloud sync by default, hard refusal to act covertly). *(HUMAN #12#14)* **Done when:** an unknown-age profile cannot write external comms; one household profile cannot read another's files.
- **S6.10 Regulated-SMB vertical pack** — 15 ClockBook + receipts slices: pilot medical/currency, FFL, FMCSA, mechanics liens, OSHA logs, CPA notice clocks, FINRA U4, insurance commission reconciliation, FDA QMSR, Amazon/FBA seller, I-9/E-Verify, PSLF/IDR, RCRA, youth-sports credentials, hotel OTA recon. *(VERTICALS #1–15)* **Done when:** at least 3 lanes pass a 4-week simulated clock timeline with zero missed windows.

> **P6 Gate:** a 4-week multi-lane simulation with zero missed statutory windows; two-key gate verified; telephony disclosure receipts verified; age + household boundaries enforced.

---

## 9. Phase 7 — Cross-platform & international †

### S7.1 Linux desktop
- systemd user unit; Wayland AT-SPI2/D-Bus tree reads + ydotool/wtype input; X11 xdotool fallback; compositor matrix test.
- **Done when:** 5/10 desktop tasks pass on Ubuntu/Fedora/Arch.

### S7.2 macOS
- launchd LaunchAgent; AXUIElement a11y tree (proves a11y-first portability); JXA/AppleScript bridges; TCC consent mapping; screen capture.
- **Done when:** 5/10 desktop tasks pass on Apple Silicon with TCC prompts mapped to the consent model.

### S7.3 Internationalization & jurisdiction profiles
- Locale/timezone/calendar/i18n architecture; non-US identity/benefit/health/tax systems (EU/UK/India/Brazil/Africa); global payment rails; cross-border data transfer posture.
- Jurisdiction profile: EU AI Act, GDPR/Data Act/DSA, ISO 42001, NIST AI RMF mapping to our controls.
- **Done when:** a non-US use case runs end-to-end under its jurisdiction profile; UI is fully localized.

### S7.4 Edge/Fleet/Pocket SKUs
- Edge edition (Pi/Jetson gateway + Home Assistant MCP bridge), Fleet edition (SSH/k8s/docker targets), Pocket edition (Android companion).
- **Done when:** one core binary serves three target packs with different capability manifests.

### Blind-spot additions (†)
- **S7.5 Jurisdiction/regime plugins** — `Regime` interface with rule sets for EU AI Act (provider/deployer/importer classifier, risk-class map with **Omnibus-shifted dates as data**, Art. 50 disclosure), GDPR Art. 22 / UK DUAA Art. 22A–22D ADM register + meaningful-human-involvement artifact + DPIA/FRIA, EU Data Act/DSA, EU KIDS Act, APAC regimes (Korea/Japan/China), cross-border transfer gates + model-region constraints, "no transfer" mode. *(INTL #1#2#3#7#10; TEMPORAL #5; HUMAN #12)* **Done when:** the same task under two regimes produces different, correct disclosure and routing decisions.
- **S7.6 Global identity & rails** — India Stack (DigiLocker fetch/verify, Account Aggregator consent object), EUDI/eIDAS 2.0 wallet + qualified signatures, Japan My Number, Brazil Gov.br; M-Pesa/USSD and Pix-key adapters; per-country arrival/benefit/tax packs. *(INTL #5#11#12)*
- **S7.7 Locale & time** — ICU MessageFormat, UI locale vs model locale separation, RTL/CJK/Cyrillic/Arabic corpora, semantic anchors instead of literal English labels, non-Latin OCR; IANA tzdb with DST gap/overlap policy, RFC 3339 + local-wall-clock dual representation, calendar-family registry (wareki/Hijri/Hebrew/Buddhist/Nepali); Clock Ledger and Clause→Cron consume `core.time`. *(INTL #8#9; ENG #12)* **Done when:** a filing deadline computed across a DST transition is correct in three locales.
- **S7.8 Standards mapping & compliance evidence** — `controls-mapping.yaml`: each control → ISO 27001 / ISO 42001 (AIMS) / NIST AI RMF GOVERN-MAP-MEASURE-MANAGE / EU AI Act Art. 9/15/55; audit log as technical-documentation spine; scheduled red-team releases with retest-after-fix; track CEN/CENELEC M/613 drafts. *(INTL #6; SAFETY B-10)*
- **S7.9 Protocol deltas & impact telemetry** — pin A2A v1.0.1, track v1.1 RC streamed messages as untrusted data; AP2 v0.2 mandates + x402 V2 seam; energy/CO₂e/water telemetry beside `$`, reasoning-trace/output token split, grid-aware deferral. *(SAFETY B-11#13)*

> **P7 Gate:** a11y-first thesis proven on 3 OSes; a jurisdiction/regime test suite passes; i18n + DST deadline tests pass; 3 SKUs share one permission engine.

---

## 10. Phase 8 — Hardening (continuous from P1, formalized here) †

### S8.1 Observability
- OpenTelemetry GenAI semantic conventions; spans across agent loop → tool → effect → audit; cross-surface trace IDs; variance-aware telemetry (stop trusting single-run ASR).
- **Done when:** one task traced end-to-end across TUI + gateway + tool with correlated IDs.

### S8.2 PKI / key lifecycle
- Concrete key management for AID records + RFC 9421 + JWS/RFC 8785 claim manifests: generation, rotation, revocation, HSM/DPAPI storage, detached verifier role.
- **Done when:** a rotated key invalidates old signatures; claim manifest verifies over the hash-chained audit log.

### S8.3 Secure updates & supply chain
- Signed binaries + signed update manifests (Sigstore/cosign, SLSA provenance); rollback-safe updates; dependency pinning; reproducible builds.
- **Done when:** a tampered update is rejected; a clean update verifies provenance.

### S8.4 Offline / air-gapped & recovery
- Local-only mode with no egress; backup/restore verification drill; crash-only recovery; resource-constrained profile.
- **Done when:** full MVP works offline; restore drill passes byte-for-byte.

### S8.5 Accessibility of the harness's own UI
- WCAG 2.2 AA on TUI/web/desktop; screen-reader-native gists; keyboard-only path; ≥18pt option for senior profile.
- **Done when:** an NVDA/JAWS user completes a confirm flow unaided.

### Blind-spot additions (†)
- **S8.6 Key & secret lifecycle** — full cryptoperiod/policy for AID and release keys; short-lived one-use secret references (`vault://name?version=3&rotates=30d&one_shot=true`), revoke-on-compromise invalidating cached subprocess copies, DPAPI re-encryption on key rotation, rotation audit events. *(ENG #2#9)*
- **S8.7 Secure updates & supply chain** — TUF v1.0.37 roles (offline Root threshold ≥2, Targets, Snapshot, online Timestamp) for rollback/freeze/mix-and-match defense; Sigstore keyless (Fulcio + Rekor, cosign) builds; SLSA v1.2 Build L2 provenance + SPDX/CycloneDX SBOM; updater verifies identity before applying; in-app "verify current install"; T3 v0.1.10 Windows-service + remote-update pattern with our own rollback gating. *(ENG #3; TEMPORAL #12)* **Done when:** a tampered/rolled-back update is rejected and provenance verifies.
- **S8.8 Offline & resource envelope** — no-egress profile (local model bundles, signed offline update bundle, zero outbound spans proof), air-gapped provisioning; caps on transcript bytes/artifacts/checkpoints/sidecar RSS with archival + storage dashboard; low-resource mode floor (8GB RAM, no GPU). *(ENG #13#14)*
- **S8.9 Benchmark CI tier** — golden recorded traces replayed with no model calls for deterministic regressions; nightly/weekly real-model evals with fixed budget + seeded sampling; response cache keyed by (prompt hash, model version, params); Pass^k with CIs. *(ENG #15)*

> **P8 Gate:** traces correlate; signed-update + key-rotation + restore drills pass; self-UI WCAG 2.2 AA audit green; offline profile proves zero egress; benchmark CI tier gates releases.

---

## 11. Phase 9 — Productization (from P5) †

### S9.1 Packaging & distribution
- winget / scoop / brew / npm / pip / app-store channels; single signed binary; per-user install; auto-start gateway.
- **Done when:** clean-machine install from each channel works.

### S9.2 Pricing & licensing experiments
- BYO-API-keys tiers; free/commercial split; license choice (MIT core vs commercial extensions); plugin marketplace economics (fees, payouts, fraud controls).
- **Done when:** 2 pricing variants measured with real users; license/governance docs published.

### S9.3 Community, docs & SDKs
- Public SDKs (TS/Python), rulepack marketplace, skill provenance reputation, contributor guide, governance model, support SLA.
- **Done when:** an external contributor ships a skill through the vetted pipeline.

### S9.4 Trust as moat
- Public audit chain export, ID-card accountability pages, transparency reports, "why did you do that?" time-travel explainer, agent insurance/bonding partnership exploration.
- **Done when:** an auditor reconstructs a task from receipts alone.

### Blind-spot additions (†)
- **S9.5 License & entity** — `LICENSE` = Apache-2.0 core + `ee/` commercial carve-out, MIT SDKs/contract; `LICENSING.md` before first external PR; US non-profit/LLC holding entity, "AI-Harness" trademark (classes 9/42), trademark policy, defensive patent only if enterprise requires. *(BUS #2#4)*
- **S9.6 Pricing** — Free (BYOK/local core forever), Pro ~$12/mo (second-opinion reviews, router, spend dashboard), Team ~$20/seat (per-seat keys, SSO, audit export); 4-week price-page A/B; published competitor pricing table; flat subscription, no token markup. *(BUS #1#14)*
- **S9.7 Liability & compliance cost** — counsel terms with higher carve-out for autonomous actions + vendor indemnity for core defects; tech-E&O with affirmative AI wording (Armilla/HSB/Counterpart); receipts as the underwriting artifact; document BYOK deployer/tool-provider status; DPA + incident-response runbook; 2-page investor narrative. *(BUS #3#15)*
- **S9.8 Governance, DevRel & SDK** — `GOVERNANCE.md`/`CONTRIBUTING.md`/`OWNERS.md`/CoC/RFC template, contributor ladder, AAIF Sandbox target; part-time DevRel, public roadmap, showcase/, monthly "guardrail of the month"; v1 public contract + MIT TS/Python SDKs generated API reference + deprecation policy. *(BUS #5#6#7)*
- **S9.9 Distribution, support & pilots** — winget/scoop/brew/npm + signed .exe/.msi/.deb with CI manifest PRs; Community/Pro/Enterprise support tiers with SLA; AAIF + Microsoft/UIA + insurer partnerships; 5 paid design-partner pilots ($2–5k) measured by pass^k on their own tasks. *(BUS #8#9#10#12)*
- **S9.10 Marketplace & metrics** — 85/15 creator split, Stripe Connect monthly payouts, $20 minimum, 14-day refunds, signed run-receipt reputation, mandatory scan + human review for confirm-tier skills; north-star **weekly successful delegated tasks per active user** guarded by pass^k ≥0.9 and zero unreviewed irreversible actions; capability-retention KPI. *(BUS #11#13; HUMAN #15)*

> **P9 Gate:** distribution live; pricing validated; license/entity/insurance in place; governance + SDK + support SLA published; external contribution pipeline working; marketplace and north-star metrics live; trust artifacts published.

---

## 12. Cross-cutting workstreams (run every phase)

| Workstream | Owner track | Per-phase obligation |
|---|---|---|
| **Evals & CI** | core | New corpus per capability; regression blocks release |
| **Safety / red team** | policy | Injection + malicious-repo + canary + chaos drills each phase |
| **Observability** | gateway | Trace IDs and audit chain on every new effect |
| **Docs / AGENTS.md** | all | Root `AGENTS.md` ≤30 lines; docs/ updated with each phase |
| **i18n / a11y** | UX | New surface ships localized + keyboard/SR accessible |
| **Cost telemetry** | core | Per-task token/$ footer; budget meter on every surface |

---

## 13. Risk register & kill criteria

| Risk | Signal | Response |
|---|---|---|
| Long-horizon failure (OSWorld 2.0 binary ~21%) | >7% of steps on detection/repair; repeated deviation | checkpoint restart + fresh context; escalate to human |
| Approval fatigue (58%/81% broad-rule elicitation) | rising approve-always rate | batch confirms; deterministic auto-grade; TL entries expire |
| Memory/social-graph poisoning (up to 93.8% ASR) | provenance gaps; private/public belief divergence | quarantine + washable restore + hidden probes |
| Silent quality rot (§25.5) | pass^k decay on user's own tasks | personal reliability report + rot canary |
| Capability outruns defense (RedTeamCUA 83% ASR) | new attack corpus success | egress DLP + out-of-band guard; no model-dismissable gate |
| Scope creep | capability PR without eval corpus | reject PR |

---

## 14. Blind-spot integration (complete)

Seven parallel research subagents produced **102 evidence-cited findings** (`BLIND-SPOTS-01..07.md`), consolidated in **`BLIND-SPOTS-INDEX.md`**. Every finding has been mapped to a † step above or to an existing step:

| Report | Findings | Landing zones in this plan |
|---|---|---|
| 01 Temporal | 15 | S1.5, S1.8, S1.9, S2.2, S2.12, S3.7, S5.6, S7.5, S8.7, S8.8 + stale-fact corrections in `PLAN.md` §§17–28 |
| 02 International | 13 | S5.6, S6.8, S7.5, S7.6, S7.7, S7.8 |
| 03 Engineering | 15 | S0.7, S1.10, S2.8–S2.11, S4.6, S7.7, S8.6–S8.9 |
| 04 Business | 15 | S9.5–S9.10 |
| 05 Human | 15 | S0.8, S0.9, S1.7, S3.6, S6.9, S8.5 |
| 06 Verticals | 15 | S6.10 |
| 07 Safety | 14 | S1.8, S1.9, S1.10, S2.8, S2.12–S2.14, S7.8, S7.9 |

**Highest-leverage changes to the original roadmap**
1. **S1.8 (P0)** — the security boundary moves from Reader/Actor to capability isolation + SMT shielding + kernel tiers. This is the single biggest architectural correction.
2. **S0.9 + S1.7 (P0)** — oversight becomes measurable: reviewability, consent ledger, fatigue KPIs, ambient channel, takeover budget. Confirms stop counting as safety by themselves.
3. **S1.9 (P0)** — eval integrity: benchmark version pins + contamination audit + Pass^k CIs.
4. **S7.x (new track)** — jurisdiction/regime, global identity/rails, i18n/time, standards mapping, impact telemetry.
5. **S8.x (new track)** — key lifecycle, secure updates, offline/resource envelope, benchmark CI.
6. **S9.x (parallel from P5)** — license, entity, pricing, liability/E&O, governance, distribution, marketplace.

**Nothing in the original build order was removed.** The sequence safety → coding → extensibility → browser → GUI → comms → obligation custody remains; the † steps add the missing human, jurisdictional, lifecycle, and product layers around it. See `BLIND-SPOTS-INDEX.md §3` for the ranked must-fix list and §5 for facts in `PLAN.md` that are now factually stale.

---

## 15. Best-of coding-agent integration (2026-11-17)

Eight more subagents audited the actual source of OpenCode, Codex, OMP (`oh-my-pi`), T3 Code v0.1.10, grok-build, a broad OSS sweep, closed harnesses, and OpenAI Dots. Full evidence, ranked "steal this" list, "do NOT copy" list, and `PLAN.md` corrections are in **`BEST-OF-ADOPTION.md`**. Integration summary:

| Source | Mechanisms folded into this plan |
|---|---|
| **OpenCode** | Versioned additive protocol + deprecated aliases (**S0.7**); confined Code Mode VM (**S0.2/S1.4/S1.8**); two-repo content-addressed snapshots (**S1.1**); typed permission engine (**S0.4/S1.3**); plugin scope/revision/auto-disable (**S2.2**); measured-token compaction + `keepID` (**S2.4**); bounded SSE overflow-fail (**S1.4/S2.9**) |
| **Codex** | Two-axis sandbox `PermissionProfile × AskForApproval × reviewer` (**S0.4/S1.8**); bubblewrap+seccomp and MXC/elevated/unelevated Windows ladder (**S1.8/S4.2**); Starlark exec policy + tree-sitter splitting (**S1.2**); Guardian with ordered decision menus + 3-of-50 circuit breaker (**S0.9/S1.7**); hash-trusted hooks (**S2.2**); goals + token accounting (**S1.1/S1.4/S1.5**); encrypted remote compaction blob (**S2.4/S2.14**) |
| **OMP** | **TTSR** stream-time rule injection + time-travel retry (**S1.2/S1.5/S0.4**); OTel spans with internal adapter (**S2.8**); schema-validated subagent yields + worktree isolation + park/revive (**S2.3**); `Redacted` secrets (**S0.4/S8.6**); **new S1.11** model-role routing + fallback chains |
| **T3 Code v0.1.10** | Transactional outbox + command receipts + lease worker (**S2.5/S2.9/S2.11**); TOCTOU recheck under lock (**S0.4/S1.8**); branded compile-time tool declarations (**S0.2/S2.2**); NTFS-aware path containment (**S1.2**); restore-safety gate (**S1.1**); DPoP/PKCE/one-use tickets (**S2.12**); blue-green Windows-service update with rollback (**S8.7**) |
| **Grok / grok-build** | Kernel sandbox-as-data + violation→grant (**S1.8**); ACP 4-transport runtime + replay-on-reconnect (**S0.7/S1.6**); checkpoint cap + hashed session ids (**S1.1/S2.9**); emit-time telemetry validator (**S2.8**); Watch/Take Over ritual (**S5.4/S1.7**). Reject: whole-repo GCS exfil, "approval doesn't reverse work", shared-account sessions, card data in context |
| **Dots** | Separate guardian reviewer hardened with our Z3/two-key (**S1.8/S0.9**); multi-model in-task routing (**S1.11/S3.7**); read-only-by-code private notes (**S2.4/S5.3**); Activity View + Take Over (**S5.4/S1.7**); one-shot payment tokens (**S5.6**); model-paused sign-in (**S0.4/S8.6**) |
| **OSS sweep + closed harnesses** | Three-axis `code|conversation|both` rewind (**S1.1**); Kilo Code Mode with host-owned budgets (**S0.2/S1.4**); OpenHands immutable event tree (**S2.5/S2.9**); OSV install gate (**S2.2/S8.7**); Aider ranked repo map + architect split (**S2.4/S1.5**); LSP semantic refactor rule (**S2.7**); ACP elicitation (**S2.7/S0.9**); managed-only fail-closed hooks (**S2.2/S0.4**); portable cross-vendor policy (**S2.2/S9.3**) |

**New step added:** **S1.11 Model-role routing & fallback chains** (nine roles, per-role fallback on 429/quota, path-scoped models, visible model/effort, published matrix, team allowlists).

**Explicit refusals inherited from the field:** never `yolo`-by-default, never fail-open hooks/checks, never a classifier as the boundary, never full-access default, never unbounded approval scopes, never whole-repo telemetry, never silent model swaps, never a plugin-API break without a deprecation window, never an unverified savings claim. See `BEST-OF-ADOPTION.md §4`.

---

*Build order principle (unchanged): safety core → coding → extensibility → browser → GUI → comms/scheduler → obligation custody → cross-platform → hardening → productization. Every capability pays the safety/eval tax before it ships.*
