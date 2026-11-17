# Agent Research 02 — OMP / "Oh My Pi" (`can1357/oh-my-pi`)

*Research date: 2026-11-17. Method: GitHub REST search + raw-file fetches of ACTUAL source/docs at pinned commit `2c18786227e28e99d314735e6e863dbbb56b0b65`; Google websearch integration returned no results for all queries, so discovery and verification were done via the GitHub API and raw GitHub content. GitHub API became rate-limited (`403` / `API rate limit exceeded`) partway through, so tag/release, contributor-count and upstream star-count were not retrievable; every other claim below is from a fetched source file or an API response captured before the limit. No claims are inferred from marketing copy alone where source was available.*

---

## 1. Identity resolution + freshness

### 1.1 What it actually is

| Field | Verified value |
|---|---|
| Canonical repo | `https://github.com/can1357/oh-my-pi` (homepage `https://omp.sh`) |
| CLI / brand | `omp`; "A coding agent with the IDE wired in. Built by Stencil Labs." |
| Owner / people | `can1357` = **Can Bölük**; primary recent committer `H4vC` (signature name "Brit"); company **Stencil Labs, Inc.** (`stencil.so`). License line: © 2025 Mario Zechner · © 2025–2026 Can Bölük · © 2026 Stencil Labs, Inc. |
| **Relationship to "Pi"** | **Explicit fork** of `https://github.com/badlogic/pi-mono`, the Pi coding agent by Mario Zechner (`@mariozechner`). README states this in the hero, in "The Pi _you love_, with batteries included", in §17, and in "Philosophy": *"omp is a fork of pi-mono … extended with a batteries-included coding workflow."* So: **Pi is the upstream engine; OMP is a productized, Rust-heavy fork**, not a config wrapper around Pi and not a renamed small project. |
| Maturity | **Large and mainstream, not obscure.** 39,290 stars · 4,501 forks · 39,290 watchers · **4,883 open issues** · repo size ~825 MB. Topics include `ai-agent`, `ai-coding-agent`, `mcp`, `multi-provider`, `tui`, `rust`, `typescript`, `bun`. |
| License | **MIT** at top level; vendored third-party code keeps its own licenses (`THIRD-PARTY-NOTICES.txt` per shipped component; `crates/pi-builtins/LICENSE`). |
| Languages / scale | TypeScript (Bun runtime) + **~80,000 lines of Rust** across 6–8 crates; 60+ LLM providers, 31 built-in tools, 14 LSP ops, 28 DAP ops, 17 TS workspace packages. |
| Distribution | `curl -fsSL https://omp.sh/install | sh` (SHA-256 check on prebuilt binary from `build.stencil.so`), Homebrew tap, `bun install -g @oh-my-pi/pi-coding-agent`, Nix flake, mise, PowerShell `irm … | iex`. |

### 1.2 Freshness (pinned)

- Repo created **2025-12-31**; last push **2026-11-17**.
- Latest commit on `main`: **`2c18786227e28e99d314735e6e863dbbb56b0b65`**, authored/committed **2026-11-17T18:04:51Z** by `H4vC`, message *"style(natives): rewrap the wayland doc comment for rustfmt"*. **Verification: `unsigned`** (relevant to §4 supply-chain analysis).
- CI is active (`.github/workflows/ci.yml`, ~65 KB; Bazel + Bun + Nix workflows; PR gate `bun run check`). PRs recently opened to everyone after a former "vouch" requirement.
- **Verdict on obscurity:** the premise "small/obscure/renamed" is **false**. This is one of the largest open coding-agent harnesses in the field, with a satellite ecosystem (third-party `pi-desktop`, `oh-my-pi-gui`, `pi-automode`, `memorix`, `agent-vision-toolkit`, `awesome-ai-plugins` all list Pi/OMP support). The *honest* caveat is that it is a **fork**, so much of what looks novel at the surface is inherited from Pi; §3 separates genuinely OMP-specific mechanisms from inherited/derivative ones.

### 1.3 Closest alternatives (resolved, not substituted)

Since the canonical repo was found, no substitute audit was needed. Adjacent repos surfaced during discovery, for orientation only: `badlogic/pi-mono` (upstream), `FaqFirebase/pi-desktop` and `nornzach/oh-my-pi-gui` (third-party GUIs), `czottmann/pi-automode`, `AVIDS2/memorix` (cross-agent memory via MCP).

---

## 2. Architecture

### 2.1 Boot & execution surfaces

Source: `packages/coding-agent/DEVELOPMENT.md` (the authoritative developer map; it explicitly says the old long architecture walkthrough was deleted because it drifted — a good docs-discipline signal).

```
process argv
  → src/cli.ts (runCli)          [worker-host dispatch + Bun version guard]
  → src/commands/* + src/cli/     [per-command adapters]
  → src/main.ts (runRootCommand) [theme / settings / model registry / session opts]
  → createAgentSession(...)      [src/sdk.ts → AgentSession]
       ├─ InteractiveMode        [TUI event loop]
       ├─ runPrintMode           [one-shot text/json]
       └─ runRpcMode             [JSONL stdin/stdout server]
```

**Four entry points, one engine:** interactive TUI; `omp -p` one-shot; Node/TS **SDK** (`ModelRegistry`, `SessionManager`, `createAgentSession`, `discoverAuthStorage`); **RPC** (`--mode rpc`, NDJSON commands in / response+event frames out, `prompt_result` terminator, `--mode rpc-ui`, `--no-ui`); and **ACP** (`omp acp`, Agent Client Protocol over JSON-RPC for Zed/editor control, with writes gated by `session/request_permission`). `cli.ts` doubles as a **worker host** for hidden `__omp_worker_*` selectors. Subagents can run on worker threads (see §3.3).

### 2.2 Package / crate topology

**TS monorepo (17 packages):** `pi-ai` (multi-provider streaming client), `pi-catalog` (model DB), `pi-agent-core` (agent runtime), `pi-coding-agent` (CLI + SDK), `pi-tui` (differential renderer), `pi-natives` (N-API bindings), `pi-wire` (collab protocol), `pi-mnemopi` (SQLite memory), `snapcompact` (bitmap-frame context compression), `omptype` (ArkType-compatible schema validation), `metaharness` (benchmark runners + Harbor storage + REST/SSE dashboard), `browser-relay`, `collab-web`, `omp-stats`, `utils`, `typescript-edit-benchmark`.

**Rust crates (~80k LoC, in-process via N-API, libuv pool):** `pi-shell` (embedded **bash 5.3 engine**, persistent sessions, ~38k LoC), `pi-natives` (~25k; desktop/AX/screenshot/input, grep, text width, snapcompact, kitty keys, ast, diff, PTY, highlight, task, glob, fd, clipboard, tokens, HTML→MD), `pi-walker` (~11.4k parallel walker), `pi-iso` (~3.3k workspace isolation: apfs/btrfs/zfs/reflink/overlayfs/projfs/rcopy), `pi-ast` (~2.9k tree-sitter + ast-grep), `pi-voice`, plus `pi-edit` (hashline patch engine) and `pi-builtins` (58–67 in-process coreutils: ls, sed, jq, awk, find, diff …).

**Design thesis** (README "Every tool, _benchmaxxed_" and "Unapologetically native"): link ripgrep/glob/find/bash into the process so hot paths have **no fork/exec** and behave identically on macOS/Linux/**Windows without WSL**. The README publishes model-level edit-format deltas (e.g. Grok Code Fast 1 6.7%→68.3%; Grok 4 Fast −61% tokens) attributed to edit-format and prompt tuning, with a blog post (`blog.can.ac/2026/02/12/the-harness-problem/`) — these are vendor-reported, not independently reproduced here.

### 2.3 State, context and policy layers

- **Session model:** JSONL session **tree** with branching (`session/`, `docs/session.md`, `docs/session-tree-plan.md`, `docs/tree.md`), export/share/fork/resume, `/fresh` provider-stream reset.
- **Context assembly:** `session/session-context.ts`; file-first discovery of AGENTS.md/rules/skills/MCP from **8 ecosystems** (`.claude`, `.cursor`, `.windsurf`, `.gemini`, `.codex`, `.cline`, `.github/copilot`, `.vscode`) with no migration script (`docs/context-files.md`, `docs/rulebook-matching-pipeline.md`).
- **Policy spine:** canonical `Rule` shape + name-based precedence across providers (native 100 → omp-plugins 90 → agents 70 → cursor/windsurf 50 → cline 40 → github 30 → builtin-defaults 1). Two buckets: passive rulebook rules (system prompt + `rule://`) and **TTSR** active stream rules.
- **Tool layer:** `tools/index.ts` declares `BUILTIN_TOOLS` / `HIDDEN_TOOLS`; one registry for built-ins, custom tools, MCP tools and "device" tools behind `xd://` (disabled by default; `read xd://` lists them). 16 internal URL schemes (`pr://`, `issue://`, `agent://`, `skill://`, `ssh://`, `conflict://`, `xd://`, `memory://`, `rule://`, `omp://`, …) resolve transparently inside every FS-shaped tool — "PRs are paths."
- **Trust boundary (explicit in README "Project inputs and trust"):** opening a repository loads project settings, extensions, hooks, tools, commands, skills, rules and project MCP **by design**. Narrowing flags (`--config` overlay with `mcp.enableProjectConfig: false`, `--no-extensions`, `--trusted-extension <abs path>`, `--no-tools`) exist but "narrow inputs without changing the repository-trust model." This is the single most important architectural difference from AI-Harness's safety-first posture (see §4).

---

## 3. Top mechanisms — file refs, novelty assessment, adoption mapping

### 3.1 Agent loop with live steering, pause gate and provider-dialect recovery

**Where:** `packages/agent/src/agent-loop.ts` (166 KB). `runLoop()`/`runLoopBody()` at ~L1115/L1233; **outer loop** at L1368 (queued follow-ups continue the run), **inner loop** at L1372 (`hasMoreToolCalls || pendingMessages.length > 0`); `resolveToolConcurrency()` at L119 (`shared`|`exclusive`, throwing per-call resolver fails closed to `exclusive`); tool execution gate L1657–L1710; deadlines via `AbortSignal.any` + max-timer chunking L1255–L1258. Supports `packages/agent/src/live-steering.ts` (steer mid-turn, including "interruptible" tools polled at 250 ms), `pause.ts` (`agentPauseGate`, turn-boundary park), `speculative-execution.ts` (`SpeculativeOperationCoordinator`), and recovery for GPT-5 **Harmony** and DeepSeek **DSML** tool-call markup leaking into text (bounded retries: 2 / 2 / 3 / 8 for different loops). Emits typed OTel spans via `telemetry.ts` (`startInvokeAgentSpan`, `startChatSpan`, `startExecuteToolSpan`, `OmpGenAIAttr`) with a real **OTLP exporter** (`packages/coding-agent/src/telemetry-export-otlp.ts`, `telemetry-export.ts`, `telemetry-settings.ts`).

**Novelty:** loop skeleton is industry-standard (and upstream Pi). Genuinely well-executed here are (a) mid-token live steering + interruptible tool wait, (b) bounded *provider-protocol leak* recovery as a first-class loop concern, (c) production OTLP spans with an internal attribute layer — notable because AI-Harness's own engineering blind-spot audit concluded GenAI semconv is "Development"-only and "OTel-compatible is not a stable wire contract." OMP appears to have solved the adapter-layer problem; **verify its opt-out semantics before copying**.

**Adopt →** `S0.3` (steerable interrupt queue, bounded retries, failure ladder), `S1.5` (loop breaker / finish gate — OMP's bounded-retry caps are a concrete pattern), `S0.7`, and **BLIND-SPOTS-03-ENGINEERING #1** (pinned internal metric names + adapter over unstable `gen_ai.*` — OMP's `OmpGenAIAttr` is a reference implementation to read line-by-line).

### 3.2 Time-Traveling Stream Rules (TTSR) — stream-time policy injection

**Where:** `docs/ttsr-injection-lifecycle.md`; implementation `packages/coding-agent/src/session/ttsr-coordinator.ts`, `ttsr-outputs.ts`, `capability/` (`rule.ts`, `rule-buckets.ts`), `prompts/system/ttsr-{interrupt,warning,tool-reminder,prose-notice}.md`.

Rules declare `condition` (regex), `astCondition` (ast-grep patterns), or `question` (judged), plus `scope` (`text`/`thinking`/any tool/named tool with path globs), `interruptMode` (`always|prose-only|tool-only|never`), `repeatMode` (`once|after-gap|always`). At session start `bucketRules()` splits rulebook vs TTSR. During `message_update`, the coordinator monitors `text_delta`/`thinking_delta`/`toolcall_delta`, reconstructs a source snapshot for edit/write tools (per-file `matcherEntries` for multi-file hashline/apply-patch), runs incremental regex with a 4 KB / +25% full-rescan throttle plus a final whole-buffer check, and **calls `agent.abort()` mid-token** (tool-scoped abort reason so sibling calls stay neutral). 50 ms later a token/generation-checked retry drops the partial output (`contextMode: discard`) or keeps it (`keep`), injects a hidden `<system-interrupt reason="rule_violation" …>` custom message, and continues. **Injections survive compaction.** AST conditions are checked in the awaited `beforeToolCall` hook, blocking side effects. Non-interrupting matches ride `afterToolCall` as passive `additionalContext`.

**Novelty:** **genuinely novel and the single best idea in this codebase** — a rule engine that acts on the *stream* before the tool call, with time-travel retry and cache-friendly persistence. This is the strongest realization of the "hooks-as-runtime-policy" pattern AI-Harness identified in PLAN §27.2.

**Adopt →** `S1.2` (full rule engine + canonicalization: OMP's incremental matcher and per-file scoping are worth porting), `S1.5` (narration-blind finish gate + ≤3 repair rounds; TTSR's "abort, inject, retry from same point" is the mirror image), `S0.4` (policy spine), and PLAN §8 §27.2. **Port the design, not the 25 KB coordinator.**

### 3.3 First-class subagents: structured yields, isolated worktrees, threads, lifecycle

**Where:** `docs/tools/task.md` (54 KB — one of the most detailed tool specs in the field), `docs/task-agent-discovery.md`, `docs/agent-hub.md`; code in `packages/coding-agent/src/task/` (`structured-subagent.ts`, `executor.ts`, `parallel.ts` session-scoped `Semaphore`, `worktree.ts`, `isolation-runner.ts`, `output-manager.ts`, `speculative-launch.ts`, `prompt-policy.ts`, `read-only-policy.ts`, `workpool.ts`), `thread/` (`subagent-worker.ts`, `rpc.ts`, `services/`), `registry/` (`agent-registry.ts`, `agent-lifecycle.ts`), `async/job-manager.ts`; TUI `overlays/agent-hub.ts`.

Mechanics worth stealing:
- **Batch shape** `{context, tasks[]}` — one subagent per item, required shared `context` rendered into every child's system prompt; per-item `agent`, `model` (operator-allowlisted only), `outputSchema`, `effort`, `isolated`, `tools`, `steps`.
- **Schema-validated yields** via hidden `yield` tool (up to 3 reminders, last forcing `toolChoice = yield`); parent reads `agent://<id>/reports/0/data` by **JSON path**, not prose. Outputs are artifacts: `<id>.md`, `<id>.json`, `<id>.jsonl`, `<id>.patch`.
- **Isolation PAL**: `apfs|btrfs|zfs|reflink|overlayfs|projfs|block-clone|rcopy` with fallback; patch mode or branch mode (`omp/task/<id>` + cherry-pick); ovnfs copy-on-write checkouts when the binary is present.
- **Concurrency/threads**: session semaphore; worker threads only beyond a threshold — `W(n)=0` for n<9, then `min(4, floor(log2(n/9))+1)`; shared state stays single-owned on the main thread and reaches workers by RPC (registry, IRC bus, MCP, auth, telemetry spans).
- **Idle/park/revive lifecycle**: `running|idle|parked|aborted`, idle TTL park, kill tombstones, revival reopens session JSONL; **Agent Hub** (`Alt+A`) shows live roster, cost/tokens, transcript, steering, revive, kill — without aborting the parent.
- **IRC peer messaging** (`write agent://<id>`, `wait` tool) — constrained channels, not free-form chatter.
- **Pre-walk** (start on normal model, hand off to cheap model at first edit) and **auto-prewalk** judged per assignment.
- **Speculative launch**: while the `task` JSON streams, each item's `SpawnRun` starts as soon as its object closes; dispatch adopts matching runs.
- **Advisor role**: a second model reads every turn on its own context and injects inline notes (concern/blocker) the doer must address or explain.

**Novelty:** orchestrator/worker, worktree-per-task and reviewer agents are cross-harness patterns already in PLAN §27.2 — **derivative in kind**. The well-executed specifics are schema-yield + JSON-path reads, the logarithmic thread placement, park/revive lifecycle, and speculative launch. **Beware:** headless subagents are **forced to `tools.approvalMode: yolo`** (no UI to confirm), and `task` recursion is limited by depth/spawn policy + `PI_BLOCKED_AGENT`.

**Adopt →** `S2.3` (direct match: isolated context, narrowed allowlist, attributable prompts, async mailbox), `S9` (concurrency cap/per-scope locks), `S2.6` M2 (unreceipted claims may not be built upon — OMP's schema yields are a lightweight precursor), `S1.5` (advisor ≈ finish gate), `S3.4` (IRC as permission-gated channels per PLAN §9).

### 3.4 Approval model — three inputs, four modes, persistent grants

**Where:** `docs/approval-mode.md`; code `packages/coding-agent/src/tools/approval.ts`, `approval-cache.ts`, `approval-scope.ts`, `approval-targets.ts`, `search-approval-targets.ts`, `resolve-tool-runtime.ts`.

Every tool declares a tier **`read` | `write` | `exec`** (unknown tools default to `exec`; MCP tools are declared `write` regardless of MCP annotations), plus optional argument-dependent `policy: allow|deny|prompt` and `override`/`reason`/`policyKey`. Modes: `always-ask`, `always-ask-cached` (file/folder grants in `approval-cache.db`, "Remember across sessions"), `write`, and **`yolo` (default)**. `tools.approval.<tool>` user overrides cannot bypass a tool's own deny/prompt. `tools.restrictedFolders` forces a prompt everywhere. `bash` has critical-pattern overrides (`rm -rf /`, fork bombs, download-and-execute, `/etc/passwd`) and opt-in `bash.allowCompoundCommands` with first-match-per-tokenized-segment semantics. Every prompt is re-resolved after extension handlers revise args.

**Novelty:** the tier taxonomy, grant keying (selectors peeled, symlinks resolved, Windows case-folded), and the deliberate anti-fatigue design (session grants, "Remember across sessions", "Deny and send message") are mature. But the **default is `yolo`** and the doc itself says *"This pattern policy controls approval for the `bash` tool; it is not process or filesystem containment"* and that an equally-capable `eval` path escapes `bash.patterns` deny rules. **Adopt the taxonomy and UX; reject the default and the containment claim.**

**Adopt →** `S0.4` (`readonly|auto|confirm|never` tiers — OMP's `read|write|exec` + policy/decision split is a useful field taxonomy to merge), `S1.3` (tiered permission UX: diff/URL/recipient + matched rule; anti-fatigue batching), `S1.7` (consent ledger with scope/TTL/revoke — OMP's grant DB is the primitive, but lacks TTL/attestation), `S1.8` (capability isolation as the real boundary).

### 3.5 Tool registry & unified resource addressing

**Where:** `packages/coding-agent/src/tools/index.ts` (47 KB; `BUILTIN_TOOLS`/`HIDDEN_TOOLS`), `docs/custom-tools.md`, `docs/tools/*`, `docs/blob-artifact-architecture.md`, `docs/tools/edit.md`, `crates/pi-edit`.

31 built-ins across Files & search (`read`, `write`, `edit`, `ast_edit`, `ast_grep`, `grep`, `glob`), Runtime (`bash`, `eval` with persistent Python + JS/Bun cells that can **call back into the agent's own tools** over a loopback bridge), Code intelligence (`lsp`, `debug` 28 DAP ops, `security_scan`), Coordination (`task`, `wait`, `todo`, `ask`), Desktop/web (`browser`, `computer`, `web_search`, `github`, `generate_image`, `tts`), Memory/skills (`checkpoint`, `rewind`, `retain`, `recall`, `reflect`, `memory_edit`, `learn`, `manage_skill`). **`hashline` edits**: the model cites content-hash anchors instead of retyping lines; stale anchors cause rejection before corruption (vendor claims −61% output tokens). `ast_edit` stages a preview, writes a reason to `xd://resolve`, and applies atomically on Accept. Conflict resolution is `conflict://N` ← `@theirs|@ours|@base`. `read` handles SQLite, PDFs, archives, notebooks, URLs, `ssh://` and all internal schemes through one path.

**Novelty:** hashline anchored edits and "everything is a path" are genuinely elegant; the loopback tool-calling from eval cells is powerful. The 16-scheme surface is also a large bespoke maintenance burden.

**Adopt →** `S0.2` (uniform tool envelope — compare OMP's `ToolResultMessage`/`details` shape to AI-Harness's `{status,stdout,stderr,exit,artifacts[],durationMs,correlationId}`; OMP lacks an explicit `correlationId` in the documented contract → **do not copy verbatim**), `S1.1`/`S2.5` (atomic apply + artifact paths), PLAN §5.

### 3.6 Memory — five pluggable backends, two-phase extraction, durable outbox

**Where:** `docs/memory.md`, `docs/mnemosyne-memory-backend.md`; `packages/coding-agent/src/memories/`, `memory-backend/`, `mnemopi/`, `hindsight/`, `advisor/`, `autolearn/`; tools `retain`/`recall`/`reflect`/`memory_edit`/`learn`.

Backends: `off` (**default**), `local` (project-scoped two-phase pipeline), `hindsight` (remote bank; per-project tags or global scope), `mnemopi` (local SQLite), `sharpshooter` (three friction-gated decision files). Local pipeline: Phase 1 per-session extraction (`default` role, 8-way concurrency, 120 s lease), Phase 2 cross-session consolidation (`smol` role, lease + 30 s heartbeat) → `MEMORY.md`, `memory_summary.md`, generated `skills/`. **Secret-pattern redaction before writing memory artifacts**; lessons capped at 100 entries / 2 KB, deduped, secret-redacted. Hindsight writes use a **durable outbox** (private file before ack, exponential backoff, id = `document_id` for idempotent resend, 7-day quarantine / 30-day delete). Docs explicitly say memory is *heuristic context*, not authoritative on current repo state, and must be paired with current-repo evidence.

**Novelty:** backend pluggability, outbox/quarantine discipline and the explicit staleness posture are strong engineering. **Derivative** of the general "memory tiers + retrieval" pattern in PLAN §5/S2.4.

**Adopt →** `S2.4` (memory tiers with TTL/staleness/re-validation — OMP's "cite the artifact and pair with repo evidence" is a good rule), `S2.5` (T3-style durable state / one-use refs — the outbox is directly transferable), `S1.6` (secret redaction before persistence — OMP does it for memory but **not by default for conversations**, see §4).

### 3.7 Compaction with multiple strategies

**Where:** `docs/compaction.md`; `packages/agent/src/compaction/{compaction,branch-summarization,pruning,compaction-v2-streaming,shake}.ts`, `packages/snapcompact/src/snapcompact.ts` (archives history as **dense bitmap images**, with a SQuAD eval suite). Compaction entries are first-class session entries (`firstKeptEntryId`, `tokensBefore`, `method`, `providerReplayThroughEntryId`); branch summaries capture abandoned `/tree` branches.

**Adopt →** `S2.4` (working-context compaction at 75% with artifact flush + checkpoint), `S1.6` (session resume), PLAN §27 `idle compaction` / `/rewind`. **Novelty:** snapcompact is distinctive; the rest is standard.

### 3.8 Model routing & provider breadth

**Where:** `docs/models.md`, `docs/providers.md`, `docs/adding-a-provider.md`, `docs/toolconv/*` (per-family tool-call conversion for anthropic, deepseek, gemini, gemma, glm-4.5, harmony, hermes, kimi-k2, minimax, pi-native, qwen3, xml); `config/model-registry.ts`, `model-resolver.ts`, `model-settings.ts`, `session/auth-storage.ts`, `packages/ai/src/auth/cascade.ts`.

**Nine roles** route by intent: `default`, `smol` (cheap fan-out), `slow` (deep reasoning), `plan`, `commit`, `vision`, `task`, `advisor`, `tiny`. Override at launch (`--smol`, `--slow`, `--plan`), cycle with `Ctrl+P`, swap with `/model`. Four routing knobs: custom OpenAI-compatible providers in `models.yml`; **per-role/per-model fallback chains** on 429/quota with cooldown restore; **path-scoped** `enabledModels`/`disabledProviders`; **round-robin credentials** with session affinity and per-key backoff. 60+ providers incl. coding-plan subscriptions (Cursor, Copilot, GitLab Duo, Kimi, MiniMax, GLM…) and local servers (Ollama, LM Studio, llama.cpp, vLLM).

**Novelty:** role-based routing + fallback chains + path scoping is a well-executed *engineering* answer to AI-Harness's PLAN §14 economics; no single piece is novel, but the coherence is high.

**Adopt →** there is **no dedicated `Sx.y` step for model routing** in `IMPLEMENTATION-PHASES.md`; the natural home is **PLAN §14**, with `S1.4` (budgets/rate limits) as the enforcement point and `S0.5`/`S8.1` for cost transparency/telemetry. Add an explicit `Sx.y` step if adopting the role/fallback/path-scope triad.

### 3.9 Extensibility: capabilities, extensions, hooks, skills, plugins, MCP

**Where:** `docs/extensions.md` (112 KB), `docs/extension-loading.md`, `docs/hooks.md`, `docs/skills.md`, `docs/marketplace.md`, `docs/mcp-{config,protocol-transports,runtime-lifecycle}.md`, `docs/custom-tools.md`.

- **Single "capability" abstraction** over discovery from all 8 config ecosystems; providers have numeric priority, first-wins dedupe by key (`rule.name`, skill name, tool name). Foreign *user* sources require `enabledProviders` opt-in; project sources are discoverable by default.
- **Extensions** are TS modules using the same tool API, slash-command registry, hotkey table and TUI primitives as built-ins — nothing reserved. `ExtensionToolWrapper` wraps tools; `ExtensionRunner` owns context transforms and lifecycle events. Hooks are now **legacy**: `--hook` aliases `--extension`, and discovered hook factories are loaded as extension modules (`pi.on(...)` handlers bind to the extension event bus).
- **Hook events** are strongly typed (`session_start`, `session_before_switch|branch` with cancel, `tool_call`, `before_subagent_spawn`, `afterToolCall`, …). `before_subagent_spawn` can route/block subagents — a governance surface.
- **Skills**: `SKILL.md` progressive disclosure (metadata in system prompt, content on demand via `skill://`, `/skill:<name>`), 3-pass load (capability providers → custom dirs → auto-learned "managed" skills dead-last so authored skills win), non-recursive one-level layout, Agent Plugins 1.0 strict validation vs permissive native scanning.
- **Plugins/marketplaces**: plugin manifests, install plumbing, Agent-Plugin + Claude-plugin + Gemini-manifest compatibility; `marketplace` install and npm publishing.
- **MCP**: config/runtime/transport docs, namespaced `mcp__<server>_<tool>` naming with collision hashing; MCP tools are declared `write` tier regardless of annotation.

**Novel:** the capability/provider priority system and the "inherit 8 formats natively" discovery are well-executed. **Derivative:** SKILL.md, hooks-as-policy, plugin manifests and MCP are all already in AI-Harness's §8/§27 backlog.

**Adopt →** `S2.2` (permission manifests — but OMP has no equivalent of "plugins can only claim what they declare"; add it), `S1.8` (guard evolution; OMP's `before_subagent_spawn` hook is a governance seam to mirror), `S8.3` (supply chain — OMP's trust model is the **anti**-pattern), PLAN §8.

### 3.10 UX / TUI

**Where:** `docs/tui.md`, `docs/tui-core-renderer.md`, `docs/tui-runtime-internals.md`, `packages/tui/src/*` (native/{reconcile,memo,settle,blobs}, overlays/*, chrome/*, tools/*, prompt/*, status-line/*), `docs/keybindings.md`.

Differential/native rendering; tool calls as cards; edits preview before apply; `ask` tool renders HTML forms mid-turn (radio/checkbox/select/text) with recommendation preselected — exposed as ACP **elicitations** to editors; Agent Hub (`Alt+A`); 4-state status line with cost/context/token-rate segments; magic keywords (`ultrathink`, `chillax`, `orchestrate`, `workflowz`) matched only in prose (not code spans/fences/XML/paths) via `prompt/prose-gate.ts`; `/vibe` persistent fast/good worker sessions; `/collab` live sharing via relay with **client-side sealed frames** ("the relay never sees your keys") + read-only view links + QR; shell completions generated from live command/flag metadata so they never drift.

**Adopt →** `S0.5` (CLI/TUI streaming + badges + confirm + budget meter), PLAN §10 cross-cutting items 1–8, `S2.7` (VS Code round-trip via ACP-style elicitations), `S2.6` (collab with sealed frames maps to §31 Social OS).

### 3.11 Adoption mapping summary

| OMP mechanism | AI-Harness `Sx.y` | Priority / note |
|---|---|---|
| TTSR stream-time rule injection + retry | `S1.2`, `S1.5`, `S0.4` | **High** — port design, not code |
| Live steering + pause gate + bounded provider-leak recovery | `S0.3`, `S1.5` | High |
| OTel spans + internal attr adapter + OTLP export | `S8.1` + BLIND-SPOTS-03-ENG #1 | **High** — closes a P1 blind spot |
| Structured subagent yields + worktree isolation + park/revive + Agent Hub | `S2.3`, `S9`, `S2.6` M2 | High |
| Approval tier taxonomy + grant UX (not the default) | `S0.4`, `S1.3`, `S1.7`, `S1.8` | Medium-high |
| Capability/provider discovery + 8-format inheritance | `S2.2`, PLAN §8 | Medium (trust model inverted — see §4) |
| Durable outbox / idempotent resend / quarantine | `S2.5`, `S1.6` | Medium |
| Memory staleness posture + secret-redacted memory artifacts | `S2.4` | Medium |
| Multiple compaction strategies incl. bitmap-frame | `S2.4`, `S1.6` | Low-medium |
| In-process Rust natives (no fork/exec, cross-platform) | `S0.2`, `S1.4`, `S8`, PLAN §11 | **Strategic** — different tech choice; study for latency budgets (ENG #11) |
| Hashline edits / unified path schemes / ask forms / ACP | `S0.2`, `S1.1`, `S2.7`, PLAN §5/§10 | Medium |
| Model roles + fallback chains + path-scoped models | **no Sx.y** → PLAN §14 + `S1.4` | Medium — add a step |

---

## 4. Anti-patterns — and what AI-Harness should explicitly NOT copy

1. **`yolo` is the default approval mode; approval ≠ sandbox.** `docs/approval-mode.md` L18–L27 and L119. Headless subagents are **forced** to `yolo` (`docs/tools/task.md` L130). The doc admits the bash pattern policy is "not process or filesystem containment," that `eval` can spawn a shell outside `bash.patterns` deny rules, and that "hostnames are not resolved, so a public name that points at a private address is not detected" (SSRF blind spot). **AI-Harness gate conflict:** `S0.4` default = `never` for unknown effects, `S1.4` egress DLP deny-by-default + metadata endpoint block, `S1.8` capability isolation. Verdict: adopt the *taxonomy*, invert the *default*, and never present approval as a security boundary.

2. **Secret obfuscation is OFF by default.** `docs/secrets.md` L7–L12: `secrets.enabled: false` by default; redaction applies only to provider-visible message/replay text (explicitly *not* static system prompts, static tool definitions, or binary payloads). Short (<8 char) secrets are ignored in obfuscate mode; reversible placeholders are restored into tool args. **AI-Harness P0 gate** requires "zero secrets in transcript" and redaction *before persistence* (`S0.4`, `S0.6`). Verdict: do not treat this feature as equivalent to AI-Harness's vault + pre-persistence redaction; it is a display-layer control.

3. **Repository trust loads executable project inputs by design.** README "Project inputs and trust": opening a repo auto-loads project settings, **extensions, hooks, tools, commands, skills, rules and project MCP config**; exclusion flags "narrow inputs without changing the repository-trust model." **Direct collision** with PLAN §8 ("treat skills as zero-trust — vet before install", "never raw `npx` — known RCE vector") and `S2.2` hostile-plugin suite. Verdict: this is the clearest thing to *reject*; OMP optimizes convenience over the threat model AI-Harness is built around.

4. **Massive surface area / "batteries included" maximalism.** 31 built-in tools, 16 internal URL schemes, 60+ providers, 25 search backends, ~80k LoC of Rust including an embedded bash 5.3 engine and 58–67 reimplemented coreutils, 4,883 open issues, a 252 KB `settings.md`. PLAN §27.3 explicitly lists as mistakes to avoid: telemetry without verifiable opt-out, silent model swaps, killing tools without parity, and rewriting plugin APIs without a migration window — and OMP's breadth is exactly the kind of surface where those mistakes compound. Verdict: copy *selective mechanisms*, not the product scope; a solo dev cannot reproduce this in 6 months (AI-Harness Phase 0–9).

5. **Derivative core, marketing-forward novelty claims.** The agent loop, AI client, TUI renderer, skill/plugin/MCP formats, plan mode, checkpoint/rewind, worktrees, reviewer agents and ACP are inherited from `pi-mono` or shared across the 2026 field (PLAN §27.2). The README's benchmark deltas are vendor-reported with no reproducible harness in the fetched docs. Verdict: give credit to upstream Pi in any adoption notes and re-benchmark claims internally.

6. **Supply-chain / signing posture.** Latest commit is **unsigned**; distribution spreads across curl|sh, Homebrew tap, npm, Nix, mise, PowerShell. Install does SHA-256-check the prebuilt binary (good) but there is no observed TUF/SLSA/sigstore story in the fetched docs — exactly the gap BLIND-SPOTS-03-ENG #3 flags for AI-Harness. Verdict: **do not** copy the distribution model; AI-Harness's planned TUF v1.0.37 + Sigstore keyless + SLSA L2 is stronger on paper.

7. **Honest self-limitations worth respecting.** `docs/approval-mode.md` L41–L44: approval keys are not resolved for DNS, so hostname rebinding to private IPs is invisible; `docs/memory.md` treats memory as heuristic, not authoritative; `docs/hooks.md` documents a *legacy* hook API still shipped for compatibility. Verdict: mirror this level of candor in AI-Harness docs (state what a mechanism does **not** guarantee).

---

## 5. Sources

All fetched 2026-11-17 unless noted.

**Repo identity / freshness (GitHub REST API):**
- `https://api.github.com/search/repositories?q=oh+my+pi&sort=stars&per_page=20` — repo metadata (stars/forks/license/dates/size/topics).
- `https://api.github.com/repos/can1357/oh-my-pi/commits?per_page=1` — pinned commit `2c18786`, 2026-11-17T18:04:51Z, author `H4vC`, `verified: false / unsigned`.
- `https://api.github.com/repos/can1357/oh-my-pi/git/trees/main?recursive=1` — full file map.
- Tag/release and contributor endpoints returned `403` (rate limit); upstream `badlogic/pi-mono` metadata likewise unavailable — identity of upstream taken from OMP's own README/LICENSE instead.

**Primary sources (raw `main`, pinned commit):**
- `README.md` (47 KB) — product overview, 21 numbered mechanisms, 60+ provider list, 25 search backends, Rust crate LoC table, four entry points, install, license, fork attribution, project-input trust section.
- `packages/coding-agent/DEVELOPMENT.md` — boot flow, source layout, subsystem index.
- `AGENTS.md` (28 KB), `CONTRIBUTING.md`, `.github/SECURITY.md` (listed in tree).
- `packages/agent/src/agent-loop.ts`, `agent.ts`, `live-steering.ts`, `speculative-execution.ts`, `telemetry.ts` (imported), `packages/coding-agent/src/telemetry-export-otlp.ts` (listed).
- `docs/ttsr-injection-lifecycle.md`, `docs/approval-mode.md`, `docs/tools/task.md`, `docs/memory.md`, `docs/hooks.md`, `docs/skills.md`, `docs/rulebook-matching-pipeline.md`, `docs/custom-tools.md`, `docs/compaction.md`, `docs/models.md`, `docs/agent-hub.md`, `docs/secrets.md`, `docs/extensions.md`, `docs/extension-loading.md`, `docs/session.md`, `docs/rpc.md`, `docs/sdk.md`, `docs/config-usage.md`, `docs/settings.md`, `docs/mcp-config.md`, `docs/tools/edit.md`, `docs/tui-core-renderer.md`, `docs/tui-runtime-internals.md`.
- `packages/coding-agent/src/task/structured-subagent.ts`, `parallel.ts`, `tools/index.ts`, `system-prompt.ts`.

**Project-internal cross-references:** `D:\projects\ai-harness\docs\PLAN.md` §3.4, §8, §9, §10, §11, §27; `D:\projects\ai-harness\docs\IMPLEMENTATION-PHASES.md` S0.2–S2.5, S8.1, S8.3; `BLIND-SPOTS-03-ENGINEERING.md` gaps #1, #2, #3, #11.

**Secondary (discovery only, not relied on for mechanism claims):** `Anionex/agent-vision-toolkit`, `hashgraph-online/awesome-ai-plugins`, `AVIDS2/memorix`, `FaqFirebase/pi-desktop`, `nornzach/oh-my-pi-gui`, `czottmann/pi-automode` — all surfaced via the GitHub repo search and confirm a live Pi/OMP plugin ecosystem.

---

*Bottom line for AI-Harness: OMP is not obscure and not a small config wrapper — it is a 39k-star, MIT, actively-committed Rust/TypeScript fork of Pi with a genuinely novel stream-time rule engine, an unusually complete subagent lifecycle, and a mature permission taxonomy. Adopt TTSR's design, the OTel adapter layer, schema-yield subagents and the approval taxonomy; explicitly reject the `yolo` default, project-auto-trust discovery, and the "approval is containment" framing.*
