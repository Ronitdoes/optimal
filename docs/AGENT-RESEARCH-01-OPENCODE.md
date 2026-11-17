# Agent Research 01 — Deep Source-Code Audit of OpenCode

*Research subagent, 2026-11-17. All file paths and line numbers below refer to the pinned commit `0e447fce1623813d8af9bd5d1b511e8e2b4f4f81` on branch `dev` (see §5). Sources were read as raw source files, not README/docs, unless explicitly marked as CHANGELOG/API metadata.*

---

## 1. Repo identity & freshness

| Item | Value |
|---|---|
| Canonical repo | **`anomalyco/opencode`** — `https://github.com/anomalyco/opencode` |
| `sst/opencode` status | Redirects to the same repository (both resolve to repo **id 975734319**, `full_name: anomalyco/opencode`). The SST org was renamed/rebranded to Anomaly; use `anomalyco/opencode` in all citations. |
| Description / homepage | "The open source coding agent." · https://opencode.ai |
| License | **MIT** (`LICENSE`, 1,065 bytes) — safe to copy patterns and code, keep attribution. |
| Default branch | **`dev`** (a separate `2.0` branch also exists; `dev` is the active v2 line). |
| HEAD commit | `0e447fce1623813d8af9bd5d1b511e8e2b4f4f81` — "feat(stories): cover device pairing flows in the Components Lab (#64490)", author `usrnk1`, **2026-11-17T18:14:12Z**, PGP-verified. |
| `pushed_at` | **2026-11-17T18:18:37Z** — pushed minutes before this audit. Extremely fresh. |
| Stars / forks / open issues | **227,527** / 30,844 / 4,069 |
| Created | 2025-04-30 · primary language TypeScript |
| Repo size | ~744 MB, **12,619 tree entries** (pinned snapshot), ~34 top-level `packages/` |
| Latest release in CHANGELOG | **v2.3.0 — 2026-11-13** (4 days before audit). |

**Reading of freshness:** this is one of the fastest-moving OSS repos in existence (dozens of PRs merged daily; commit numbers in the #64,490 range). The v2 rewrite is complete and stable-ish (v2.1 → v2.3 in ~1 month), but the public contracts are still churning (§4). Treat any copied code as a snapshot, pin the commit, and re-audit before shipping.

**Important correction to PLAN.md §15:** "LSP-in-the-loop diagnostics" was a **v1** OpenCode feature. In the v2 tree the in-process LSP implementation is gone — only config schema (`packages/schema/src/config/lsp.ts:6-19`) and an event (`packages/schema/src/lsp-event.ts:5`, `lsp.updated`) remain; there are zero `vtsls`/`pyright`/`gopls`/`lsp-client` source files in 12,619 entries. Copy the **pattern** (compiler diagnostics as loop feedback), not a v2 file — see mechanism 9.

---

## 2. Architecture summary

OpenCode v2 is a **local-first client/server product**, not a single TUI binary:

```
TUI (opentui/SolidJS) ─┐
Desktop app (Electron) ─┼──> generated typed HTTP/RPC clients ──> ONE local server ("Location")
Web app (SolidJS) ──────┤                                          │
ACP/editor adapters ────┘                                          │
                                                                   ▼
   packages/protocol (typed Api + OpenAPI) → packages/server (routes, SSE EventFeed, middleware)
                                                                   ▼
   packages/core (Effect-TS services: Session, Permission, Snapshot, Worktree, Plugin, Bus, Mcp, Skill, Job)
                                                                   ▼
   packages/ai (provider adapters, auth combinators, compaction, retry) + packages/codemode (confined JS VM)
```

Key architectural facts from source:

- **Effect-TS is the backbone.** Nearly every service is an `Effect Context.Service` with a `Layer`, a `node` provider, and scoped finalizers (e.g. `packages/core/src/snapshot.ts:70-74,223-227`; `packages/core/src/permission.ts:129,402-406`). Services are wired declaratively in `packages/server/src/routes.ts:60-100` and replaced per runtime profile at `:137-180`.
- **One server behind every surface.** `createRoutes` / `createEmbeddedRoutes` (`packages/server/src/routes.ts:102-127`) build the same service graph for network and embedded use; `packages/protocol/src/client.ts:28-34` builds the shared `ClientApi`; `effect/` and `promise/` client variants are generated from the same contract.
- **Everything is an event.** `packages/core/src/bus.ts` is a typed pub/sub with optional SQLite persistence, monotonic `seq`, aggregate IDs, versions, and an atomic local-projection commit hook (`:51-72`, `:102-109`). RPC methods publish events to the bus; the server re-broadcasts them as SSE.
- **Schema-first contracts.** Inputs/outputs/errors are validated with Effect `Schema` on both call paths (`packages/core/src/rpc.ts:108-142`); the HTTP API exposes `/openapi.json` (`packages/server/src/routes.ts:207`).
- **Plugins are first-class state participants.** They can transform tool/session/mcp/skill registries and register typed hooks — including a hook that can **rewrite permission decisions** (`packages/core/src/permission.ts:207-215`).
- **Code Mode** is a bespoke confined JavaScript interpreter with its own bytecode, opcode set, stdlib, snapshot/resume, and tool catalog (`packages/codemode/src/*`, ~70 files) — not `vm.runInNewContext`.

---

## 3. Top 10 mechanisms worth stealing (with source refs + AI-Harness adoption)

### 1. One shared local service behind every surface (client/server gateway)
- **Refs:** `packages/server/src/routes.ts:60-100` (shared service node list), `:102-127` (`createRoutes` vs `createEmbeddedRoutes`), `:129-181` (configurable layer replacements), `:207-220` (HttpApi + OpenAPI + auth/policy middleware); `packages/core/src/rpc.ts:16-23,35-175`; `packages/client/src/service.ts` / `service-probe.ts` / `service-version.ts` (tree).
- **What it does:** All clients talk to one server process. Server is assembled from replaceable Effect layers; embedded (TUI) and network (web/desktop/editor) modes share the identical core. RPC is typed, schema-validated, with declared error codes and typed event subscriptions.
- **Adopt →** **S0.1, S0.7**: make `packages/gateway` the only process that owns policy, session, audit and tool execution; CLI/TUI/VS Code/Desktop are thin generated clients. Build layer/service replacement from day one so evals and tests can swap in-memory/storage backends (mirrors our S2.9/S2.11 goals).

### 2. Versioned, additive protocol with generated clients and a real deprecation policy
- **Refs:** `packages/protocol/src/client.ts:36-69` (namespaced endpoint groups), `:71-75` (`experimental` endpoints nested under `client.experimental`, "no compatibility promise"), `:77-106` (`deprecatedClientPaths` frozen set, "keeps a deprecated alias until the next minor"), `:108-124` (Promise/Effect omit lists); `packages/core/src/rpc.ts:188-200` (event type `rpc.<id>.<name>`, ephemeral durability); `packages/server/src/event-feed.ts:29-31` (SSE `data: {json}\n\n`).
- **What it does:** Endpoint groups are enumerated; experimental methods are isolated by URL; moved methods keep deprecated aliases for exactly one minor; clients are code-generated in two flavors. This is the mature version of our S0.7 contract idea.
- **Adopt →** **S0.7, S9.8**: copy the `deprecatedClientPaths` + experimental-namespace mechanism verbatim for `harness.v1.*` method/event names; generate TS SDK from the same schema; publish the one-minor deprecation rule in `GOVERNANCE.md`.

### 3. Typed event bus + bounded SSE streaming (backpressure that fails loudly)
- **Refs:** `packages/core/src/bus.ts:51-58` (serialized event: `id, type, created, seq, aggregateID, data`), `:68-96` (durable envelope `{aggregateID, seq, version}` and single-place legacy payload upgrade), `:102-109` (`commit` = local projection committed atomically with the durable event); `packages/server/src/event-feed.ts:8-13` (`SubscriberCapacity = 4_096`, `SubscriberOverflowError`), `:48-70` (lazy encode once, drop-and-fail slow subscribers), `:76-84` (`Queue.dropping` + type filter).
- **What it does:** Durable events get `seq`/`version` and an atomic projection commit; live SSE subscribers get a bounded dropping queue and are failed explicitly on overflow rather than silently lagging.
- **Adopt →** **S2.8, S2.9**: use this envelope shape for our event schema and borrow the atomic event+projection commit; adopt the bounded-subscriber/overflow-fail rule for terminal and tool output streaming (also complements T3's stream budgets in PLAN §21).

### 4. Code Mode — compose many tool calls into one confined JS script
- **Refs:** `packages/core/src/codemode/instructions.ts:9-23` (model prompt: "cannot be called directly: they only work as JavaScript inside `execute`", plus global `search(...)`), `:19-23,43-133` (token-budgeted catalog and delta updates when MCP tools change); `packages/core/src/codemode/catalog.ts:50-56,56-126` (`INLINE_BUDGET = 2_000`, round-robin signature selection), `:137-143` (signature + 500-char description); `packages/codemode/src/codemode.ts:34-48` (`ExecutionLimits`: `timeoutMs`, `maxToolCalls`, `maxOutputBytes`), `:109-163` (normalized diagnostics as data), `:233-287` (runtime), `:311-317` (suspend/resume); `packages/codemode/src/code.ts:19-118` (immutable compiled bytecode), `:375-435` (FNV-1a hash); `packages/codemode/src/vm.ts` (~103 KB interpreter); `packages/core/src/plugin/mcp-codemode-defaults.ts:9-37` (prevents nested Code Mode when an MCP server is itself Code Mode).
- **What it does:** MCP/tool servers are exposed as a JS namespace inside one `execute` tool. The model writes a script that calls many tools with loops/conditionals; data stays inside the VM; only JSON crosses back. A partial catalog plus `search()` keeps the prompt small; resource limits and typed diagnostics bound abuse. Changelog v2.2.6:238 shows a bytecode VM behind `experimental.codemode_vm`.
- **Adopt →** **S0.2, S2.1, S1.4**: implement a reduced `code_execute` tool for MCP-heavy workflows (massive round-trip reduction). Start with a restricted JS/QuickJS sandbox + our uniform tool envelope; require `maxToolCalls`, `maxOutputBytes`, timeout, and full permission re-check per inner call (borrow `Permission.nested` attribution, mechanism 6). Add the "no nested Code Mode" interop rule.

### 5. Git snapshots + /undo (content-addressed, path-contained, staged restore)
- **Refs:** `packages/core/src/snapshot.ts:43-68` (`capture/files/diff/restore` interface), `:89-106` (dedicated snapshot repo under global data, seed from source repo), `:133-148` (`git.tree.capture`, `maximumUntrackedFileBytes: 2 MiB`, 10-min object pack), `:200-217` (`plan` checks `FSUtil.contains(worktree, absolute)` → "Path escapes the project", then restore); `packages/core/src/session/revert.ts:22-56` (`stage` captures before-state + per-file diffs), `:58-73` (`clear`), `:83-112` (per-message `snapshot.start` used as restore point); `packages/ai/test/provider/checkpoint.test.ts` (tests).
- **What it does:** Every mutating step records a content-addressed tree in a *separate* git repo; revert selects files back to their pre-step trees and emits staged/committed/cleared events. Symlink/escape checks are fail-closed.
- **Adopt →** **S1.1, S1.6, S2.4**: mirror with `refs/harness/checkpoints/*` (PLAN §21) but reuse this two-repo design so snapshots never pollute the user's index; adopt the "capture before mutation" hook and the exact `FSUtil.contains` check; wire undo to the audit trail. Note OpenCode's undo is file-only — our ledger must also undo non-git effects (PLAN §15 "build custom").

### 6. Typed permission engine with hookable decisions, saved "always", unattended rules, and batch attribution
- **Refs:** `packages/core/src/permission.ts:20` (missing agent permissions = **deny-all**), `:94-108` (`evaluate`: last matching rule wins, default `ask`; `merge`), `:165-227` (agent+session+saved rules, **plugin hook can override the effect**, unattended ruleset denies unresolved asks), `:265-305` (`ask`/`assert`, typed `BlockedError`/`CorrectedError`, decline tunnels as a defect so it never becomes model-visible tool output), `:323-384` (reply: reject/always, batch reject propagates feedback, `always` persists and re-evaluates pending), `:190-199,122-126` (`nested()` maps Code Mode inner calls to the parent row); `packages/core/src/plugin/hooks.ts:13-30,88-95`; `packages/cli/src/acp/permission.ts:39-74` (confirm card options once/always/reject + **diff previews**).
- **What it does:** A single typed action/resource/effect rule engine with wildcard matching, default-deny for unknown agents, plugin-extensible policy, unattended-mode settlement, and attribution of nested tool calls. The editor-facing confirm card shows a real diff.
- **Adopt →** **S0.4, S1.3, S1.7**: use as the reference for our canonicalized rule pipeline; improve on it by adding our `reviewability` field and human-oversight KPIs (fatigue/batching — OpenCode has none). Keep the "decline is not model output" trick and the `always` re-evaluation semantics.

### 7. Plugin API with revisioned hot activation, scoped teardown, fallback and auto-disable
- **Refs:** `packages/core/src/plugin.ts:18-100` (`load`: forked scope, grouped failures, fallback), `:103-198` (`activate`: duplicate-ID detection, prefix diff by `{id, revision}`, reverse-order teardown, **restore last healthy activation when a new revision fails setup**), `:200-254` (worker auto-disables a plugin whose state transform fails, with an error `ref`); `packages/core/src/config/plugin/compatibility.ts:13-94` (built-in compatibility plugin loads `SKILL.md` from `.claude/` and `.agents/` roots, debounced 100 ms, semaphore-locked); `packages/core/src/plugin/hooks.ts:13-59` (typed hook domains `aisdk|session|permission|shell|tool` and per-domain failure channels — only `tool.execute.before` may fail).
- **What it does:** Plugins own registrations inside a scope; activation is incremental and reversible; broken plugins are disabled at runtime rather than crashing the server; compatibility with Claude/AGENTS skills is a plugin, not core.
- **Adopt →** **S2.2, S9.8**: copy the scope/revision/fallback/auto-disable model and the typed hook domains for our `plugin.toml` + Agent Plugins 1.0 work. **Do not** copy the break cadence (§4).

### 8. Session compaction with background precompute, cache-awareness, and overflow shrink ladder
- **Refs:** `packages/core/src/session/compaction.ts:41-49` (`Settings {auto, buffer, keep, background}`), `:55-80` (`Trigger` = `auto|overflow|manual`; `precompute()`), `:104-168` (all tuned constants: 15%/16k reserve, `SHRINK_STEPS [0.85,0.7,0.5,0.35]`, `PRECOMPUTE_LEAD = 60_000`, `CACHE_LIFETIME`, `PRECOMPUTED_TAIL_SHARE = 0.4`, max 2 background failures), `:211-249` (structured summary template), `:298-336` (compact with native-provider support + on-overflow smaller target), `:348-400` (precompute guarded by prompt-cache TTL), `:430-438` (`due` uses provider *measured* tokens when available), `:576-590` (native trigger path).
- **What it does:** Summaries are prepared off the critical path, swapped in only when a measured/estimated window threshold is crossed, keep recent tokens verbatim, preserve prompt-cache reuse, and retry with progressively smaller summaries after provider "too long" errors. New in 2.1: summaries reference kept messages via `metadata.keepID` instead of duplicating text (CHANGELOG:1210-1220).
- **Adopt →** **S2.4, S2.9**: this is the best available spec for our "compaction at 75% with artifact flush + rolling brief + checkpoint". Adopt the measured-token threshold, the shrink ladder, and the `keepID` reference model (never duplicate text).

### 9. Worktree-per-task tools + pluggable strategies
- **Refs:** `packages/core/src/worktree.ts:93-102` (`list/create/remove/refresh`), `:121-128` (loads strategy from the target worktree's own plugin set), `:154-186` (DB ownership with conflict guard: "Discovery may claim an unowned row, but never replace another strategy's ownership"), `:204-255` (`create`: canonical paths, slug + `-N` up to 10 attempts, optional `start` command run with `OPENCODE_WORKTREE_BASE/PATH` env), `:257-327` (remove + discovery/refresh in one transaction); `packages/core/src/worktree/strategies.ts`, `packages/core/src/worktree/directory.ts` (`canonical`).
- **What it does:** Worktrees are first-class, discoverable, strategy-backed (git or others) locations with lifecycle commands and ownership tracking — exactly the "worktree-per-task" pattern in PLAN §27.2.
- **Adopt →** **S1.1, S2.3, S2.10**: expose create/remove/refresh as standard tool-envelope tools with confirm tier; adopt canonical-path + ownership rules so two surfaces cannot fight over a worktree; use a per-task worktree as our strongest subagent isolation primitive.

### 10. Background shells and subagents as recoverable jobs with notify-on-finish
- **Refs:** `packages/core/src/tool/plugin/subagent.ts:19-28` (background result: "You will receive a `<subagent>` message when it goes idle… DO NOT sleep, poll, proactively check"), `:30-57` (input schema + tool description), `:118-138` (depth limit, agent-mode guard, permission assert **before spawn**), `:171-217` (child session, model precedence explicit > agent > parent, register recovery job); `packages/core/src/session/subagent-job.ts:17-52` (job `background: true`, `notificationID`, global context so jobs outlive the launching plugin); `packages/core/src/session/background.ts:26-83` (`outstanding()` walks parent/child chains and includes a child's own background shell); `packages/core/src/persistent-pty/` + `packages/core/src/job/recovery.ts` (tree); `packages/core/src/model-retry.ts:11-36,43-61,65-134` (bounded `retry-after` ≤15 min, unreachable-network 1 h budget, max 3 timeout retries, one-shot retry only when provider didn't run).
- **What it does:** Subagents always run in background as recoverable jobs; parents are notified on idle instead of polling; descendant background work keeps a session "not done"; model calls survive outages with deliberately bounded backoff (a hostile `retry-after` cannot stall a call for hours).
- **Adopt →** **S2.3, S5.3, S3.7**: copy the job+notification model and the "don't poll" instruction; reuse the retry policy (bounded retry-after, separate unreachable budget) for our model router; treat kill/restart recovery as a first-class job property.

### Also audited (smaller mechanisms, same commit)

| Mechanism | Source ref | What it does | Adopt → |
|---|---|---|---|
| LSP-in-the-loop diagnostics **(v1 only; removed in v2)** | `packages/schema/src/config/lsp.ts:6-19`, `packages/schema/src/lsp-event.ts:5` | v2 keeps only LSP config schema + `lsp.updated` event; no in-process LSP client. v1 had the implementation — copy the *pattern*, not these files. | **S2.7** (VS Code `getDiagnostics` in loop), S1.5 |
| Multi-provider auth combinators + redacted secrets | `packages/ai/src/route/auth.ts:28-68` (`Credential`/`Definition` with `orElse`/`andThen`), `:70-101` (Redacted + config), `:119-150` (bearer/header/custom scheme) | Composable auth fallbacks; secrets are `Redacted` objects, never logged. | **S3.7, S8.6** |
| MCP dual transport + protocol revision negotiation | `packages/core/src/mcp/client.ts:14` (`StreamableHTTPClientTransport`), `:36,299` (stdio via location `Environment`), `:220-222` (revision `2026-07-28`), `:350-416` (initialize, roots deprecated, SEP-3415 events, elicitation), `:62` (fast reconnect) | stdio + Streamable HTTP with 2026-07-28 revision, elicitation/tasks, stale-session reconnect. | **S2.1** (note PLAN §28.1: do not build on Sampling/Roots/Logging/SSE) |
| ACP integration with diff-bearing permission cards | `packages/cli/src/acp/agent.ts:26-138` (full method set incl. `session/fork`, `set_mode`, steering extension at `:120-127`), `packages/cli/src/acp/permission.ts:39-74,83-115` | Editor protocol bridge; permission asks carry real diffs and once/always/reject options. | **S2.1, S0.9** (reviewability of confirm card) |
| Agent Skills (`SKILL.md`) progressive disclosure | `packages/core/src/skill.ts`, `packages/core/src/skill/discovery.ts:112,169-194` (atomic staging→backup→rename install), `.opencode/skills/*/SKILL.md` (8 shipped skills), `packages/core/src/config/plugin/skill-scan.ts` | Skills scanned/watched/hot-reloaded; Claude+AGENTS compatibility via plugin (above). | **S2.2, S9.10** (marketplace reads same format) |
| Bounded tool output | `packages/core/src/config/plugin/tool-output.ts:9-23` (`maxLines`, `maxBytes` from config) | Config-sourced caps on tool output retained in context. | **S1.4, S2.9** (stream budgets) |
| Hooks-as-runtime-policy + prompt-cache-safe instruction updates | `packages/core/src/plugin/hooks.ts:13-59`; CHANGELOG:704 (keyed `session.instructions.entry.put/list/remove` survive compaction/revert without an extra model turn) | Small instruction deltas instead of rewriting the system prompt. | **S2.2, S2.4** |

---

## 4. Anti-patterns to avoid (visible in issues / CHANGELOG / code)

1. **Serial breaking changes to public plugin/SDK contracts.** The CHANGELOG shows repeated hard breaks: v2.2.0 `@opencode/ai` root flipped to Promise API and `/promise` removed (`:708-719`), GitHub Action removed (`:721-723`), TUI plugins forced to OpenTUI 0.7 then **0.8** in consecutive releases (`:725-727`, `:436-438`); v2.1.0 renamed `providerState→native`, `resultState→native`, `providerContext→native.messages`, removed `session.compacted` (`:1163-1237`); v2.2.7 removed `ImageResponse.image` (`:126-128`). PLAN §27.3 #5 ("rewrite plugin APIs with no migration window") is confirmed. → Our S0.7 additive-only + one-version deprecation is the fix.
2. **Feature churn / removal-then-restoration.** "Restored the `todowrite` tool" (CHANGELOG:705, :1244), "Restored session archiving" (:1245), "Restored pasted text when undo brings back its placeholder" (:57). Repeated removal/re-add erodes trust. → For AI-Harness, deprecate with aliases; never remove a working surface without parity.
3. **Experimental-flag zoo.** Features ship behind `experimental.codemode_vm`, `experimental.background_compaction`, `experimental.media_blob`, `experimental.todo`, `experimental.subagent_depth`, `experimental.schedule`… and flags are renamed/removed across minors. → Keep a small, documented flag registry with expiry dates.
4. **Prompt-cache fragility.** Many fixes ("cache warming stops for a session when a ping rebuilds the cache", :161; "stabilized image trimming and instruction baselines to improve prompt-cache reuse", :1257) show how easily compaction/instruction edits bust caches. → Our S2.4 must keep instruction deltas and `keepID` references, not rewrite the prompt.
5. **Unbounded discovery / watches can DoS startup.** "Bounded skill discovery and file watches, and tolerated filesystem permission denials during sandboxed startup" (:1260); a shell permission scanner could freeze on repeated zsh `q` flags (:254). → Bound all filesystem discovery and never let a parse loop block the loop.
6. **Security regressions found post-ship.** Webfetch and Code Mode `fetch` initially accepted URLs with embedded credentials and matched permission rules against fragments; redirects were not all re-checked (:247). Bun runtime needed emergency security upgrade (:1045-1053). `external_directory` permission had to be added for `session_move` (:248). → Our egress DLP (S1.4) must check credentials-in-URL, fragments, and *every* redirect from day one.
7. **Silent failures in long-running paths.** Repeated fixes for long TUI/shell calls hitting Bun's 5-minute fetch deadline (:1161, :1252), Code Mode async jobs "interleaving in the middle of statements and losing writes" (:1153), background compaction stopping a Claude Agent session mid-tool-call (:153), MCP token refresh signing users out when multiple clients refresh (:138, :149). → Durable job boundaries and cancellation semantics need tests, not just code.
8. **Default-access mistakes elsewhere in the field** (from PLAN §19/§21, adopted as our design rules): T3's "new threads default to Full access" and unchained audit logs. OpenCode's own defaults are better (deny-all for unknown agents — `permission.ts:20`), but its audit/event chain is not hash-chained — our tamper-evident chain (S0.4) remains a moat.

---

## 5. Sources

**Repo/API metadata (fetched 2026-11-17):**
- `https://api.github.com/repos/sst/opencode` and `https://api.github.com/repos/anomalyco/opencode` (same repo id 975734319)
- `https://api.github.com/repos/anomalyco/opencode/commits/dev` (HEAD `0e447fc…`)
- `https://api.github.com/repos/anomalyco/opencode/branches` (incl. `2.0`)
- `https://api.github.com/repos/anomalyco/opencode/git/trees/af9b7b99e0fb33520b1752e8a23b60af95828d03?recursive=1` (12,619 entries; full response saved locally)
- Raw sources: `https://raw.githubusercontent.com/anomalyco/opencode/0e447fce1623813d8af9bd5d1b511e8e2b4f4f81/<path>` — 40+ files downloaded to `C:\Users\ASUS\AppData\Local\Temp\opencode\src\` and read directly (paths and line numbers cited above).

**Primary source files read (all at pinned commit):** `packages/codemode/src/{codemode,code,vm}.ts`; `packages/core/src/codemode/{catalog,instructions}.ts`; `packages/core/src/{rpc,bus,snapshot,git,permission,plugin,skill,model-retry,event-logger}.ts`; `packages/core/src/plugin/{hooks,compatibility-bridge,mcp-codemode-defaults}.ts` (compat file is `config/plugin/compatibility.ts`); `packages/core/src/session/{compaction,revert,background,subagent-job,subagent-depth,shell}.ts`; `packages/core/src/tool/plugin/subagent.ts`; `packages/core/src/worktree.ts` + `worktree/strategies.ts`; `packages/core/src/mcp/client.ts`; `packages/protocol/src/client.ts`; `packages/server/src/{routes,event-feed}.ts`; `packages/ai/src/route/auth.ts`; `packages/cli/src/acp/{agent,permission,compaction}.ts`; `packages/schema/src/{config/lsp,lsp-event}.ts`; `CHANGELOG.md`; `AGENTS.md`.

**Internal planning docs read:** `docs/PLAN.md` §15 (525-554), §17-21 (574-700), §27 (911-925), §28 (929-955); `docs/IMPLEMENTATION-PHASES.md` §2-4 (46-187) and S3.7/S8.6-8.7 mappings.

**Note on negative findings:** no `lsp` implementation, no `failover`-named module, and no published plugin-migration shim for the V1 plugin API were found in the v2 tree — the V1 plugin break is evidenced by the CHANGELOG breaking-change sections and the `packages/core/src/v1/config/*` + `database/migration/20260910120000_clear_v1_session_permission.ts` compatibility code instead.
