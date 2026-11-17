# Agent Research 09A — Hermes: Identity Resolution + Deep Source Audit

*Audit date 2026-11-18. Target resolved to `github.com/NousResearch/hermes-agent`, default branch `main`, shallow-cloned at `37b3e405a1b5a73120f9a2ad6e18a3233c6ba986` (2026-11-17T18:44:09Z) into `C:\Users\ASUS\AppData\Local\Temp\opencode\hermes-agent`. GitHub API `/commits/main` at audit time reported `b356613ba4001b31fcacfe59c5ed8e40e9254c08` (2026-11-17T18:42:43Z, signed merge by Teknium) — same day, ~90 s apart; both are cited where relevant. All line refs below are from the clone HEAD unless the API commit is named explicitly.*

> **Delta over existing docs.** `PLAN.md` §25.4 (line 860) names Hermes exactly once: "local-first desktop agents rising as a Reddit-documented pattern (MantisClaw, Hermes — 'requires tech investment')." `BEST-OF-ADOPTION.md` contains **zero** Hermes coverage (its 8 reports cover OpenCode, OMP, Codex, closed harnesses, OSS agents, Grok, Dots, T3). Everything in this report is new detail; no claim below repeats an already-catalogued mechanism without adding a Hermes-specific implementation and line ref.

---

## 1. Identity resolution (and namesakes ruled out)

**Resolved identity: `NousResearch/hermes-agent` — "The agent that grows with you" (Hermes Agent ☤), built by Nous Research.**

Evidence chain:

1. **Context fit.** §25.4 pairs Hermes with MantisClaw as a *local-first desktop agent* pattern. `NousResearch/hermes-agent` is precisely that: a personal agent that installs natively on Windows (`setup-hermes.ps1`, `README.md:43-61`), ships an Electron Desktop app (`apps/desktop/package.json`, `"main": "dist/electron-main.mjs"`) and a TUI, and keeps state under `%LOCALAPPDATA%\hermes` / `~/.hermes` (`README.md:61`, `hermes_constants.py`).
2. **Ecosystem adjacency to the same "claw" family named in the doc.** The repo explicitly migrates from OpenClaw: `README.md:188-215` includes `hermes claw migrate`, imports `SOUL.md`, `MEMORY.md`, `USER.md`, skills and approval allowlists from `~/.openclaw`. Community bridges (`README.md:235` HermesClaw) and adjacent projects (`garrytan/gbrain` = "Garry's Opinionated OpenClaw/Hermes Agent Brain", `iOfficeAI/AionUi` = "OpenClaw, Hermes, Claude Code...", `NVIDIA/NemoClaw` = "Run agents like Hermes, LangChain Deep Agents, and OpenClaw more securely") all place Hermes Agent in the same local-desktop-agent cohort as OpenClaw/MantisClaw.
3. **Scale/governance fit for "requires tech investment".** 264k stars, 59.6k forks, a Python+TypeScript monorepo of ~8,000 source files, a 1.5 GB checkout, and an install chain that bundles Python 3.14, Node, pnpm, ripgrep, FFmpeg and Git (`README.md:53-55`) — this is unambiguously the "tech investment" example, not a one-line bot.

**Honesty caveat:** §25.4 gives no repo URL and the original Reddit thread was not retrievable (the web-search integration returned no results for all queries). The resolution is a strong inference from the doc's category + naming + ecosystem evidence, **not** a quoted source from that thread. No repo was invented: every path below exists at the pinned commit.

**Namesakes ruled out:**

| Namesake | Why ruled out |
|---|---|
| **NousResearch Hermes LLM family** (Hermes 2/3, OpenHermes models) | Same organization, different artifact class: model weights/instruction-tuned LLMs, not an agent harness with a loop, tool registry or approval model. The doc says "desktop agent", which is the repo, not a checkpoint. |
| **`fathah/hermes-desktop`** ("Desktop Companion for Hermes Agent") | A third-party companion GUI *for* the agent, not the agent itself; differs from the canonical `apps/desktop` Electron app in the target repo. |
| **`nesquena/hermes-webui`** ("use Hermes Agent from the web or from your phone") | Third-party WebUI wrapper; a surface, not the runtime. |
| **"Hermes" messaging/IM bots** (e.g. Slack/Telegram bots named Hermes; `langbot-app/LangBot` integrates "openclaw / hermes agent") | Chat-bot integrations *of* Hermes Agent, or unrelated bots; the doc distinguishes local-first desktop agents from messaging bots. |
| **Meta/Facebook Hermes JavaScript engine** (React Native runtime) | A JS engine, no agent loop, no tool permissions; name collision only. |
| **Hermes the airline/GDS system (Amadeus), Hermes the Greek deity, other "Hermes" homonyms** | Domain/name collision, no agent semantics. |

---

## 2. Freshness

| Signal | Value (fetched 2026-11-18) |
|---|---|
| Repo | `github.com/NousResearch/hermes-agent`, homepage `hermes-agent.nousresearch.com`, tagline "The agent that grows with you" |
| License | **MIT** (`LICENSE`, confirmed in clone) |
| Stars / forks / watchers / subscribers | **264,162 / 59,589 / 264,162 / 1,008** |
| Open issues | **53,475** |
| Created | 2025-07-22 (≈16 months old) |
| Pushed at (API) | **2026-11-17T18:42:48Z** — active the day before audit |
| Default branch / language | `main`; Python primary, large TypeScript monorepo |
| Clone HEAD | `37b3e405` (2026-11-17T18:44:09Z, Teknium, merge of `refactor/storage-sqlite-backend`) |
| Latest release | **v0.23.2**, tag target `ae2a81ccc00…`, published 2026-11-16T20:59:18Z, not prerelease; "1,989 merged PRs, 7,918 non-merge commits, 5,211 changed files, 940 closed issues since v0.23.1" |
| Release channels | CLI + installers, **Microsoft Store app**, Desktop `.exe/.dmg` installer, Docker (`nousresearch/hermes-agent:stable`), Hermes Cloud |
| Checkout size (sparse) | repo `size` field 1,505,535 KB (~1.5 GB); sparse checkout used here: 12,304 files (2,464 `.py`, 5,595 `.ts/.tsx`) |
| Tests (doc claim) | ~25,000 tests across ~1,250 files (`website/docs/developer-guide/architecture.md:139`) |
| Contributors | Exact count not exposed by the API response body; 59.6k forks, a 5.8 KB `.mailmap`, and ~2k PRs per release window indicate a very large contributor base. `teknium1` (NousResearch) is the public merge authority; merge commits are PGP-verified. |

**Freshness verdict:** among the fastest-moving repos we have audited — thousands of PRs per release window and a new storage-interface refactor landing on the audit date. Treat line refs as a point-in-time snapshot; numbers will drift faster than for T3/Codex.

---

## 3. Architecture (confirmed from source, not README marketing)

Hermes is **a complete personal-agent runtime**, not a wrapper/control surface (contrast T3 in report 08). One `AIAgent` class serves CLI, gateway (Telegram/Discord/Slack/WhatsApp/Signal/email + webhook/API), ACP (VS Code/Zed/JetBrains), cron, batch and API-server entry points (`website/docs/developer-guide/architecture.md:13-49, 196-220, 264-269`).

```
entry points: cli.py | gateway/run.py | acp_adapter/ | batch_runner.py | cron/ | API
        │
        ▼
AIAgent (run_agent.py facade)  ── loop in agent/conversation_loop.py:1810 run_conversation()
   ├─ prompt: agent/prompt_builder.py (stable→context→volatile tiers)
   ├─ provider: runtime resolver → chat_completions | codex_responses | anthropic_messages
   ├─ tools: tools/registry.py (70+ tools, ~28 toolsets, import-time self-registration)
   ├─ guardrails: agent/tool_guardrails.py, tools/approval.py (28 approval_*.py helpers)
   ├─ backends: tools/environments/{local,docker,ssh,modal,daytona,vercel_sandbox} + browser/MCP
   ├─ memory/skills: agent/memory_manager.py, agent/curator.py, skills/, plugins/memory/
   └─ persistence: hermes_state.py + hermes_state_*.py (SQLite+FTS5) → NEW persistence/ backend-neutral contracts (34 Protocols, 22 repositories)
```

Key design facts worth borrowing (all with source):

- **Profile isolation** — each `hermes -p <name>` gets its own `HERMES_HOME`, config, memory, sessions and gateway PID; concurrent profiles run side by side (`architecture.md:269`).
- **Prompt stability** — system prompt does not mutate mid-conversation except explicit user action like `/model` (`architecture.md:264`); implemented via `_restore_or_build_system_prompt`, `_stored_prompt_matches_runtime`, `_ensure_cached_system_prompt_static` (`agent/conversation_loop.py:802, 957, 1276`).
- **Storage interface refactor (audit-date)** — `persistence/__init__.py` declares a backend-neutral contract: "no public unit-of-work at this boundary: atomic transitions are domain operations on the owning repository," with typed error taxonomy (`AuthorityRefusal`, `FenceStale`, `TurnLeaseLost`, `RewindRefused`, `ProofRejected`, …), `AuthorityEpoch`/`SessionRevision`/`ExecutionGeneration` fences, and capability declarations. `persistence/authority/admissions.py` implements a durable **admission ledger** with FIFO claims, leases, steering, `unknown` settlement and boot recovery. This is the most sophisticated durability design we have seen in the field.
- **Three-axis platform core** — CLI/gateway/TUI/DACP share the agent; platform differences live in entry points, not the loop (`architecture.md:267`).

**Coverage caveat:** this is an ~8,000-file monorepo; I audited the named mechanism areas only. I did not run the code, its ~25k tests, or exercise the Desktop UI. Line refs are from a sparse checkout; a few directories (e.g. `contributors/`, full `website/`) were not materialised locally.

---

## 4. Top mechanisms — file refs + adoption mapping

Mapping convention: **[O]** = `Optimal/phases.md` step (O0.1–O8.4); **[S]** = cross-track `docs/IMPLEMENTATION-PHASES.md` step. Where both apply, both are given.

### 4.1 Consent as an allow-list, normalized before checking, aliases to the narrowest scope
**Source:** `tools/approval.py:878-908`.
- `CONSENT_ANSWERS = frozenset({"once", "session", "always"})` (`:885`). The comment documents the historical bug: a deny-list that "refused 'deny', None and 'timeout' and treated every other value as a yes — so an unexpected answer reaching `grant` executed the action." Now anything unrecognized is refused.
- Platform adapters normalize case/whitespace and affirmative aliases **before** the allow-list (`:887-900`); `_CHOICE_ALIASES = {"approve": "once", ...}` and the comment: "Every alias maps to 'once', the NARROWEST scope: an Approve tap is one operation and must never persist a session or permanent grant" (`:898-900`).
- Non-string answers become `""` and are refused (`:905-906`); `_UNRECOGNIZED_ANSWER` (`:911-916`) tells the model to report the failure and not retry.

**Copy:** exact-scope cards plus an allow-list of consent tokens; normalize transport vocabulary first, map every affirmative alias to one-shot. **Maps to:** **[O1.3]** (confirm UX with ordered decisions), **[S1.3]** (tiered permission UX), **[S6.5]** (two-key gate). Our plan already has exact-scope cards; the new detail is the *normalize-then-allow-list* ordering and the "aliases collapse to once" rule.

### 4.2 Denial result carries explicit anti-retry / anti-route-around instructions
**Source:** `tools/approval.py:911-927` (`_UNRECOGNIZED_ANSWER`, `_STOP_COMMAND`, `_STOP_ACTION`), injected into gateway/Cli denial templates at `:929-978`.
- Wording: "The user has NOT consented to this action. Do NOT retry this command, do NOT rephrase it, and do NOT attempt the same outcome via a different command. Stop the current workflow and wait for the user."

**Copy:** put this in the tool result (the model sees it), not just the UI. **Maps to:** **[O1.3]**, **[O1.6]** (finish gate / loop breaker), **[S1.3]**, **[S1.7]** (counterfactual cards / Contest action). This is a concrete gap in our current O1.3 wording.

### 4.3 Unconditional floors evaluated before every bypass; user `deny` is never bypassable
**Source:** `tools/approval.py:1299-1333`, `1336-1369`.
- `_floor_block` (`:1310-1333`) runs **before** yolo / `mode=off` / cron approve-mode: hardline catastrophic commands, sudo-stdin password piping, the user's own `approvals.deny` rules, and — notably — "deletion of the Python interpreter/venv this very runtime boots from (a delete the agent cannot walk back — the next start fails before any tool can run, #58748)".
- `_user_deny_block` (`:1299-1307`): deny rules "are documented as never bypassable — not by yolo, not by mode=off, and not by an isolated container either: they express intent about what the agent may DO, not what it can reach."
- `_should_skip_container_guards` (`:1282-1296`): Docker with host bind mounts keeps the guards; Modal/Daytona/Vercel skip; unknown plugin backends fail **soft to False** (guards stay on).

**Copy:** a fixed precondition ladder with `never` rules outside the bypass surface; block runtime self-deletion. **Maps to:** **[O0.4]** (canonicalize + `never` tier + Z3 at load), **[O1.3]**, **[O1.5]** (Windows boundary fail-closed), **[S0.4]**, **[S1.8]** ("agent cannot rewrite its own tooling/crew" — this is its concrete instance).

### 4.4 Guardian step with denial circuit-breaker and unattended instant-deny
**Source:** `tools/approval.py:73-98` (denial tally), `:981-1010` (`_smart_gate`), `:1023-1025` (manual-only descriptions).
- APPROVE approves **this command only** — "pattern-level persistence would let one benign command suppress review of later commands in the same broad detector category" (`:988-990`). DENY counts toward a breaker and can be overridden once by an interactive owner. ESCALATE follows normal manual behavior.
- Unattended programmatic platforms deny instantly: "never a pending approval nobody can answer (#37284, #87509)" (`:1004-1008`).
- Any answer that "can discard the operator's unsaved work" (osascript) skips the guardian entirely because its APPROVE "is not the consent of the person who owns that work" (`:1023-1025`).

**Copy:** guardian is advisory with a circuit breaker and no pattern-level persistence from a model verdict; unattended contexts resolve deterministically. **Maps to:** **[O6.2]** (guardian advisory, 3-of-50 breaker, Z3+two-key underneath), **[O2.6]** (consent ledger + fatigue KPIs), **[S0.9]**, **[S1.7]**. Do **not** copy the model-as-approver authority.

### 4.5 Bounded, fail-closed tool availability probes
**Source:** `tools/registry.py:260-289` (TTL/grace constants), `:368-445` (`_Probe`, `_call_check_fn`).
- `check_fn` availability probes run on a daemon thread under a copied context, with a hard timeout (`_call_check_fn`, `:389-445`); concurrent identical probes are deduplicated into one shared worker (`:397-407`).
- 30 s TTL cache plus a 60 s "grace window" where a flapping probe serves the last-good `True` — explicitly to prevent "a whole toolset [being] silently strip[ped]" (`:260-267`). Unknown backend failure keeps guards on (`:1289-1296` above).
- `ToolEntry` (`:216-244`) carries `max_result_size_chars`, `execution_timeout`, `deadline_floor`, `dynamic_schema_overrides`, `compat_args`, `self_checked_args`.
- Plugin override of a built-in tool requires operator opt-in (`_PluginOverridePolicy`, `:246-257`).

**Copy:** probe with timeout, dedupe, TTL + last-good grace, fail-closed on timeout, and non-overridable built-in tools. **Maps to:** **[O0.2]** (tool registry with `probe.cmd` gating — near-identical intent), **[O4.3]** (fail-closed required MCP server), **[S0.2]**, **[S2.2]**. Our plan lacks the flapping-grace nuance.

### 4.6 Tool-loop guardrails with canonical call signatures and hard stops for unattended runs
**Source:** `agent/tool_guardrails.py:93-195`, controller at `:302`.
- `LoopCapConfig` (`:93-105`): hard per-turn ceilings on `web_search` calls and subagent spawns, counting **total** calls, not repeats.
- `ToolCallGuardrailConfig` (`:108-143`): warn-after 2 / block-after 5 identical failures; same-tool fail warn 3 / halt 8; no-progress warn 2 / block 5. "Warnings never prevent execution; hard stops are opt-in on interactive platforms, **default on** for unattended gateway/cron."
- `ToolCallSignature.from_call` (`:156-165`) hashes canonical sorted-JSON args (`_sha256(canonical_tool_args(...))`); `IdenticalCallObservation` (`:146-153`) can replace a byte-identical duplicate result with a stub rather than re-running the tool.

**Copy:** canonical arg hashing + identical-call stubbing + separate interactive/unattended thresholds. **Maps to:** **[O1.6]** (loop breaker "≤3 reps" — Hermes gives the exact threshold ladder), **[O1.7]** (per-task budgets), **[S0.3]**, **[S1.5]**.

### 4.7 Egress proxy as enforced data, with collision-proof environment merging
**Source:** `tools/environments/docker_egress.py:20-136`, `:214-275`.
- `iron-proxy` injects `HTTP(S)_PROXY` to a local CONNECT listener, mounts the CA read-only, and substitutes **per-provider proxy tokens under the real provider env names** so SDKs work unchanged (`:103-109`). A `HERMES_EGRESS_PROXY=1` marker tells the in-sandbox agent it is proxy-aware.
- Half-configured states (enabled but unconfigured/not running/CA missing/mappings empty) **raise so the sandbox refuses to start** when `proxy.enforce_on_docker` is on (default true) — only `ImportError` is swallowed (`:29-53`).
- `_PROXY_CONTROL_ENV` (`:21-26`) plus collision guards (`:222-275`) prevent user-supplied `-e` / `extra_args` from overriding proxy/CA variables. Config read failures fail safe to the enforcing default (`:127-136`).

**Copy:** egress deny-by-default inside the sandbox, token substitution, fail-closed on half-configuration, and treat proxy-control env vars as protected. **Maps to:** **[O1.4]** (egress DLP + "what left the machine" receipt), **[S1.4]**, **[S1.8]**. Strongest concrete egress implementation we have found.

### 4.8 Sandbox identity isolation and cross-agent refusal
**Source:** `tools/environments/sandbox_scope.py:33-140`.
- `SandboxIdentity(tenant, agent)` with percent-quoted cache key (`:42-52`); `sandbox_visible` / `event_visible` (`:90-119`) make a record visible only to its owning agent.
- `require_identity` (`:122-129`) raises `CrossAgentHandleRefused` when a handle names another agent's sandbox; `admit_env` (`:132-140`) refuses a cached environment that is not the current agent's.

**Copy:** stamp every sandbox handle/event with a tenant+agent identity and refuse cross-tenant reuse. **Maps to:** **[O5.4]** (single desktop lock — one user's desktop is serial), **[S1.8]**, **[S4.2]** (capability isolation). Note Hermes needs this because it multiplexes hosted agents; our single-user desktop may adopt only the handle-refusal pattern.

### 4.9 Windows-native directory ACL, elevation and reparse-point handling
**Source:** `hermes_dir_acl.py:57-77, 129, 290, 328-381`.
- Refuses reparse points in tree walks (`ReparsePointRefused`, `:57`, `iter_tree_without_reparse_points`, `:328`), checks process elevation (`:77-129`), grants access with expected-owner verification via pywin32/icacls (`:142-230`, `:290-328`).

**Copy:** explicit elevation detection and reparse-point refusal before any recursive Windows filesystem grant. **Maps to:** **[O0.1]** (single-instance/DPI Windows setup), **[O1.5]** (never-elevated daemon), **[O4.1]** (Windows fs adapter with symlink/OneDrive/reparse guards), **[S4.2]**.

### 4.10 Model-scoped credential cooldowns (fallback that does not bench the whole account)
**Source:** `agent/credential_pool_model_cooldowns.py:20-94`.
- A usage-window 402 benches **only the refused model**; every other model stays usable on the same credential (`:20-22`). `model_cooldown_until` (`:37-47`) is per-model; unscoped callers stay conservative (any active cooldown blocks them). `merge_model_cooldowns` (`:50-59`) takes the latest reset per model across snapshots.
- `_is_model_scoped_failure` (`:76-90`): per-model 429 (Anthropic) and model-entitlement 400 (Codex account) are model-scoped, not credential-scoped.

**Copy:** this is the working implementation of the gap that forced **new step S1.11** in `BEST-OF-ADOPTION.md` §6. **Maps to:** **[S1.11]** (model-role routing + per-role/per-model fallback chains + cooldown restore), **[O8.4]** (published matrix, no silent swap), **[S3.7]**. Our S1.11 text names the pattern; Hermes supplies the per-model-vs-per-credential detail.

### 4.11 `execute_code` as a budgeted, authenticated, allow-listed tool RPC (Code Mode analogue)
**Source:** `tools/code_execution_rpc.py:85-123`.
- Constant-time RPC token check that fails closed on an empty token (`:85-90`); allow-listed tools and a per-execution `max_tool_calls` (`:93-105`); refusals are free (do not consume budget).
- For `terminal`, blocked params are stripped from script-supplied args before dispatch (`:106-108`); all calls are logged with tool/args-preview/duration/error (`:116-122`).

**Copy:** keep our `rhai` confinement but adopt: constant-time token, refusals-free budget accounting, protected-param stripping, per-call audit log. **Maps to:** **[O4.5]** (Code Mode rhai with host budgets, nested permission attribution, no nested Code Mode, cap 8), **[S0.2]**, **[S1.4]**.

### 4.12 Skill guard: content scanning, content hash, source-derived trust levels
**Source:** `tools/skills_guard.py:788-1004, 1017-1081` and `tools/skill_provenance.py:12-43`.
- `scan_skill` / `scan_skill_cached` (`:910, :957`) scan text, code, shell-write and prose-modify patterns, with decoded views and unicode-name masking (`:637-674, :751-788`); `content_hash` (`:951`) supports cached verdicts keyed by content.
- `_resolve_trust_level(source)` (`:1064`) and `should_allow_install(result, force)` (`:988`) gate installs; `_determine_verdict(findings)` (`:1075`) produces the allow/deny verdict.
- `skill_provenance.set_current_write_origin` / `is_background_review` / `is_unattended_review` tag *who* wrote a skill, so autonomous writes are distinguishable from human ones.

**Copy:** hash-keyed scan cache + source-derived trust + write-origin provenance. **Maps to:** **[O4.4]** (vetted install, hash-trusted hooks, managed-only channel), **[S2.2]** (plugin scope/revision/auto-disable), **[S9.3]** (skill provenance). Do **not** copy autonomous write-origin without our two-key guard.

### 4.13 Skill curator — autonomous staleness lifecycle with deferral and pinning
**Source:** `agent/curator.py:158-217` (plus transitions to `:464`).
- First observation seeds `last_run_at` and **defers one interval** — "a fresh install/update never mutates the library on its first tick" (`:158-168`).
- Pure-function transitions move skills active→stale→archived by real activity (`:198-217`); pinned/locked skills are never touched; cron-referenced skills are treated as pinned (`:173-209`); built-ins are seeded with a baseline so their inactivity clock starts now.
- Ledger actor is tagged `"curator"` so an autonomous transition reads as such (`:183-195`).

**Copy:** TTL/staleness with first-run deferral and pinned references. **Maps to:** **[O2.4]** (file-first memory with TTL/staleness + re-validation — direct match), **[O4.4]**, **[S2.4]**. Our O2.4 says "stale recalled fact is re-checked against current files before acting"; Hermes adds the "don't mutate on first tick" and "pinned by scheduler reference" refinements.

### 4.14 Self-improvement loop (skill creation nudges) — copy as a *review* signal only
**Source:** `agent/agent_init.py:1234-1237` (`_skill_nudge_interval`, default 10, configurable `agent.skills.creation_nudge_interval`), `agent/review_signals.py:37`, `agent/codex_runtime.py:733`, `agent/learn_prompt.py:177-187`, `agent/curator.py:264-414` (prompts that steer `skill_manage action=create|patch`). The README (`:19, 26`) calls this a "closed learning loop."

**Copy:** the *nudge cadence and review signal* are worth studying; the autonomous create/patch authority is **not** — our **[S1.8]** explicitly requires "agent cannot rewrite its own tooling/crew." **Maps to:** **[O4.4]**, **[S2.2]**, and negatively to **[S1.8]** (must be gated behind propose→verify→two-key activation).

### 4.15 Cron: code-skew yielding, assembled-prompt injection scanning, toolset denylist
**Source:** `cron/scheduler.py:167-236, 407-476`.
- `CronTickYielded` (`:188-220`): a stale-code ticker raises **before** the tick lock when boot fingerprint ≠ disk revision and a fresher gateway owns the lock — prevents two code versions racing on one schedule. "Skew `None` never yields (fail open)" (`:191-194` — caveat below).
- `CronPromptInjectionBlocked` (`:407-415`): scans the **assembled** job prompt including runtime-loaded skill content, closing the gap where create-time scanning missed malicious skills reaching a non-interactive auto-approve cron agent.
- `_resolve_cron_disabled_toolsets` (`:419-440`): `messaging`, `clarify` always disabled; `cronjob` disabled by default for loop prevention; config denylist layered on top so per-job lists cannot widen it. `_resolve_cron_enabled_toolsets` (`:469-476`) fails **closed** on lookup failure.

**Copy:** assembled-prompt scanning at execution time + scheduler toolset denylist + fail-closed lookups. **Maps to:** **[O6.1]** (durable scheduler), **[O6.5]** (subscription agents draft-only), **[O1.6]** (loop gate in every child scope), **[S5.3]**, **[S2.2]**.

### 4.16 Telemetry: split collection/upload consent with pinned endpoint
**Source:** `hermes_cli/config_defaults.py:2274-2287` (and `:2024` privacy note, `:2490-2492` cua-telemetry opt-out).
- `telemetry.shared_metrics.enabled: False` and `send: False` independently; "`send` alone logs an error. A package is sent only if its whole period is inside a recorded consent window." Endpoint is hardcoded HTTPS and "deliberately NOT env-overridable. Non-HTTPS refused unless the host is localhost."

**Copy:** two-key (collect/upload) consent, whole-period consent windows, non-overridable HTTPS endpoint. **Maps to:** **[O8.4]** (wire-verifiable telemetry opt-out), **[O2.5]** (export-time fail-closed validator), **[S2.8]**. Our O8.4 promises "no silent telemetry change ships"; this is a concrete default to invert *toward* (theirs is already off).

### 4.17 Update receipts and packaged-artifact verification
**Source:** `hermes_cli/update_receipt.py:144-226`, `hermes_cli/desktop_update_verify.py:17-70`.
- `UpdateReceipt` records `schema`, `outcome ∈ running|success|partial|failed`, pre/post code identity, `update_id`/`correlation_id`, steps/skips/stages/followups, gateway-restart results. `finalize()` downgrades `success`→`partial` when the user still has to act (`:221-226`). Recovery buckets distinguish `"verified"` from `"relaunch_attempted"` — only the former claims supervisor coverage (`:196-218`).
- Windows Desktop update verifies the Electron ASAR header and declared entry point, rejecting path traversal in ASAR members and requiring a local renderer module tag (`:17-70`).

**Copy:** conservative outcome vocabulary and verified-vs-attempted distinction. **Maps to:** **[O7.1]** (TUF/Sigstore/SLSA, verify-before-apply, blue-green, rollback), **[S8.7]**. **Caveat:** Hermes verifies *structure*, not signatures — we must add our TUF/Sigstore trust root (already in O7.1).

### 4.18 Force-redaction of error surfaces at the persistence boundary
**Source:** `agent/conversation_loop.py:1879-1917`.
- `_redact_display_metadata` force-redacts every string in a `display_metadata` payload with `force=True` — "keeps the boundary closed even when `security.redact_secrets` is off, matching the compressor's persistence boundary" (`:1898-1905`).

**Copy:** redaction enforced at the persistence sink independent of the user toggle. **Maps to:** **[O0.5]** (secret redaction before persistence), **[O2.5]**, **[S0.4]**, **[S8.6]**.

---

## 5. Anti-patterns to avoid (observed in Hermes, inverted in our plan)

| Anti-pattern | Hermes evidence | Our guard |
|---|---|---|
| **One-click "Approvals: off"** | `tools/approval.py:1228-1232` — Desktop toggle writes `approvals.mode: off`; only hardline floors survive. | **[O0.4/S0.4]** deny-by-default; no global bypass toggle; two-key for any downgrade (S1.8). |
| **Session "yolo" bypass** | `tools/approval.py:337-398` (`enable_session_yolo`, `_yolo_active`). | **[O1.3/S1.3]** exact-scope grants with TTL + revoke; measure rubber-stamping (O2.6/S1.7). Already flagged in `BEST §4` for other vendors. |
| **Default auto-approve in non-interactive contexts** | `tools/approval.py:1237-1273`: no CLI/gateway/ask bridge → `_approved()` by default; log prefix literally "AUTO-APPROVED dangerous command in non-interactive non-gateway context" (`:1368`). Only specific contexts (cron/single_query/unattended with `mode: deny`) or `fail_closed_when_no_human=True` block. | **[O0.4/S0.4]** unattended ⇒ deny, never approve; our O6.1 "job that hits confirm skips + notifies" is the right inversion. |
| **Approval-scope overreach via session/always patterns** | `tools/approval.py:456-522` persists pattern keys; a broad pattern key quiets later matches. | **[O1.3]** exact-scope cards, per-item receipts, TTL (already in plan). |
| **Model as approver** | `_smart_gate` APPROVE (`:981-997`). | **[O6.2]** guardian advisory only; Z3 + two-key underneath (already in plan). |
| **Fail-open code-skew probe** | `cron/scheduler.py:167-175` import failure returns `None` ⇒ "Skew `None` never yields (fail open)" (`:191-194`). | **[O7.1/S8.7]** version pinning + verify-before-apply; a failed skew probe must stop the ticker, not let a stale process continue. |
| **Trajectory harvesting / cloud-portal coupling** | README `:19, 30`: ShareGPT trajectories from sessions, "Tool Gateway" bundles web search/image/TTS/browser behind one Nous subscription; telemetry endpoint `telemetry.nousresearch.com` (`config_defaults.py:2285`). | **[O1.4/O8.4/S2.8]** content capture OFF, default-deny egress, provider-agnostic routing (S1.11), no vendor tool bundle as a dependency. |
| **Remote install script piped to shell** | README `:40, 50`: `irm ... | iex`; installer stages Git and bundles an unsigned-Rust `uv.exe` that triggers AV false positives (`:72-103`). | **[O8.1/O7.1]** signed `.exe/.msi`, winget/scoop manifests, SBOM + attestation; never `iex`. |
| **Project-directory plugin/skill auto-loading** | `plugins/plugin_loader.py:81-125` discovers `~/.hermes/plugins/`, `.hermes/plugins/`, pip entry points; project plugins load by default (scanning in `skills_guard.py` mitigates but trust is source-derived). | **[O4.4/S2.2]** opt-in for foreign sources, signed provenance (already flagged in `BEST §4`). |
| **"Local-first" that is actually cloud-first** | README `:19-20, 29`: "$5 VPS, GPU cluster, serverless," six terminal backends including Modal/Daytona/Vercel, "not tied to your laptop — talk to it from Telegram while it works on a cloud VM." | Our **O7.3** air-gapped profile + **O1.4** egress receipts are the differentiator; do not adopt remote-execution defaults while marketing local-first. |
| **Bundled `uv` / AV whitelisting as a support strategy** | README `:98-101`: whitelist the folder rather than the hash because the hash changes every version. | **[O7.2/O7.1]** signed, pinned, hash-verified binaries; hash changes must invalidate trust, not require an AV exclusion. |

---

## 6. Sources

All line refs are from the sparse shallow clone at `37b3e405` unless marked `API b356613`.

**Identity + freshness**
- GitHub REST API: `/repos/NousResearch/hermes-agent`, `/commits/main`, `/releases/latest`, `/search/repositories?q=Hermes+agent…` (fetched 2026-11-18).
- `README.md:5-31, 35-61, 125-215, 229-243`; `LICENSE`; `website/docs/developer-guide/architecture.md:1-283`.

**Architecture / durability**
- `run_agent.py:215` (`class AIAgent`); `agent/conversation_loop.py:802, 957, 1276, 1463, 1810-1924`; `website/docs/developer-guide/architecture.md:13-49, 196-283`.
- `persistence/__init__.py`, `persistence/authority/admissions.py`, `persistence/authority/bindings.py` (added in API commit `b356613`, +67,572 lines in that single merge).

**Approval / policy**
- `tools/approval.py:73-98, 861-1010, 1013-1030, 1207-1386`; `agent/runtime_self_protection.py:76-178, 328`; `agent/redact*.py`.

**Tool registry / Code Mode / guardrails**
- `tools/registry.py:216-289, 368-478, 596`; `agent/tool_guardrails.py:93-212, 302`; `tools/code_execution_rpc.py:85-123`.

**Sandbox / egress / Windows**
- `tools/environments/docker_egress.py:20-136, 214-275`; `tools/environments/sandbox_scope.py:33-140`; `hermes_dir_acl.py:57-129, 290-381`; `hermes_scope.py:38-299`.

**Routing / memory / skills / subagents / cron / telemetry / updates**
- `agent/credential_pool_model_cooldowns.py:20-94`; `agent/memory_manager.py`, `agent/curator.py:158-217, 264-414`; `agent/agent_init.py:1234-1237`, `agent/review_signals.py:37`; `tools/skills_guard.py:788-1081`, `tools/skill_provenance.py:12-43`; `tools/delegate_tool.py:65-594`, `tools/delegation_output_schema.py:19-109`; `cron/scheduler.py:167-236, 407-476`; `hermes_cli/config_defaults.py:2274-2287`; `hermes_cli/update_receipt.py:144-226`, `hermes_cli/desktop_update_verify.py:17-70`.

**Coverage caveats (explicit):**
- Web-search integration returned no results; the original Reddit thread behind §25.4 could not be fetched. Identity is inferred from repo content + ecosystem adjacency.
- Only named mechanism areas were read; no test execution, no Desktop UI walkthrough, no full-history analysis of the ~8,000-file tree.
- Repo mutates at ~2k PRs per release window; all counts and line numbers are 2026-11-17/18 snapshots.
- Exact contributor count was not available from the GitHub API response body; stated proxies are forks, `.mailmap`, and PR volume.
