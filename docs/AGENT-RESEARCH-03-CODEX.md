# Agent Research 03 — Deep Source Audit: OpenAI Codex (`openai/codex`)

**Date:** 2026-11-17 · **Auditor:** research subagent · **Method:** GitHub REST API, raw file fetches, published Markdown docs (`learn.chatgpt.com`), and a shallow blobless sparse clone at `C:\Users\ASUS\AppData\Local\Temp\opencode\codex-src` (HEAD `c08843a5bd16de5524129b2f4270490447991984`). All `codex-rs/…` paths and line numbers below are from that HEAD unless stated otherwise.

**Local priors read first:** `docs/PLAN.md` §6 (safety), §11 (tech stack), §15 (reuse), §19 (T3), §27 (coding-harness race), §27.3 (mistakes), and `docs/IMPLEMENTATION-PHASES.md` (all Sx.y steps).

---

## 1. Repo identity & freshness

| Field | Value (GitHub API, 2026-11-17) |
|---|---|
| Repo | `openai/codex` — "Lightweight coding agent that runs in your terminal" |
| Created / last push | 2025-04-13 / **2026-11-17T17:30:29Z** (pushed *today*) |
| Stars / forks / open issues | 135,652 / 21,506 / 27,678 |
| License / language / default branch | Apache-2.0 / Rust / `main` |
| HEAD commit | `c08843a5` "Skip redundant history materialization on flush (#62085)" (2026-11-17) |
| Build system | Cargo + Bazel (both first-class; `.bazelrc`, `MODULE.bazel`, `defs.bzl`, `justfile`) |
| Top-level layout | `codex-rs/` (139 crates), `codex-cli/`, `sdk/`, `docs/` (13 mostly *stub* docs), `tools/`, `patches/`, Bazel helpers |

**Freshness signals (today's commits, all on 2026-11-17):** Guardian v2 score caches isolated by policy revision (#62083); policy-checked dispatch for bundled App hooks (#62080); device-code login for config servers (#62079); turn-interrupt reply ordering (#62066); Linux sandbox stopped masking system SSH drop-ins (#62058); `spawn_agent` model shortlist cap raised 5→16 (#62051).

**Nov 2026 product updates** (`learn.chatgpt.com/docs/whats-new.md`, fetched 2026-11-17): **GPT-6.2 Astra and Sol launched today (Nov 17)** in ChatGPT Work, Codex and API; Nov 9–13 custom instructions for dots; Nov 2–6 improved Windows task-resume reliability. Source tree already contains `guardian-v3` (`ext/guardian-v3/src/{housekeeper,evidence,selection,delivery,retrieval_tools}.rs`) and v3 schema families — i.e. the open repo leads the public changelog.

**Docs caveat:** in-repo `docs/sandbox.md`, `docs/execpolicy.md`, `docs/agents_md.md` are one-line stubs redirecting to `developers.openai.com`; the authoritative content lives in `learn.chatgpt.com/docs/*.md`. This itself is an anti-pattern (§4) and reinforces our "docs/ updated with each phase" cross-cutting obligation (`IMPLEMENTATION-PHASES.md` §12).

---

## 2. Architecture (what the code actually is)

Codex is a **Rust workspace of ~139 crates** around one agent core, with five distinct surfaces and a deliberately layered trust stack:

```
surfaces:  codex-cli/TUI  ·  app-server (JSON-RPC 2)  ·  exec (headless)
           ·  app-server-client/daemon  ·  SDK (TS/Python)
core:      codex-rs/core  (agent loop, session, compaction, MCP, Guardian, goals hooks)
tools:     codex-rs/tools, apply_patch, exec-server (process + PTY + fs sandbox glue)
sandbox:   linux-sandbox (bubblewrap+seccomp)  ·  sandboxing (landlock.rs, windows*.rs)
           windows-sandbox-rs (ACLs, capability SIDs, restricted tokens, WFP, job objects)
           mxc-sandbox + windows-sandbox-service (native MXC / elevated provisioning)
policy:    execpolicy (Starlark .rules)  ·  protocol/src/approvals.rs + config_types.rs
           hooks (14 lifecycle events)  ·  ext/guardian-v{2,3} (async/sync reviewers)
state:     state/ (SQLite migrations, thread_goals), rollout JSONL, history/ (compaction checkpoints)
protocol:  protocol/ (core events), app-server-protocol/ (v1/v2 generated JSON+TS schemas)
ext:       ext/goal, ext/skills, ext/web-search, ext/code-mode, ext/guardian-reviewer, …
```

**Key architectural idea to steal:** the *policy decision*, the *OS enforcement*, and the *approval UI* are three separate layers with typed boundaries. `AskForApproval` (approval policy) never widens the sandbox, and `PermissionProfile` (OS boundary) never decides when to ask. This is exactly PLAN.md §6.1's single decision pipeline, but with the OS-enforcement axis implemented at kernel/ACL level rather than as a wrapper convention.

The **two-axis model**, verbatim from `learn.chatgpt.com/docs/sandboxing.md`:

> "Sandboxing and approvals are different controls that work together. The sandbox defines technical boundaries. The approval policy decides when the agent must stop and ask before crossing them."

OS axis (`sandboxing.md`, `permissions.md`): `read-only` / `workspace-write` / `danger-full-access` (legacy `sandbox_mode`) **or** the newer beta `PermissionProfile`s `:read-only` / `:workspace` / `:danger-full-access` with `:root`/`:minimal`/`:workspace_roots` filesystem maps (`read|write|deny`, most-specific-wins, `deny` > `write` > `read`) plus network domain allow/deny enforced by a local proxy (`features.network_proxy = true`, `proxy_url = http://127.0.0.1:3128`, SOCKS5 `:8081`, local/private-network guard on by default).

Approval axis (`codex-rs/protocol/src/protocol.rs:1184–1225`): `AskForApproval` = `UnlessTrusted` (untrusted projects; retired as selectable in docs), `OnRequest` (**default**; alias `on-failure`; "the model decides when to ask"), `Granular(GranularApprovalConfig)` with five boolean switches (`sandbox_approval`, `rules`, `skill_approval`, `request_permissions`, `mcp_elicitations` — lines 1210–1225), and `Never` ("failures returned to the model, never escalated"). Full access = `danger-full-access` + `never`; low-risk preset = `workspace-write` + `on-request`.

---

## 3. Top 10 mechanisms — file refs + adoption mapping

### 1. Two-axis sandbox model & permission profiles (incl. approval-policy axis)
**Evidence:** `docs/sandboxing.md`, `docs/permissions.md`, `protocol/src/protocol.rs:1184–1225`, `protocol/src/config_types.rs:297–301`, `config/src/permission_config.rs:84`, `config/src/config_toml.rs:878–909`, `sandboxing/src/{landlock.rs,windows.rs,windows_mxc.rs}`, `app-server/src/request_processors/windows_sandbox_processor.rs:338–344`.
**Mechanism:** OS-enforced boundary (`PermissionProfile`) × approval policy (`AskForApproval`) × *who* reviews (`approvals_reviewer = user|auto_review`). Policies merge across config layers; managed `allowed_permission_profiles` is deny-by-default (omitted profiles denied). `deny` globs are enumerated at process start (documented TOCTOU window). Fail closed: macOS Seatbelt refuses to run a command it cannot constrain; Windows `unelevated` refuses policies it cannot enforce.
**Adopt →** `S0.4` (single decision pipeline, tiers), `S1.2` (rule resolution chain), `S1.3` (tiered UX), `S1.4` (deny-by-default egress), `S1.8` (capability isolation as real boundary), `S5.3` (unattended = strictest profile); validates PLAN.md §6.1/§6.5.

### 2. `codex-linux-sandbox` internals — bubblewrap + seccomp (+ legacy Landlock)
**Evidence:** `codex-rs/linux-sandbox/README.md:10–14, 52–143`; `linux-sandbox/src/{seccomp.rs,bwrap.rs,bundled_bwrap.rs,launcher.rs,daemon_mounts.rs,managed_tcp.rs,tcp_namespace.rs}`; vendored C source at `codex-rs/vendor/bubblewrap/bubblewrap.c`.
**Mechanism:**
- bubblewrap is the default FS sandbox: `--ro-bind / /` then `--bind <root> <root>` for writable roots; `.git` (incl. resolved `gitdir:`), `.codex`, protected subpaths re-applied read-only; symlink / first-missing-component protected paths masked with `/dev/null`; glob deny entries expanded pre-launch (`rg --files --hidden --no-ignore` with internal globset fallback); overlapping split-policy entries applied in path-specificity order.
- Namespace isolation `--unshare-user`, `--unshare-pid`, `--unshare-net` (restricted network); fresh `/proc`.
- In-process `PR_SET_NO_NEW_PRIVS` + seccomp after FS view: restricted network blocks network syscalls; `AF_VSOCK` blocked for FS-only policies; **`io_uring` blocked in all modes** to prevent socket-family bypass.
- Fallback chain: system `bwrap` on PATH → bundled `codex-resources/bwrap`; WSL2 supported, WSL1 rejected. Legacy Landlock is retained in `seccomp.rs` but rejected for FS-restricted policies (cannot isolate app-server Unix sockets).
- **Transparent TCP / managed proxy:** private DNS maps ≤8,192 names into `198.18.0.0/15` and `fdc0:de00::/96`; TCP→UDS→TCP bridge to attributed proxy endpoints; 128 concurrent connections/bridge; sockets identified by both endpoints + inode; closing cancels pending approvals.
**Adopt →** `S1.4` (SSRF/metadata deny + proxy allowlist), `S1.8` (kernel-tier isolation), `S8.8` (offline/no-egress profile), `S4.2` ("never run agent elevated"). This is a concrete blueprint for our L4/L3.5 tiers in PLAN.md §6.4.

### 3. Windows sandbox — three implementations, "best Windows sandbox in the field"
**Evidence:** `learn.chatgpt.com/docs/windows/windows-sandbox.md`; `protocol/src/config_types.rs:297–301` (`WindowsSandboxLevel::{Disabled,RestrictedToken,Elevated}`); `windows-sandbox-rs/src/{cap.rs:19–26,88–104, token.rs:393–435, acl.rs, deny_read_acl.rs, wfp.rs, wrapper.rs:104,478–482, bin/command_runner/win.rs}`; `windows-sandbox-service/src/{machine_policy.rs:116, provisioning.rs}`; `core/src/windows_sandbox.rs`; `app-server-protocol/src/protocol/v2/windows_sandbox.rs:21,29`.
**Mechanism:** ordered selection `mxc` → `elevated` → `unelevated`:
- **MXC** (recommended): native Microsoft Execution Containers process isolation; no admin setup, no extra accounts, no firewall rules; requires Win11 24H2 build 26100.9278 / 25H2 26200.9278; `[features] prefer_mxc = true`; `windows.allow_mxc = false` managed block. Env-level offline controls; private-network destination checks removed but domain proxy rules retained.
- **elevated** (legacy fallback): dedicated *lower-privilege sandbox users*, filesystem ACL boundaries, **WFP firewall offline rule**, local policy changes, private desktop by default; commands run without admin rights; IT-led provisioning `codex sandbox setup --elevated --user 'DOMAIN\alice'`.
- **unelevated**: `CreateRestrictedToken`-style restricted token derived from current user + per-workspace/per-write-root **capability SIDs** (`cap.rs`), ACL-based FS boundary, environment-level offline controls; **weaker network isolation, no deny-read paths**; unsupported policies are refused rather than silently downgraded. Daemon refuses to start elevated ("shared clients must not inherit administrator privileges" — `app-server-daemon/src/backend/windows.rs:118–121`).
**Nuance vs PLAN.md §27.1:** "integrity levels" is our paraphrase; the code's actual ladder is `WindowsSandboxLevel` + MXC. Note the deny-read glob snapshot caveat (files created after process start stay readable).
**Adopt →** `S1.8` (real boundary/kernel tiers), `S4.2` (no elevated agent, gsudo per-verb), `S5.2` (scoped CLI creds), PLAN.md §6.4 L0–L3; mirror the *fail-closed when unenforceable* rule.

### 4. App-server JSON-RPC 2.0 protocol for IDEs (one service behind every surface)
**Evidence:** `learn.chatgpt.com/docs/app-server.md`; `codex-rs/app-server/README.md:1–794` (behaviour changelog); `app-server-protocol/schema/json/v2/*.json` (hundreds of typed requests/notifications), `app-server-protocol/schema/typescript/v2/*.ts`; `app-server-client/`, `app-server-daemon/`.
**Mechanism:** bidirectional JSON-RPC 2.0 (wire omits `"jsonrpc":"2.0"`); transports stdio (JSONL default), WebSocket (`ws://IP:PORT`, experimental), Unix socket, `off`; mandatory `initialize` → `initialized` handshake per connection; capabilities `experimentalApi`, `optOutNotificationMethods` (exact names), `requestAttestation`, `explicitGatewayOauth`; stable generated schemas per version (`codex app-server generate-ts|generate-json-schema`); primitives thread/turn/item; methods `thread/start|resume|fork|revert|compact/start|goal/*`, `turn/start|steer|interrupt`, `review/start`, `windowsSandbox/readiness|setupStart`, `hooks/list`, `item/permissions/requestApproval`, `config/batchWrite`, `fs/*`. Structured errors: `-32600` (invalid/shutdown/conflict with `data.reason`), `-32602` (invalid params), `-32603` (internal), `-32001` "Server overloaded; retry later" for bounded queues. WS auth: capability-token (file/sha256) or signed bearer; health probes `/readyz`, `/healthz` (Origin header → 403).
**Adopt →** `S0.7` (protocol/schema contracts, handshake, capabilities, additive-only versioning), `S2.7` (VS Code extension), `S5.4` ("every frontend + IDE plugin + SDK talks the same thing" per PLAN.md §11), `S9.8` (public SDK + deprecation policy).

### 5. Exec policy rules — Starlark `.rules`, prefix matching, tree-sitter command splitting
**Evidence:** `codex-rs/execpolicy/README.md:1–40`, `execpolicy/src/{parser.rs,policy.rs,rule.rs,decision.rs,amend.rs,sandbox_migration.rs}`, `cli/tests/execpolicy.rs`, `learn.chatgpt.com/docs/agent-configuration/rules.md`.
**Mechanism:** `prefix_rule(pattern=[…], decision="allow"|"prompt"|"forbidden", justification=…, match=[…], not_match=[…])` in side-effect-free Starlark; `match`/`not_match` are **inline unit tests validated at load time**; `host_executable(name, paths=[…])` constrains absolute-path resolution (exact-first-token precedence). **Most restrictive decision wins** when rules overlap. Shell wrappers: `bash -lc`/`sh -c` scripts made of plain words joined by `&& || ; |` are parsed with **tree-sitter and split into sub-commands**, each evaluated separately (`git add . && rm -rf /` → two commands, dangerous one blocked). If the script contains redirection, `$()`, env assignments, wildcards or control flow, Codex refuses to interpret it and evaluates the whole invocation conservatively. `codex execpolicy check --rules … -- <cmd>` emits JSON with strictest decision + justifications. Rules scanned across every active config layer; admin `requirements.toml` can enforce rules; `decision = "prompt-user"` creates a **managed human checkpoint** that session/persistent approvals cannot satisfy (`app-server/README.md:1–20`). Smart approvals propose prefix amendments; TUI "always allow" writes to `~/.codex/rules/default.rules`.
**Adopt →** `S1.2` (argv parsing + compound split + first-match/most-restrictive), `S0.4` (rule→tier), `S1.3` (prompt shows matched rule), `S1.7` (reviewability / meaningful oversight), `S9.4` (time-travel "why did you do that" via justifications).

### 6. Approval UI semantics + out-of-band Guardian auto-review
**Evidence:** `protocol/src/approvals.rs:34–54, 188–215, 272–412` (typed approval events); `core/src/guardian/*` (20 files), `ext/guardian-reviewer/src/*`, `ext/guardian-v2/src/async_scorer/*`, `ext/guardian-v3/src/*`; `learn.chatgpt.com/docs/sandboxing/auto-review.md`; `code-review.md`; `app-server/README.md:121–128`.
**Mechanism:**
- `ExecApprovalRequestEvent` (`:283–356`) carries `command`, `cwd`, `parsed_cmd`, `approval_id` (per-subcommand via execve intercept), `reason`, `network_approval_context`, `proposed_execpolicy_amendment`, `proposed_network_policy_amendments`, `additional_permissions`, and an explicit **`available_decisions` ordered menu**; `default_available_decisions()` (`:379–412`) derives approve / approve-for-session / "+ add rule" / abort. `ExecApprovalKind::HumanCommand` (`:275–281`) requires a fresh human decision; automated/reusable approvals are invalid. Network prompts offer network-policy amendments; permission prompts only approve/abort.
- Guardian risk/user-authorization enums, review reasons (`Policy, FreshRequired, MissingScore, StaleScore, InvalidScore, IncompatibleCompaction, ElevatedRisk, ScoringFailure, AuthorizationChanged` — `:203–215`) make cached classifier approvals *policy-revision-scoped* (today's #62083 commit).
- Auto-review: separate reviewer agent sees compact transcript + exact action, **not hidden reasoning**; can do read-only checks; denial returns rationale + "do not workaround" instruction; circuit breaker after **3 consecutive denials or 10 of the last 50** reviews per turn (1 denial for cyber models); `/approve` allows exactly one retry of one recorded denial, still re-reviewed; `approvals_reviewer = auto_review`; reviewer policy template + bundled `policy.md`; managed `guardian_policy_config` wins; `auto_review.circuit_break_action = "strict"` surfaces `tooManyDenials`.
- `/review` workflow: dedicated reviewer, scopes base-branch/uncommitted/commit/custom instructions, `review_model` separate from session model, detached or inline delivery, inline comments are *guidance* (not auto-applied), no auto-merge, normal sandbox/approval applies if asked to fix.
**Adopt →** `S0.9` (meaningful oversight / reviewability), `S1.7` (consent ledger, fatigue KPIs, counterfactual cards, graduated handover), `S2.14` (guard/provenance evolution propose→verify→activate), `S1.3` (decision menus), `S9.4`. **Guardrail lesson:** classifier is a helper, not the boundary — keep egress + sandbox as the wall (PLAN.md §24.3).

### 7. Hooks as runtime policy — trust-by-hash + managed hooks
**Evidence:** `codex-rs/hooks/README-less crate`: `hooks/src/{types.rs:14–72, events/*, engine/{dispatcher,matcher,command_runner,mcp_runner,plugin_trust,output_parser}.rs, config_rules.rs}`; `hooks/schema/generated/*.json` (PreToolUse, PostToolUse, PermissionRequest, PreCompact, PostCompact, SessionStart/End, SubagentStart/Stop, Stop, Interrupt, UserPromptSubmit, ModelRequest); `config/src/requirements_layers/hooks.rs`; `learn.chatgpt.com/docs/hooks.md`.
**Mechanism:** 14 lifecycle events; `hooks.json` per layer (system `%ProgramData%\OpenAI\Codex\hooks.json`, user `$CODEX_HOME/hooks.json`, trusted project `.codex/hooks.json`) plus inline `[hooks]` in `config.toml` and plugin-bundled `hooks/hooks.json`. **Non-managed hooks must be reviewed and trusted against the hook's current hash; changed hooks are skipped until re-reviewed** (`/hooks` browser). Managed hooks from `requirements.toml`/MDM are policy-trusted and cannot be disabled; `allow_managed_hooks_only = true` ignores user/project/plugin hooks. `PreToolUse` can `deny` (with reason), `allow`+`updatedInput` (rewrite), or add context; `PermissionRequest` can allow/deny (any deny wins, no decision → normal prompt); `PostToolUse` cannot undo effects but can replace tool output; `Stop`/`SubagentStop` `decision:"block"` continues the turn; `PostModelRequest` reports `token_usage`. Output >~2,500 tokens is **spilled to disk** with a head/tail preview. Async hooks (max 8 concurrent) cannot block. `mcp_tool` hooks call an already-connected MCP server; plugin hook scripts require new review on change (hash of script, not just definition).
**Adopt →** `S0.7`/`S2.2` (hooks-as-runtime-policy from PLAN.md §27.2), `S1.7` (governance surfaces), `S2.2` (hostile-plugin suite), `S2.12` (sign/manifest/verify), `S8.2` (key lifecycle), `S9.3` (skill provenance). **Copy the hash-trust rule verbatim** — it neutralizes poisoned-hook updates.

### 8. Goals mode + token/cost accounting
**Evidence:** `codex-rs/ext/goal/src/{lib.rs:1–28, spec.rs:9–94, accounting.rs:20–120, runtime.rs, steering.rs, tool.rs}`, `ext/goal/templates/goals/{budget_limit,continuation,objective_updated}.md`, `state/migrations/0029_thread_goals.sql`, `app-server-protocol/schema/json/v2/ThreadGoal*.json`, `protocol/src/protocol.rs:2561–2607, 2741–2792`; `learn.chatgpt.com/docs/long-running-work.md`.
**Mechanism:** `/goal` creates a **persisted thread goal** (≤4,000 chars) via `get_goal`/`create_goal`/`update_goal` tools; `create_goal` only on explicit user/system request, `token_budget` only when explicitly requested; statuses `active|complete|blocked|paused`, with `blocked` requiring **≥3 consecutive goal turns** with the same blocker and `paused` only on explicit user request (`:46–83` — a built-in loop/termination discipline). Accounting tracks per-turn and descendant token deltas, wall-clock time, execution failures, empty turns, and budget-limit disposition (`accounting.rs:20–88`). `ThreadGoal` RPCs (`thread/goal/set|get|clear`, `tokenBudget`, `tokensUsed`, `timeUsedSeconds`) expose the same state to IDEs. `TokenUsage` splits `input_tokens`, `cached_input_tokens`, `cache_write_input_tokens`, `output_tokens`, `reasoning_output_tokens`, `total_tokens` plus provider rollout-budget units (`protocol.rs:2562–2581`), with helpers `cached_input()`, `non_cached_input()`, `add_assign()` (`:2741–2792`) and `TokenUsageRecord` carrying per-response/turn/thread totals (`:2585–2594`). Goals keep the same sandbox/approval policy; parallel goals require worktrees.
**Adopt →** `S0.3` (plan-as-file + todos), `S1.4` (per-task token/$ ceiling, breach → pause), `S1.5` (loop breaker / stuck monitor / finish gate — the 3-turn blocked rule is a ready-made heuristic), `S2.4` (file-first memory), `S5.3` (durable scheduler), `S2.8`/§12 (cost telemetry footer on every surface).

### 9. Remote compaction v2 — opaque encrypted compaction blob
**Evidence:** `core/src/{compact_remote_v2.rs, compact_remote_v2_attempt.rs, compact_remote_v2_installation.rs, compact_remote_v2_media.rs}`, `core/tests/suite/compact_remote.rs:304, 367, 421, 629, 786–818, 1117, 1245–1353, 1577, 1779`; `prompts/templates/compact/{prompt.md,summary_prefix.md}`; `context/compaction_summary.rs`; `state/src/model/thread_goal.rs` (goal survives compaction).
**Mechanism:** compaction is a **separate model call** whose result is an opaque item `{"type":"compaction","encrypted_content":"…"}` — tests assert the blob is carried into the next request's input verbatim (`compact_remote.rs:367, 818, 1031, 1742`). v2 adds background kickoff with candidate capture, interrupt/resume preserving the summary (`compact_resume_metadata.rs`), retained tool outputs, media/audio budgets, per-transport retry cap `MAX_REMOTE_COMPACTION_V2_STREAM_RETRIES = 2` (`compact_remote_v2.rs:98–101`), Pre/PostCompact hooks, and **preservation of encrypted delegated-task content across compaction** (`:1245–1353`). Reasoning `encrypted_content` is requested via `include: ["reasoning.encrypted_content"]` (`codex-api/src/endpoint/responses_websocket.rs:1219`). Guardian review reasons include `IncompatibleCompaction` and `StaleScore` — compaction invalidates cached approvals.
**Adopt →** `S2.4` (working-context compaction at 75% with artifact flush + rolling brief), `S1.6` (session resume across compaction), `S2.9` (durability contracts / snapshots / replay), `S2.14` (invalidate approvals on compaction), `S1.9` (benchmark CI for compaction fidelity). Treat the blob as opaque and integrity-bound; never re-interpret its plaintext.

### 10. AGENTS.md layered discovery + MCP as a tool bus
**Evidence:** `learn.chatgpt.com/docs/agent-configuration/agents-md.md` (in-repo `docs/agents_md.md` is a stub); `codex-rs/exec-server/src/sandbox_integrity/AGENTS.md`, `rollout-trace/AGENTS.md`, `tui/src/bottom_pane/AGENTS.md`, `code-mode-host/AGENTS.md`; `core/src/{mcp.rs,mcp_tool_call.rs,mcp_tool_exposure.rs,mcp_tool_approval_templates.rs}`; `app-server/README.md:147–166, 219–236, 553–589`.
**Mechanism (AGENTS.md):** instruction chain built once per run/session. Global layer: `~/.codex/AGENTS.override.md` else `~/.codex/AGENTS.md` (first non-empty wins). Project layer: walk from Git root down to cwd; per directory `AGENTS.override.md` → `AGENTS.md` → fallback names from `project_doc_fallback_filenames`; at most one file per directory; concat root→cwd (closer wins); skip empty; stop at `project_doc_max_bytes` (default **32 KiB**). `## Code Review Rules` section in the nearest AGENTS.md configures reviewer criteria. `CODEX_HOME` switches profiles; loaded sources are returned in `thread/start|resume|fork` as `instructionSources` and auditable via TUI log.
**Mechanism (MCP):** hosted `codex_apps` MCP server with opt-in `[features] codex_apps_mcp_2026_07_28 = true` (2026-07-28 protocol; legacy default); third-party servers via `mcpServer/*` RPCs, OAuth login, elicitation, tool annotations→approval, reserved `_meta` keys (`x-codex-turn-metadata`, `threadId`, `codex/connectionLease`) rejected from client resource reads; `mcp_tool` hooks on already-connected servers; `required` MCP server failure aborts `thread/start|resume` (fail-closed).
**Adopt →** `S2.4` (file-first project memory; our root ≤30-line rule from PLAN.md §8), `S2.1` (MCP 2026-07-28 dual transport), `S2.2` (pinning + description scanning + reserved metadata), `S9.4` (instruction-source transparency), `S2.14`.

---

## 4. Anti-patterns — documented and observed

**A. Silent model swaps / ignored config — the headline mistake (PLAN.md §27.3 #3).**
- **Issue #32692** (open, created 2026-07-13, labels `bug`/`CLI`/`config`): "Codex CLI always starts with gpt-5.4 model even though different model is specified in config.toml." v0.144.1, Enterprise/ChatGPT sign-in; `codex doctor` reports `model gpt-5.4` despite `model = "gpt-5.6-sol"` in `~/.codex/config.toml`; `/model` works but shouldn't be needed; user deleted `~/.codex` and reinstalled — persisted.
- **Issue #56819** (open, created 2026-10-25, labels `bug`/`CLI`/`custom-model`/`config`): mid-turn model change reported as applied (`thread_settings_applied`, UI shows new model) but **166 subsequent requests still went to the old plan-billed model for ~70 minutes**, draining quota. Explicitly cross-references **#32612** (access-control changes not applied to a running turn) and **#48949** (model-switch/compaction interaction). The reporter's ask: either apply the change at next request *or say plainly "applies after this turn completes"*.
- Related retirement churn: GPT-5.5 retired from Codex **Oct 14, 2026** and GPT-5.4 retired **Aug 31, 2026** (docs give notices, but stale workspace defaults/custom agents/scheduled tasks must be manually updated). `untrusted` approval policy was retired as selectable; stale configs need migration.
- **Our guard:** model/provider config is *effective-config visible* in the prompt and receipts; model changes mid-turn must either take effect at the next model call or emit an explicit deferred-switch notice with the still-effective model; never report success while the old model bills.

**B. Network policy footgun.** `permissions.md`: `network.enabled = true` with `features.network_proxy` **off** means commands have *direct, unrestricted* network access — profile domain rules are silently unenforced. Only "network on + proxy on" filters. Our egress DLP (PLAN.md §28.2 #7) must fail closed if the enforcing proxy is down, not fall back to direct egress.

**C. Deny-read glob TOCTOU.** `permissions.md` and `windows-sandbox.md` document that deny-read glob matches are snapshotted when each sandboxed process starts; files created later stay readable to that long-running process. Do not claim glob denies are durable; re-enumerate on new processes and keep secrets out of writable roots.

**D. Approval-rule irony / rule elision.** PLAN.md §24.1 (arXiv:2610.22987): adversarial context elicited broad allow-rules in 58% of targets, 81% silently completed. Codex "Smart approvals" proactively proposes `prefix_rule` amendments — convenient but exactly the attack surface; mitigations in Codex are admin-enforced `requirements.toml` rules, `decision = "prompt-user"` fresh-human checkpoints, and justification surfacing. Our counterpart: S1.7 reviewability + expiring training-wheel entries + two-key rule changes (S1.8).

**E. Hooks are a guardrail, not a boundary.** `hooks.md`: cloud MCP `PreToolUse` errors/timeouts/malformed responses *can let the tool run*; background hooks cannot block; `PostToolUse` cannot undo effects; "Some specialized tool paths can opt out of the default hook path." Trust-by-hash is the mitigation, but the boundary must remain the sandbox. Same lesson as PLAN.md §24.3 (Anthropic auto mode blocked its own cleanup; classifier ≠ wall).

**F. "No native rewind" needs nuance.** PLAN.md §27.1 says Codex has no native rewind. Source confirms **conversation-level** `thread/revert` (paginated threads only) and `thread/fork`, and `thread/rollback` was *removed* from the app-server API (`app-server/README.md:572–581`); there is no file/code rewind equivalent to Cursor's paired conversation+files rewind (§27.1). Our checkpoint/undo ledger (S1.1) remains differentiated.

**G. Docs drift.** In-repo docs are stubs; review/auto-review behaviour is documented against tagged `rust-v0.167.0` templates while `main` is already at Guardian v3. Keep `docs/` generated from code or CI-checked against schemas (our S0.7/S9.8).

**H. Additional Codex footguns worth avoiding:** `dangerously_allow_all_unix_sockets` / `allow_local_binding` / `dangerously_allow_non_loopback_proxy` broad escape hatches; `--dangerously-bypass-hook-trust`; `thread/shellCommand` and `process/spawn` explicitly run **outside** the sandbox (full access) — clients must expose them only for explicit user actions; non-loopback WebSocket listeners allow unauthenticated connections by default during rollout.

---

## 5. Sources

**Local:** `D:\projects\ai-harness\docs\PLAN.md` (§§6, 11, 15, 19, 27), `docs\IMPLEMENTATION-PHASES.md`; shallow clone `C:\Users\ASUS\AppData\Local\Temp\opencode\codex-src` @ `c08843a5` (sparse: `codex-rs/`, `docs/`, `AGENTS.md`).

**GitHub API / raw (fetched 2026-11-17):**
- `https://api.github.com/repos/openai/codex` · `/git/trees/main?recursive=1` · `/commits?since=2026-11-01T00:00:00Z&per_page=100` · `/issues/32692` · `/issues/56819`
- `https://raw.githubusercontent.com/openai/codex/main/{README.md,docs/sandbox.md,docs/execpolicy.md,docs/agents_md.md}`

**Code files cited (under `codex-rs/` at HEAD):** `protocol/src/{approvals.rs,protocol.rs,config_types.rs}`; `execpolicy/{README.md,src/*}`; `linux-sandbox/{README.md,src/*}`; `sandboxing/src/{landlock.rs,windows.rs,windows_mxc.rs}`; `windows-sandbox-rs/src/*`; `windows-sandbox-service/src/*`; `core/src/{windows_sandbox.rs,compact.rs,compact_remote_v2*.rs,guardian/*}`; `ext/{goal,guardian,guardian-reviewer,guardian-v2,guardian-v3}/src/*`; `hooks/{src/*,schema/generated/*}`; `app-server/{README.md,src/*}`; `app-server-protocol/{schema/json/v2/*,schema/typescript/v2/*,src/protocol/v2/*}`; `app-server-client/`, `app-server-daemon/`; `core/tests/suite/compact_remote.rs`; `core/tests/windows_sandbox.rs`.

**Official Markdown docs (fetched 2026-11-17; `learn.chatgpt.com` Markdown twins indexed at `https://developers.openai.com/codex/llms.txt`):**
`docs/sandboxing.md` · `docs/permissions.md` · `docs/permission-modes.md` · `docs/windows/windows-sandbox.md` · `docs/agent-configuration/rules.md` · `docs/agent-configuration/agents-md.md` · `docs/app-server.md` · `docs/hooks.md` · `docs/sandboxing/auto-review.md` · `docs/long-running-work.md` · `docs/code-review.md` · `docs/whats-new.md` · `docs/extend/mcp.md` (index entry) · `docs/agent-approvals-security.md` (index entry) · `docs/changelog` (index entry).
