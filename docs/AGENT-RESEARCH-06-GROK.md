# Agent Research 06 — Grok Bot + grok-build current audit

**Research date:** 2026-11-17 · **Scope:** `xai-org/grok-build` (source mirror) + `docs.x.ai/grok-bot/*` and `docs.x.ai/build/*` (official product docs) · **Baseline:** `docs/PLAN.md` §18–§19, `docs/IMPLEMENTATION-PHASES.md` (Sx.y steps).

> **Mandate note — NEW / deeper only.** This report does not restate §18–§19. It adds verified source file paths, corrects three §18 generalisations, and maps concrete mechanisms to `Sx.y` build steps. All line/path refs are against `xai-org/grok-build@main` unless marked as docs.

---

## 0. Corrections & deeper findings vs. `PLAN.md` §18–§19 (read this first)

| # | `PLAN.md` says | Verified 2026-11-17 finding |
|---|---|---|
| C1 | §18 "**Checkpoints hidden git refs** + restore gating exist in grok-build/T3-family lineage" | Hidden git refs are a **T3** artifact (`refs/t3/checkpoints/*`, §19). **grok-build does not use hidden git refs for rewind.** Its primary checkpoint is a per-prompt JSON blob at `<cwd>/.grok/rewind-checkpoints/<session_id>/checkpoint-<prompt_index>.json` (`.gitignore: "*"`), cap 64, with an *optional* git HEAD/index domain. Source: `crates/codegen/xai-grok-workspace/src/session/checkpoint.rs`, `checkpoint_store.rs`. Keep the §19 mapping for `refs/harness/checkpoints/*`; borrow grok-build's durability mirror + cap + session-id sanitisation instead (see M4). |
| C2 | Mandate mentions "`:run`/`:swap`" | There are **no `:run` / `:swap` user commands** in the OSS tree. The real `:swap` analogue is the **toolset-swap guard** `crates/codegen/xai-grok-workspace/src/session/swap_policy.rs` (rejects tool installs/swaps while a turn is active or tool calls are in flight). The `:run` analogue is `/workflow <name>`, `/workflow pause|resume|stop`, `/deep-research` and the product's "**Run now**" routine action (changelog v1.0.1, Nov 11 2026). Do not copy the idea of a colon-prefixed ad-hoc command language; copy the *guard*. |
| C3 | §18 "**One shared cloud computer per account** — isolation is per user, not per Bot" | Exactly right, but sharpen: **conversations/memories are per-Bot; the computer is per-account**. Each Bot gets its own *screen* ("separate work surfaces, not separate security boundaries"). Docs literally say **"Do not use separate Bots as a security boundary."** Between *users*, isolation is a dedicated **Firecracker microVM per user** (kernel/memory/devices) — strong between users, zero between a user's Bots. Source: `/grok-bot/computer-and-apps`, `/grok-bot/security-faq`. |
| C4 | §18 "Auto Review is an LLM classifier" (Cursor) | In the shipped CLI it is an **independent review model** ("Auto permission mode: LLM transcript classifier with safe fast-paths") with **provenance tracking** (`llm / heuristic / timeout / transport_error / fast_path / not_wired`) and four prompt scopes. It also reviews **automation writes (routine/trigger edits) and delegation (Cloud Agent/subagent launches)** — broader than §18 implies. Source: `xai-grok-workspace/src/permission/auto_mode/mod.rs`; `/grok-bot/security`. |
| C5 | Repo framing implied a coding agent of unclear size | It is **Rust**, ~**844,530 lines** (Simon Willison SLOCCount, ~3% vendored), **5,102 files** in the mirrored tree, and a **periodic one-way mirror of an internal monorepo** (every commit is bot-authored "Synced from monorepo" with a `Source-Revision:` SHA; `SOURCE_REV` = `559751fdcec02d413e4c57c8832ab275e4f44980`). It contains **in-tree ports of `openai/codex` and `sst/opencode` tool implementations** (`THIRD-PARTY-NOTICES`, 763 KB). External contributions are **not accepted**. |

---

## 1. Repo + product freshness

### 1.1 Repository (`xai-org/grok-build`) — GitHub API, 2026-11-17

| Field | Value |
|---|---|
| Description | "SpaceXAI's coding agent harness and TUI. Fullscreen, mouse interactive, extensible." |
| Created / pushed | `2026-07-14T20:04:23Z` / last code push **`2026-09-29T16:57:13Z`** |
| API `updated_at` | `2026-11-17T17:59:53Z` (metadata only; **no code in ~7 weeks**) |
| Stars / forks / watchers / subscribers | **27,799 / 5,247 / 27,799 / 246** |
| Language / license | **Rust** / **Apache-2.0** (`LICENSE`, 11,388 bytes) |
| Default branch / tree | `main` / commit `2bdd1d6a6369de0e8c68132ea4539e9abd9e14a8` (2026-09-29, **unsigned**) |
| Contributors | **1** — `grokkybara[bot]`, 51 contributions (mirror bot) |
| Tags / releases / issues / PRs | none / none / disabled / `collaborators_only` |
| Size | 62,238 KB; 5,102 tree entries; `Cargo.lock` 364 KB |
| Security reporting | HackerOne `hackerone.com/x` only; **no public issues** (`SECURITY.md`) |
| Contribution policy | "External contributions are not accepted" (`CONTRIBUTING.md`, README) |

**Freshness verdict:** the public repo is a **trust-recovery artifact** from the July 2026 exfil scandal, not the live development stream. The real product code is in the SpaceXAI monorepo; the mirror is ~7 weeks stale as of today. Adopt from it as *architecture evidence*, not as a maintained dependency.

### 1.2 Product (Grok Bot) — official docs & changelog

- Grok Bot consumer preview **2026-08-11**, Enterprise GA **2026-09-03**; flagship **Grok 4.7** (2026-09-21); xAI folded into SpaceX Feb 2026; Cursor acquired $60B, closed 2026-08-14 and supplies the cloud computer + Auto Review. (§18 baseline, unchanged.)
- **Latest release `v1.2.0` — 2026-11-16** ("Flight redesign and hotel maps"); `v1.1.0` Nov 13 ("Gift Concierge, X drafts, meeting controls"); `v1.0.3` Nov 12 ("Image markup, profile photos, Vault cards"); `v1.0.1` Nov 11 (meetings, MS Teams, shared connectors); `v0.79.3` Nov 6; `v0.79.2` Nov 5; `v0.78.5` Nov 4; `v0.78.4` Nov 3; `v0.78.1` Oct 27; `v0.77.0` Oct 26 ("Permissions, Vault payments, and question cards"). Source: `https://x.ai/changelog/bot` (RSS at `/changelog/bot/rss.xml`).
- Docs pages carry **"Last updated: November 13, 2026"** (`/grok-bot/computer-and-apps`).
- Notable **Nov 2026** changes relevant to us (verbatim-ish):
  - "Command approval cards show the exact command **in full with hidden characters marked**, and **Allow once runs it one time only**."
  - "**Auto-review decides on large and binary file writes** on the Bot's computer, so safe ones go through without an approval card."
  - "Auto-review shows **fewer unneeded approval cards** when its check is slow or the conversation is long." (approval-fatigue response — §18/§21 "fewer approval cards" confirmed, now dated Nov 3 2026.)
  - "**Card numbers, expiry dates, and security codes are removed from memories**, including long ones, team memory, Bots shared as templates, and forget requests held for review."
  - "When a web page hides instructions for AI agents, the Bot tells you it found them and **didn't follow them**." (v0.79.2, Nov 5.)
  - Slack: "a Bot that needs your approval posts a **Review an action card only you can answer**, instead of dropping the action." (v1.2.0.)
  - "Team admins can turn off **Proactive messages** for the team's Bots." (v1.2.0.)

---

## 2. Architecture

### 2.1 Build & crate layout (from `README.md` and the tree)

```
crates/codegen/xai-grok-pager-bin   composition root → binary `xai-grok-pager`, shipped as `grok`
crates/codegen/xai-grok-pager       TUI: scrollback, prompt, modals, rendering, slash commands
crates/codegen/xai-grok-shell       agent runtime (MvpAgent), leader / stdio / headless entry points
crates/codegen/xai-grok-tools       tool implementations (terminal, file edit, search, skills, workflow…)
crates/codegen/xai-grok-workspace   host FS, VCS, execution, checkpoints, permission manager, sandbox gate
crates/codegen/xai-{acp-lib,workflow,subagent-resolution,grok-permission-rules,grok-sandbox,
                     grok-telemetry,grok-hooks,grok-mcp,grok-agent,grok-markdown,grok-subagent-resolution,…}
crates/common, crates/build, prod/mc  shared leaf crates
third_party/                        vendored Mermaid diagram stack
```
Root `Cargo.toml` is **generated / read-only**; per-crate manifests are authoritative. Build host: macOS/Linux; **Windows builds are "best-effort and not currently tested from this tree."**

### 2.2 Execution loop & control surfaces

- Core loop: `xai-grok-shell/src/session/acp_session_impl/run_loop.rs`, `turn.rs`, `tool_dispatch.rs`, `stop_gate.rs`, `wait_interrupt.rs`, `parent_interject.rs`, `interjection.rs`, `sampler_turn.rs`, `side_call.rs`, `goal.rs`.
- **Goals** (`/goal`, `--budget`): after every round the agent self-checks; when it looks done, a **panel of reviewer subagents** votes against evidence — "marked complete only when most of them agree." This is a sharper version of our finish gate (S1.5) but still LLM-judged.
- Companion state machine: plan mode (`Inactive → Pending → Active → ExitPending`), persisted to disk; transient states collapse on restart.

### 2.3 Permission pipeline (ordered — important for our S0.4)

`PreToolUse hooks → permission rules (deny > ask > allow, merged across global/project/personal scopes) → remembered grants → built-in read-only auto-approvals → prompt policy from mode`.

Modes: `default (ask)`, `acceptEdits`, `plan` (Claude-compat), `auto`, `dontAsk`, `bypassPermissions (always-approve)`. **Always-approve short-circuits after rules**; `deny` rules and hooks still apply. Read-only shell allowlist is word-boundary matched and **compound-split on `&&`/`||`/`;`/pipes** with per-segment classification. Explicitly protected: `git -c`/`--config-env`, `core.fsmonitor`, `diff.*.command`/`textconv`/`external`, shell aliases `!…`, `sort --compress-program`, `cargo check` (build.rs/proc-macro execution), `rg --pre`. Source: `user-guide/22-permissions-and-safety.md`.

### 2.4 ACP integration into editors (source + docs)

- `xai-acp-lib/src/{lib.rs,gateway.rs,channel.rs,message.rs,normalize.rs,line_reader.rs,stdin_reader.rs}` — a **transport-agnostic typed wrapper** over `agent-client-protocol`: `AcpAgentMessage` (Initialize/NewSession/Prompt/Cancel/…) vs `AcpClientMessage` (RequestPermission/ReadTextFile/SessionNotification/…); `AcpGatewaySender/Receiver`. **Session notifications are fire-and-forget** (no ACK wait) so a half-dead relay cannot freeze the streaming loop.
- **Four transports → one `MvpAgent`** (`impl acp::Agent`): `stdio` (`grok agent stdio`, primary editor path), `WS serve` (`grok agent serve --bind … --secret …`), `WS relay` (headless via grok.com), **`leader`** (one shared backend behind a `~/.grok/…sock` lock; on leader crash the client replays cached `initialize` + each `session/load`, then sends `x.ai/leader_reconnected`).
- `session/new` `_meta` supports `yoloMode`, `autoMode`, `rules`, `systemPromptOverride`, `agentProfile`. Pager side: `xai-grok-pager/src/acp/{leader_bridge.rs,model_state.rs,skills_listing.rs,spawn.rs,subagent_label_registry.rs,tracker.rs,version_mismatch.rs}`, `src/app/acp_handler/*` (~25 test modules), `src/slash/acp_command.rs`.
- Compatible clients (docs `15-agent-mode.md`): **Zed, Neovim (CodeCompanion/avante.nvim), Emacs, marimo** — supported; **JetBrains — coming soon**. Registry distribution: `npx @xai-official/grok@1.0.78 agent stdio`; installations via ACP Registry in Zed (`zed: acp registry`) and JetBrains AI Assistant. Third-party GUIs (AFK Pilot VS Code/Cursor, Grok App, T3 Code, Orca) exist because xAI ships **no first-party GUI**.

### 2.5 Sandbox, memory, workflows, telemetry

- **Sandbox** (`xai-grok-sandbox/`): OS kernel primitives — **Landlock (Linux)**, **Seatbelt/macOS SBPL** profiles (`command/backend/sbpl/{base,platform_defaults,preferences}.sbpl`). Profiles `off | workspace | devbox | read-only | strict`; **child-network blocking is Linux-only (seccomp) and a no-op on macOS**. Rich violation pipeline: `command/violation/{coarse,attribute,propose}.rs`, `deny/glob.rs`, `hook_write_deny.rs`, `network_policy.rs`, `child_net.rs`, `allow_path.rs`.
- **Memory** (`xai-grok-shell/src/session/acp_session_impl/`): `memory_capture.rs`, `memory_carryover.rs`, `memory_control.rs`, `memory_dream.rs`, `batch_memory_dream.rs`, `v2_memory_dream.rs`, `memory_forget.rs`. "Dream" = consolidation of session logs into organized topics; user commands `/memory`, `/dream`, `/flush`.
- **Workflows = Routines** (`xai-workflow/src/{engine,host,journal,run,validate,meta}.rs`; `xai-grok-shell/src/session/workflow/*`; `.grok/workflows/<name>.rhai` — **Rhai** DSL). Triggers are cron/schedule, Slack/GitHub/Linear/Sentry/PagerDuty events, bot email, webhook. Product limits: **50 routines/bot**, 20 retained run records, ≥5 min apart, **300 runs/hour/account and 200/hour/routine**.
- **Telemetry** (`xai-grok-telemetry/`): ~40 event modules (`permission`, `permission_analytics`, `plan_mode`, `skills`, `subagent`, `memory`, `git`, `consent`, `yolo`, `sandbox`, `prompt`, `turn`, …), **external OTLP** stack (`external/{emit,redact,truncate,schema,config,providers,metrics}.rs`) with an **export-time fail-closed validator**: drop the record if it has a non-empty body, a non-schema key, a closed-gate key, an unscrubbed string, or a non-scalar value; "Dropping telemetry on a schema bug is acceptable; leaking is not." There is a **remote kill switch** that can force gates off mid-run and exporters re-read on every export, plus `otel_layer/redact.rs`, `sentry.rs`, and mTLS/OTLP-gRPC tests. **This is the single best "what changed after the scandal" artifact** — pair it with §4 A1.

---

## 3. Top 10 mechanisms — file refs + adoption mapping

| # | Mechanism (source evidence) | Key file path(s) | Adopt / map to `Sx.y` |
|---|---|---|---|
| **M1** | **Plan mode as a state machine + edit gate** — only `plan.md` editable; tool-call fails with a short message naming the single editable path; approval view with `a/s/c/y/q`, inline line comments, model switch before approval; state persisted, compaction-aware. | `crates/codegen/xai-grok-pager/docs/user-guide/19-plan-mode.md`; `src/app/acp_handler/tests/plan_mode*.rs`; `src/views/plan_approval_view.rs`; `src/slash/commands/{plan,view_plan}.rs` | Extend **S0.3** (plan-as-file) with the 4-state machine + **S1.3** confirm UX (diff/comments); keep our stronger gate (see A5). |
| **M2** | **LLM permission classifier with provenance & fast paths** — `ClassifierVerdict {allow,block,unavailable}`, sources `llm/heuristic/timeout/transport_error/fast_path/not_wired`, prompt scopes `full/no_user_tool_prefix/bare_instructions/just_command`, tree-sitter shell splitting, heading neutralisation of untrusted text. | `xai-grok-workspace/src/permission/auto_mode/mod.rs` (136 KB), `auto_mode/{routine_git,security_findings}.rs`; fixtures `tests/fixtures/enterprise-managed-settings-ga.json` | **S0.4 + S1.10** (calibrated autonomy): record provenance in the receipt; classifier is an *advisory signal*, never the final gate (we keep deterministic rules + two-key for irreversible). |
| **M3** | **Toolset-swap guard (the real "`:swap`")** — one decision table `SwapPolicy::evaluate(SessionSnapshot)`; triggers `create/fork/owner_rebind/update_tool_config/mcp_snapshot/hub_tools/other`; defers on `turn_active`, `turn_active_late`, `in_flight`; Prometheus counters by trigger/guard state. | `xai-grok-workspace/src/session/swap_policy.rs` | **S2.10** (actors/locks/fencing) + **S2.1** (MCP config merge from multiple sources). Adopt the "no tool-set mutation mid-turn" invariant and metric labels. |
| **M4** | **Checkpoints: durability mirror, cap, safe naming** — `<cwd>/.grok/rewind-checkpoints/<session_id>/checkpoint-<n>.json`, `.gitignore:"*"`, cap 64 with oldest eviction, `io_lock` serialises persist vs truncate, session id hashed ("`../../etc` would escape the store root"), optional git/hunk domains, restore stays **in-process** (mirror ≠ restore), all disk I/O behind `GROK_WORKSPACE_REWIND_DURABLE` (default **off**). | `xai-grok-workspace/src/session/checkpoint.rs`, `checkpoint_store.rs`; `xai-grok-markdown/src/checkpoint.rs`; tests `rewind_cross_compaction_tests.rs` | **S1.1** (keep `refs/harness/checkpoints/*` from T3) **+ S2.9** (durability contracts). Adopt cap + sanitisation + durability-mirror split; improve by making durable mode **on** and restore verified. |
| **M5** | **Subagent / task-force mechanics** — `spawn_subagent` with `isolation: worktree`, `resume_from`, `run_in_background`; personas with declared `inputs`/`outputs` contracts chained stage-to-stage; `send_subagent_message` with `steer`/`queue`/`interject` delivery and **bounded quotas (4 in-flight per sender-target, 32 outbound per attempt)**; MCP inheritance `all/named/except/none`; `attempt_store` with accounting/codec/completion/decoder/intent/recovery/rewind. | `user-guide/16-subagents.md`; `xai-grok-shell/src/agent/subagent/{spawn.rs,child_runtime.rs,attempt_store/*,prompt_turn_receipt.rs,isolated_spawn_e2e.rs}`; `xai-grok-subagent-resolution/`; `xai-grok-tools/src/implementations/grok_build/send_subagent_message.rs` | **S2.3** (narrowed allowlists, attributable elevation) + **S2.6 M1–M3** (I/O contracts = our receipt envelope). Adopt delivery modes + quotas + worktree isolation; **keep** our async mailbox/DAG instead of peer chat. |
| **M6** | **Learn-by-demo → Skill → Routine (the product version)** — "Teach a task" records visible computer interaction **≤10 min, no mic audio**, produces a **draft skill** to review/test before scheduling; skills are `SKILL.md` with `allowed-tools`, `user-invocable`, `disable-model-invocation`, `model`/`effort`; discovery skips project skills in **untrusted folders**; trust ladder: *one-time task → save skill → automate*. | `/grok-bot/skills-routines-and-automations`; `user-guide/08-skills.md`; `xai-grok-tools/src/implementations/skills/{discovery,skill,types}.rs`; `/grok-bot/faq` ("Can I show a Bot how I do a task?") | **S3.3** (Stagehand action cache) + **S5.3** (durable scheduler) + **S9.10** (marketplace, provenance). Adopt the trust ladder + "test on safe input before scheduling" + explicit no-data/stale-data policy. |
| **M7** | **Transport-agnostic agent runtime + crash replay** — ACP protocol written once, reused over stdio/WS-serve/WS-relay/leader; fire-and-forget notifications; leader replay state machine makes self-update/leader crash near-invisible. | `xai-acp-lib/src/{gateway,channel,message,normalize}.rs`; `xai-grok-shell/src/session/acp_session_impl/{run_loop,turn,mcp,rewind,spawn}.rs`; `user-guide/15-agent-mode.md` | **S0.7** (versioned JSON-RPC contracts) + **S1.6** (session resume) + **S8.8** (offline/recovery). Adopt protocol/transport decoupling and replay-on-reconnect; our gateway stays local-only. |
| **M8** | **Egress & telemetry hardening after the scandal** — emit-time redaction + export-time fail-closed schema validator, content gates with remote kill switch, no free-text bodies, drop-not-scrub; GCS upload machinery still in tree but disabled server-side. | `xai-grok-telemetry/src/external/{redact,emit,schema,config,truncate}.rs`; `xai-grok-telemetry/src/otel_layer/redact.rs`; `xai-grok-shell/src/upload/{gcs,drain,manifest,trace,turn,memory}.rs` | **S2.8** (OTel spans, content capture OFF, pinned adapter) + **S1.4** (egress DLP, default-deny). Adopt the *validator* pattern; **never** ship a server-side-only kill switch (see A1). |
| **M9** | **Kernel sandbox as a real boundary, expressed as data** — named profiles, SBPL/Landlock, deny globs, hook-write denial, violation attribution + proposed grants, violation cards delivered through the permission hook. | `xai-grok-sandbox/src/{lib.rs,network_policy.rs,child_net.rs,allow_path.rs,deny/glob.rs,hook_write_deny.rs}`; `command/{policy,grants,observe,violation,protected}*`; `user-guide/18-sandbox.md`; `xai-grok-workspace/src/permission/{sandbox_gate,sandbox_network,sandbox_wire}.rs` | **S1.8** (capability isolation + gVisor/Firecracker tiers) + **S5.3** ("sandbox violation cards"). Adopt profile-as-data + violation→grant flow; fix the macOS no-op (A6). |
| **M10** | **Wake / Watch / Chief-of-Staff surfaces** — computer preview to *watch* clicks/typing; **Take over → "I'm done"** for SSO/passkey/CAPTCHA; group chats with one owner per stage and visible handoffs; primary Bot "notices work it can take off your plate" and offers first; Slack "Review an action card only you can answer"; redesigned per-action cards (shopping/order/flight/hotel/draft/phone) with exact-command display. | `/grok-bot/computer-and-apps` (Watch, Take over), `/grok-bot/initiative`, `/grok-bot/chat-and-collaboration` (group chat, handoffs, draft cards), `/x.ai/changelog/bot` (v1.0–v1.2) | **S5.4** (tray/notification centre, watch/take-over) + **S1.7** (graduated handover + context packet) + **S2.3** (specialist personas = "Chief of Staff"/lanes). Adopt the *ritual* (standup/handoff/watch), not the opaque routing. |

**Additional mapped items (not top-10 but in the backlog):** connectors vs computer-use → **S2.1/S3.1** (prefer structured connector/MCP; computer-use as fallback for API-less sites); bot email → **S5.1** (draft-first sending, no unattended send); 1Password/Vault/Secure Form → **S0.4/S5.6** (value-to-target, never to model — with A7 caveat); X firehose (`/grok-bot/tag-on-x`, read-only public X search without connecting an account; `@bot` handoff) → **S6.6** registry watchers / **S9.9** distribution experiments.

---

## 4. Anti-patterns — what to explicitly avoid

**A1 — Whole-workspace telemetry/exfiltration (the defining incident).** Wire-level work by `cereblab` on **v0.2.93** (published 2026-07-10, HN 2026-07-12) showed two channels: **Channel A** = file contents read into `POST /v1/responses` verbatim/unredacted (a tracked `.env` with canary `API_KEY`/`DB_PASSWORD` leaked); **Channel B** = a **whole-repo git bundle** (every tracked file + full commit history) to `POST /v1/storage` → `gs://grok-code-session-traces/repo_changes_dedup/v2/…/sha256_…`. On a **12 GB** repo of never-read files: **5.10 GiB** in **73 × ~75 MB** chunks, all HTTP 200, vs **192 KB** model traffic — a **~27,800×** ratio. A planted `src/_probe/never_read_canary.txt` the agent was told not to open was recovered verbatim by cloning the captured bundle. One user reported running it in their home directory and uploading SSH keys, password-manager DB, documents, photos. **The "Improve the model" opt-out only changed *retention*, not *transmission*** — `/v1/settings` still returned `trace_upload_enabled: true`. Uploads stopped ~July 12–13 via a **server-side switch with no client update**; 0.2.99 still contained the upload code, returning a hard-coded unavailable error. Musk: uploaded data "completely and utterly deleted"; repo open-sourced July 15. Simon Willison counted 844,530 lines of Rust; control group (Claude Code 2.1.204, Codex/GPT-5.5, Gemini 0.38.2) uploaded no repo bundle — **"Grok was the outlier."** → **Our design (S1.4, S2.8, S8.4): default-deny egress, no repo/manifest snapshots, content capture OFF, client-side kill switch, destination allowlist, and an auditable "what left the machine" receipt.** Do not rely on a remote flag or a retention toggle.

**A2 — "Approval does not reverse work already done."** Both the consumer docs and `/grok-bot/security` state: *"An approval controls the proposed action. It does not reverse work already completed."* An approval card is therefore **not** a safety boundary. → Keep **S1.1 checkpoint-before-effect + global undo ledger + hash-chained audit (S0.4)**; every confirm card must show what will be reverted and how.

**A3 — Bots as a security boundary / shared session.** One Firecracker microVM **per user**, but all Bots share files, cookies, CLI credentials and browser sessions; screens are "separate work surfaces, not separate security boundaries"; docs say **"Do not use separate Bots as a security boundary"** and **"when a workload needs its own computer and credential set, give it its own Cursor user."** → Our **S2.3 scoped subagents + S1.8 capability isolation** must be per-agent, not per-account. Never share a browser profile across agent lanes.

**A4 — LLM-as-final-authority + "fewer approval cards" as a feature.** Auto Review is documented as *"best-effort guardrails rather than a hard security boundary"*; the classifier can be `unavailable` (timeout/transport error) and then surfaces or blocks depending on mode; Nov 3/4 2026 changelog explicitly celebrates showing **"fewer unneeded approval cards"** and deciding large/binary writes automatically. → Keep deterministic `allow/ask/never` rules, **S0.9 reviewability**, **S1.7 fatigue KPIs** (we measure rubber-stamping instead of hiding cards), and **N-of-M two-key** for irreversible ops (S2.6 M4, S6.5).

**A5 — Plan-mode blind spots.** grok-build's own docs admit: *"Bash commands are not inspected for file writes — plan mode blocks the edit tools, not shell redirection"*; and *"Subagents are not covered by the parent session's plan-mode edit gate… can edit files while the parent is still in plan mode — and it inherits the parent's permission mode (including always-approve)."* → Our plan gate must live at the **tool/effect layer** (S1.2 canonicalisation, S1.3) and be enforced in **every child scope** (S2.3). Also note **always-approve can be locked by admins but a `deny` tool=bash rule cannot express "only git/gh"** (severity ordering坑) — we use `dontAsk`-style deny-by-default.

**A6 — Sandbox gaps.** Child-network blocking is **Linux-only** (seccomp), a **no-op on macOS** for `read-only`/`strict` profiles; sandbox is **off by default**; and `workspace` profile allows child network. → Enforce egress deny-by-default **inside** every sandbox tier (S1.4, S1.8), don't ship "off" as default, and test parity across OSes (S7.1/S7.2).

**A7 — Secrets/card data entering model context.** In the Grok Bot app, *"a Bot can also see a saved card's number and security code. It types them into a checkout after you tap **Pay**… and tells you a card's details when you ask."* Vault is write-only for passwords but **not** for cards in chat; a batch order card can cover **up to 500 card payments** with **one Pay tap**, and one approval covers payments for **24 hours**. Nov 2026 fixed "card numbers … removed from memories" — i.e. the leak existed. → **S0.4/S0.6/S5.6:** vault references substituted only inside the executor, one-shot merchant-scoped tokens, Windows Hello confirm, **per-item receipts**, and our **S6.5 Fiduciary Two-Key Gate** for disbursements. Never allow a single approval to cover an unbounded batch.

**A8 — Opaque model routing.** FAQ: *"Can I pick the model? **No.** Grok Bot picks the model for each task… first-party SpaceXSI **or a third-party model** from the sub-processor list, and the choice can differ from one request to the next. Users cannot be routed to or away from a particular model… does not show the underlying model… Teams' model allowlist/blocklist does not apply."* Also: API docs list **"Model Retirement (May 15, 2026)"**, and §18 notes silent deprecations. → Our **S3.7 provider-agnostic failover** must be **visible and user-controlled**: show model/effort, BYO keys, per-task cost by provider, published matrix, and honour team allowlists.

**A9 — Approval fatigue / stop doesn't undo.** Nov 2026 changelog: "Auto-review shows fewer unneeded approval cards when its check is slow or the conversation is long"; "**Stop now** does not undo actions the Bot already completed"; routines that fail on rate limit are **not retried**. → Keep undo ledger + monotonic receipts + explicit retry policy; do not "solve" fatigue by silently auto-approving.

**A10 — Trust theatre & moderation risk.** Apache-2.0 repo with **external contributions refused**, one bot contributor, no releases/issues, and a squashed mirror — openness as PR, not governance. Separately §18 records the moderation/deepfake record (1.8M deepfake images in 9 days, Jan 2026; 14-org FTC complaint; EU condemnation) and "personality-first, politically tuned" positioning. → Our **S9.5–S9.8** must be *real* OSS governance (LICENSE/entity/CONTRIBUTING/OWNERS, signed releases, external contribution pipeline) and **S9.4 trust-as-moat** with public audit-chain export.

---

## 5. Sources

**Local baseline**
- `D:\projects\ai-harness\docs\PLAN.md` §18 (line 604), §19 (line 633), §20–§21 (lines 659–700), corrections at line 1348/1405.
- `D:\projects\ai-harness\docs\IMPLEMENTATION-PHASES.md` — Sx.y definitions and gates (especially S0.3/0.4/0.7/0.9, S1.1/1.2/1.3/1.4/1.6/1.8/1.10, S2.1/2.3/2.6/2.8/2.9/2.10, S3.3/3.7, S5.1/5.3/5.4/5.6, S6.5, S8.4/8.8, S9.4/9.5/9.10).

**Repository (GitHub API + raw, fetched 2026-11-17)**
- `https://api.github.com/repos/xai-org/grok-build` (+ `/commits`, `/tags`, `/contributors`, `/git/trees/main?recursive=1`)
- `https://raw.githubusercontent.com/xai-org/grok-build/main/` — `README.md`, `SECURITY.md`, `SOURCE_REV`, `CONTRIBUTING.md`, `LICENSE`
- In-repo user guide `crates/codegen/xai-grok-pager/docs/user-guide/`: `04-slash-commands.md`, `08-skills.md`, `15-agent-mode.md`, `16-subagents.md`, `18-sandbox.md`, `19-plan-mode.md`, `22-permissions-and-safety.md`
- Key source: `xai-grok-workspace/src/session/{checkpoint.rs,checkpoint_store.rs,swap_policy.rs}`, `xai-grok-workspace/src/permission/auto_mode/mod.rs`, `xai-grok-telemetry/src/external/redact.rs`, `xai-grok-shell/src/upload/gcs.rs`, `xai-acp-lib/src/gateway.rs` (tree paths verified via API)
- `https://docs.x.ai/llms.txt` (authoritative page index; every page supports `.md` — note the harness's webfetch needed canonical URLs without the suffix)

**Official product docs (canonical URLs, fetched 2026-11-17)**
- `/grok-bot/overview`, `/grok-bot/computer-and-apps`, `/grok-bot/approvals-security-and-privacy`, `/grok-bot/security`, `/grok-bot/security-faq`, `/grok-bot/vault`, `/grok-bot/skills-routines-and-automations`, `/grok-bot/chat-and-collaboration`, `/grok-bot/initiative`, `/grok-bot/faq`, `/grok-bot/settings-and-notifications`
- `/build/overview`, `/build/modes-and-commands`, `/build/features/plan-mode`
- `https://x.ai/changelog/bot` and `/changelog/bot/rss.xml` (Nov 2026 releases)
- `http://zed.dev/acp/agent/grok-build`, `https://agentclientprotocol.com/`, `https://flaviocopes.com/ai-glossary/agent-client-protocol/` (editor/ACP corroboration)

**Incident sources (2026-07)**
- `https://gist.github.com/cereblab/dc9a40bc26120f4540e4e09b75ffb547` (wire-level analysis)
- `https://thehackernews.com/2026/07/grok-build-uploads-entire-git.html`
- `https://blog.pragmaticengineer.com/grolk-cli-uploaded-all-your-files-to-the-cloud`
- `https://betterstack.com/community/guides/ai/grok-cli-data-leak/`
- `https://simonwillison.net/2026/Jul/15/grok-build`
- `https://hyrax.dev/blog/grok-build-ssh-key-exfiltration-agent-audit`, `https://s5labs.io/resources/insights/grok-build-uploaded-git-repos-secrets/`, `https://thenextweb.com/news/grok-build-uploaded-entire-git-repositories-secrets`

**Nov 2026 freshness corroboration**
- `https://metagrok.io/how-to/grok-bot-vault-or-1password` (2026-11-17), `https://enjoy.dev/blog/grok-build-gui` (2026-11-09)
