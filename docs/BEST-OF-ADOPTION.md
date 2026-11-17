# Best-of Adoption — what to steal from the 2026 coding agents
### 2026-11-17 · consolidates `AGENT-RESEARCH-01..08.md`

> **Method.** Eight parallel subagents audited actual source (or authoritative docs for closed harnesses) at commits dated 2026-11-17, after reading `PLAN.md` to exclude already-known material. Every mechanism below carries a repo file reference and lands on a concrete `Sx.y` step in `IMPLEMENTATION-PHASES.md`.
> **Rule:** copy the *mechanism*, never the defaults, and never the marketing number. Where a vendor's default is unsafe, our plan inverts it and the inversion is called out explicitly.

---

## 1. Executive summary

The field has converged on a handful of load-bearing mechanisms, and the AI-Harness plan already names most of them. The value of this pass is **specificity**: for each named pattern we now have a working implementation to copy, the failure mode that implementation exposes, and the exact step to put it in.

**The five highest-leverage adoptions:**

1. **A real security boundary, not a prompt/classifier** — Codex's two-axis sandbox (`PermissionProfile × AskForApproval × reviewer`) with fail-closed OS enforcement, plus Grok's kernel sandbox-as-data and T3's TOCTOU recheck under lock. → **S1.8 / S0.4**
2. **Durable state with receipts** — T3's transactional outbox + command receipts + lease worker, which is the working implementation of our event-sourcing slogan. → **S2.5 / S2.9**
3. **Code Mode with budgeted, permission-rechecked sub-calls** — OpenCode's confined JS VM and Kilo's host-owned budgets, which solve tool-catalog bloat without bigger models. → **S0.2 / S1.4 / S1.8**
4. **Stream-time policy injection** — OMP's TTSR: abort mid-token, inject a rule, retry from the same point, survive compaction. This is the strongest realization of "hooks-as-runtime-policy" in the field. → **S1.2 / S1.5 / S0.4**
5. **Compaction and rewind done properly** — OpenCode's measured-token threshold + shrink ladder + `keepID` references, and Cline/OpenCode/Claude's three-axis `code|conversation|both` rewind over shadow git. → **S2.4 / S1.1**

**The most important refusal:** every competitor's approval/auto-review/classifier is documented as *not a security boundary*. We keep them as advisory signals and put capability isolation, egress DLP, deterministic rules, and two-key gates underneath.

---

## 2. Source scoreboard (all pinned 2026-11-17)

| Source | Identity | Stars | License | Freshness / note |
|---|---|---|---|---|
| **OpenCode** | `anomalyco/opencode` (sst redirects) | 227,527 | MIT | `dev` @ `0e447fc`, pushed today; 40+ files audited |
| **Codex** | `openai/codex` | 135,700 | Apache-2.0 | Rust, 139 crates, `c08843a5`, pushed today; Guardian v3 on main |
| **OMP** | `can1357/oh-my-pi` (fork of `badlogic/pi-mono`) | 39,290 | MIT | `2c18786` today (unsigned); 17 packages + ~80k LoC Rust natives |
| **T3 Code** | `pingdotgg/t3code` | 63,874 | MIT | v0.1.10 `7d32075` today; Orchestrator.ts now 16,160 lines |
| **grok-build** | `xai-org/grok-build` | 27,799 | Apache-2.0 | Last push 2026-09-29 (7 weeks stale), **1 bot contributor**, squashed mirror |
| **OSS sweep** | Cline, Kilo, Goose, Aider, OpenHands SDK, Crush, Continue, Zed, Roo | — | mixed | Roo archived 2026-05-15; Goose → AAIF; Crush = FSL-1.1-MIT; Zed agent crates GPL |
| **Closed harnesses** | Claude Code v2.1.326, Cursor 3.25/3.26 + VS Code 1.139 AHP, Amp, Windsurf→Devin, Copilot | — | — | docs/changelogs through 2026-11-16 |
| **Dots** | OpenAI dots + ChatGPT Wallet | — | — | multi-model/region since Nov 3–13 2026 |
| **Hermes** | `NousResearch/hermes-agent` | 264,162 | MIT | v0.23.2 (2026-11-16), pushed 2026-11-17, Python + native Desktop + gateway; see `AGENT-RESEARCH-09-HERMES.md` |

---

## 3. The steal-this list (ranked by leverage)

### Tier 1 — adopt before the next capability phase

| # | Mechanism | Source ref | Why it matters | Land in | Do it better |
|---|---|---|---|---|---|
| 1 | **Two-axis sandbox**: OS boundary × approval policy × reviewer, fail-closed when unenforceable | Codex `protocol/src/config_types.rs:297-301`, `sandboxing/src/{landlock,windows,windows_mxc}.rs`, `docs/sandboxing.md` | The only mechanism in the field that treats OS enforcement and approval as independent axes | **S0.4, S1.8, S5.3** | Add our Z3 policy validation and `never` default; T3 defaults new threads to `full-access` — invert that |
| 2 | **Kernel tiers as data**: bubblewrap+seccomp (`io_uring` blocked, `PR_SET_NO_NEW_PRIVS`, managed TCP proxy), Windows `mxc → elevated → unelevated`, grok named profiles + violation→grant cards | Codex `codex-rs/linux-sandbox/README.md:10-14,52-143`, `windows-sandbox-rs/src/*`; Grok `xai-grok-sandbox/src/*` | All cited 2026 sandbox escapes were kernel/credential events; process isolation cannot answer them | **S1.8, S4.2** | Egress deny-by-default *inside* each tier; never run the daemon elevated; no macOS no-op |
| 3 | **Transactional outbox + command receipts + lease worker** | T3 `EventSink.ts:316-341,532-599`, `EffectWorker.ts:898-1092`, `CommandReceiptStore.ts:43-149` | The working implementation of our "event-sourced from day 1" claim; kills duplicate effects across surfaces | **S2.5, S2.9, S2.11** | Keep modules small (T3's Orchestrator is 16k lines); add the hash chain T3 lacks |
| 4 | **TOCTOU-safe recheck under the commit lock** | T3 `DispatchModeLimit.ts:33-94`, `McpToolAccess.ts:199-240` ("modes can rise after this check; the orchestrator rechecks under lock") | A permission change mid-call is otherwise exploitable; also explains errors to the user | **S0.4, S1.8, S2.12** | Copy verbatim, including the recorded refusal |
| 5 | **Compile-time-mandatory risk declarations** (private ctor + unique symbol, lint bans raw tool registration) | T3 `McpToolAccess.ts:39-114,144-354`, `oxlint-plugin-t3code/rules/no-raw-mcp-registration.ts` | Turns "plugins declare permissions" from documentation into a type-system guarantee | **S0.2, S2.2, S2.11** | Stronger than our current manifest plan — adopt the branded type |
| 6 | **Guardian / auto-review as a separate reviewer** with ordered decision menus, policy-revision-scoped cached scores, 3-of-50 circuit breaker | Codex `core/src/guardian/*`, `protocol/src/approvals.rs:283-412`; Dots guardian; Grok Auto Review | Scales oversight without rubber-stamping; Dots quantifies ~200× fewer stops at 99.1% approval | **S0.9, S1.7, S1.8, S2.14** | Every vendor says it is "not deterministic" — put Z3 + two-key underneath; never let the model edit the gate |
| 7 | **Code Mode with host-owned budgets + nested permission attribution** | OpenCode `packages/codemode/*` (`ExecutionLimits`, immutable bytecode, FNV hash), `instructions.ts:9-23`; Kilo `packages/codemode` | 10–100× fewer round-trips on MCP-heavy work; data stays inside the VM | **S0.2, S1.4, S1.8, S2.11** | No `eval`, cap concurrency (8), re-check permission per inner call, ban nested Code Mode |
| 8 | **TTSR — stream-time rule injection with time-travel retry** | OMP `packages/coding-agent/src/session/ttsr-coordinator.ts`, `capability/rule*.ts`, `docs/ttsr-injection-lifecycle.md` | Catches violations *before* the tool call, injects a correction, retries from the same token, survives compaction | **S1.2, S1.5, S0.4** | Port the design, not the 25 KB coordinator; pair with our canonicalizer |
| 9 | **Versioned additive protocol + generated clients + deprecation aliases** | OpenCode `packages/protocol/src/client.ts:71-124` (`deprecatedClientPaths`, experimental namespace); Codex `app-server-protocol/schema/*` + `initialize` handshake + `-32001` overload | Precisely the OpenCode-V1-break / Gemini-CLI-kill failures our plan promises not to repeat | **S0.7, S9.8** | One-minor aliases, experimental URLs, capabilities object, generated TS SDK |
| 10 | **Hash-trusted hooks, managed-only channel, fail-closed** | Codex `hooks/src/engine/*`, `config/src/requirements_layers/hooks.rs`; Claude managed recursive hooks; Copilot "any deny wins" | Neutralizes poisoned hook updates and repo-editable policy | **S2.2, S0.4, S8.6** | Hooks are advisory, not a boundary; changed hook hash → skip until re-reviewed |

### Tier 2 — high value, fold into the named step

| # | Mechanism | Source ref | Land in |
|---|---|---|---|
| 11 | **Measured-token compaction + shrink ladder + `keepID` + background precompute** | OpenCode `session/compaction.ts:41-49,104-168,348-400,430-438`; CHANGELOG `:1210-1220` | **S2.4** — best available spec for our 75% compaction; never duplicate text |
| 12 | **Three-axis rewind (`code|conversation|both`) over a separate snapshot repo** | Cline; OpenCode `snapshot.ts:89-106,200-217`; Claude 6-way rewind; T3 `CheckpointRestoreSafety.ts:13-105` | **S1.1, S1.6** — snapshots must not pollute the user's git index; "can't roll back conversation → reject before touching FS" |
| 13 | **Schema-validated subagent yields + JSON-path reads + worktree isolation + park/revive + Agent Hub** | OMP `docs/tools/task.md`, `task/{structured-subagent,worktree,isolation-runner}.rs`-equivalent TS, `thread/`; Grok `spawn.rs`, `attempt_store/*` | **S2.3, S2.6** — keep our async mailbox/DAG, not free-form chat |
| 14 | **OTel spans with an internal attribute adapter + real OTLP export** | OMP `telemetry.ts`, `telemetry-export-otlp.ts`, `OmpGenAIAttr` | **S2.8** + BLIND-SPOTS-03 #1 — the reference implementation for the pinned-schema adapter problem |
| 15 | **Bounded streams with overflow-fail; terminal retention caps** | OpenCode `event-feed.ts:8-13,48-70` (`SubscriberCapacity=4096`, drop-and-fail); T3 `LiveStreamBudget.ts:18-19`, `clientTerminals.ts:16-21` | **S1.4, S2.9** — slow subscribers fail loudly, not silently lag |
| 16 | **Blue-green service update: DB backup → trial → durable commit marker → rollback, migration preflight** | T3 `serviceLauncher.ts:1297-1416`, `cloud/servicePreflight.ts:24-78` | **S8.7** — add TUF/Sigstore; T3 only checks SHA256SUMS from the same endpoint |
| 17 | **Declarative tool-risk → confirmation annotations, fail-closed on unannotated** | Copilot MCP annotations, Amp checks, T3 risk declarations | **S0.2, S0.4, S2.2** — unannotated = destructive |
| 18 | **OSV MAL-* gate on package installs, inverted to fail-closed** | Goose OSV checker | **S2.2, S8.7** — unknown ecosystem/token → confirm/deny, never allow |
| 19 | **Aider ranked repo map + architect/editor two-model split with explicit confirm boundary** | Aider `repo map`, `ArchitectCoder` | **S2.4, S1.5, S1.3** — token-budgeted context without a vector DB |
| 20 | **LSP semantic refactor as a rule, not a convenience** | Crush LSP tools; OpenCode v1 LSP-in-loop pattern (removed in v2) | **S2.7, S1.5** — "prefer semantic rename over multi-file text edit" in finish gate |
| 21 | **Elicitation as the human-in-the-loop primitive** | Zed ACP elicitation, OMP `ask` tool HTML forms (`docs/tui.md`, `prompt/prose-gate.ts`) | **S2.7, S0.9, S1.7** — forms mid-turn surfaced as ACP elicitations |
| 22 | **Activity View + Watch/Take Over + graduated handover with context packet** | Dots Activity View; Grok `/grok-bot/computer-and-apps`; our S1.7 | **S5.4, S1.7** — copy the ritual; add our Takeover Time Budget |
| 23 | **Read-only-by-code proactive research → private notes** | Dots background scans | **S2.4, S5.3, S5.6** — capability restriction enforced in code, notes file-first |
| 24 | **One-shot merchant-scoped payment tokens; value never enters model context** | Dots ChatGPT Wallet; our S5.6 | **S5.6, S8.6** — DPAPI mint + Windows Hello confirm; strictly more sovereign than Dots |
| 25 | **Redacted secret types + emit-time redaction + fail-closed export validator + one-use secret refs** | OpenCode `auth.ts:70-101` (`Redacted`), Grok `xai-grok-telemetry/src/external/{redact,schema}.rs`, T3 `SecretRequests.ts:224-287` | **S0.4, S2.8, S8.6** — client-side kill switch, not a server-side flag |

### Tier 3 — strategic / study

- **In-process Rust natives** embedding bash 5.3 + ripgrep/fd/glob (OMP) — a different tech choice that attacks our latency budget (ENG #11); study for S8.8 resource envelope.
- **ACP-native threads with replaceable external agents** (Zed) — treat the host as protocol-side and agents as swappable processes (**S0.7, S2.7**).
- **Event-driven subscription agents** (Cursor Projects) — watch Slack/schedule/PRs, coordinator delegates never executes (**S5.3, S2.6**).
- **Cross-vendor portable Cedar-style policy** (Endor/GitGuardian/Sondera) — one rule pack normalized across harnesses (**S2.2, S9.3**); aligns with our packaged rulepack marketplace.
- **Thread-ID in commit trailer + context-minimal handoff** (Amp) — audit linkage from git log back to conversation (**S0.4, S9.4**).
- **Bitmap-frame compaction** (OMP `snapcompact`) — distinctive but low priority (**S2.4**).

---

## 4. The "do NOT copy" list

| Anti-pattern | Who / evidence | Our guard |
|---|---|---|
| **`yolo` / auto-approve as the default**, headless subagents forced to it | OMP (`approval.ts` default `yolo`), Roo `dangerously-skip-permissions`, Kilo/Cline auto modes, Grok "fewer cards" | **S0.4** deny-by-default; **S0.9** reviewability; **S1.7** measure rubber-stamping |
| **Fail-open hooks / checks** | Most vendors fail open on timeout; Copilot fails open even for admin hooks; Goose OSV logs "fail open" | **S1.8, S2.2** — crashing hook must deny; unknown → confirm |
| **Hooks / classifier as the security boundary** | Endor/CIO/SGit: hooks see intent, not OS effect; every vendor's auto-review says "best-effort, not a boundary" | **S1.8** capability isolation + egress DLP; classifier is advisory |
| **New threads default to full access** | T3 `DEFAULT_RUNTIME_MODE = "full-access"` | **S0.4** invert to `never` |
| **Approval-scope overreach** | Copilot "approve for session" on `rm one-file` silently allows any `rm`; Grok one Pay tap covers up to 500 payments / 24h | **S1.3, S1.7, S6.5** — exact-scope cards, per-item receipts, two-key |
| **"Approval does not reverse work already done"** | Grok consumer docs + `/grok-bot/security` | **S1.1, S0.4** — checkpoint before effect + undo ledger; card shows what reverts |
| **Whole-repo telemetry upload** | grok-build v0.2.93: 5.10 GiB of git bundle to GCS vs 192 KB model traffic (~27,800×); canary file recovered from bundle; opt-out changed retention, not transmission; stopped by server-side switch with no client update | **S1.4, S2.8, S8.4** — default-deny egress, no snapshots, content capture OFF, client-side kill switch, "what left the machine" receipt |
| **Silent model swap / config ignored** | Codex #32692 (config model ignored), #56819 (silent mid-turn swap, billed old model); Grok FAQ "No, you cannot pick the model" | **S1.11, S3.7** — visible routing, BYOK, published matrix |
| **Secrets / card data in model context** | Grok Bot can see saved card number + CVV and recite them; 500-payment batch under one tap | **S0.4, S5.6, S6.5** — vault refs substituted in executor only |
| **Plan-mode gaps** | grok-build docs admit shell redirection and subagents bypass the parent plan-mode edit gate | **S1.2, S1.3, S2.3** — gate at the tool/effect layer, enforced in every child scope |
| **Plugin API rewrite with no migration** | OpenCode V1 break; serial SDK/UI breaks | **S0.7** one-minor deprecation + compatibility reader (Kilo keeps legacy format) |
| **Kill an open tool for a closed successor** | Gemini CLI → Antigravity (no 1:1 parity) | **S0.7, S8.7** — dual-run window, parity checklist, signed rollback |
| **Unverified savings claims** | Code Mode "37%" is a borrowed 2025 API figure; real tests 11–55% but slower | **S1.9, S8.9** — Pass^k + CIs + benchmark version disclosure |
| **Checkpoint storage blowup** | Cline warns per-tool snapshots slow large repos | **S8.8** caps on transcript/checkpoint bytes |
| **License / governance surprises** | Crush FSL-1.1-MIT not MIT; Zed crates GPL; Goose trademark held by Block until AAIF donation; Roo archived 2026-05-15 | **S9.5–S9.8** — verify SPDX, real governance, entity, trademark policy |
| **Trust-by-design auto-loading of project hooks/tools/MCP** | OMP loads executable project sources by default | **S2.2, S8.3** — vetted install pipeline, signed provenance, opt-in for foreign user sources |
| **Cloud storage of code as a privacy tradeoff** | Cursor Projects unavailable in Privacy Mode; self-hosted workers stream diffs/screenshots up | **S1.8, S7.4** — local-first, wire-verifiable "what leaves the machine" |

---

## 5. Corrections to `PLAN.md` forced by this pass

| `PLAN.md` claim | Correction (2026-11-17) | Report |
|---|---|---|
| §15 "LSP-in-the-loop diagnostics" as a live pattern to copy | OpenCode **removed** in-process LSP in v2 — only config schema + `lsp.updated` event remain. Copy the v1 *pattern*, not a v2 file; VS Code 1.139 supplies AHP diagnostics | 01, 04 |
| §18 "hidden git checkpoints" attributed to grok-build | Hidden git refs are **T3-only**; grok-build uses `<cwd>/.grok/rewind-checkpoints/<id>/checkpoint-N.json`, cap 64, durable mode off by default | 06, 08 |
| §18 `:run` / `:swap` as user commands | Not user commands; `:swap` is `swap_policy.rs` toolset-swap guard; Auto Review also gates routine writes and subagent launches | 06 |
| §27.1 Codex "integrity levels" | Actual ladder is `WindowsSandboxLevel::{Disabled, RestrictedToken, Elevated}` + native **MXC**; daemon refuses to start elevated | 03 |
| §27 "Code Mode 37% token savings" | Borrowed 2025 API figure; real Code Mode tests 11–55% with slower runs | 04 |
| §27.1 "editing an earlier message rewinds conversation + files (VS Code 1.139)" | File rewind is **Cursor 3.25/3.26**; VS Code 1.139 supplies the Agent Host Protocol | 04 |
| §23 Roo Code still active | **Archived 2026-05-15**, pivot to closed Roomote | 05 |
| §15/§26 license assumptions | Crush = **FSL-1.1-MIT**; Zed agent crates GPL; Goose moved Block → **AAIF** | 05 |
| §19 T3 snapshot (v0.1.3) | v0.1.10: 63,874★, `Orchestrator.ts` 16,160 lines, Windows service + remote update, SHA256SUMS-only integrity — re-audit done in report 08 | 08 |
| §18 grok-build as an open platform | Apache-2.0 mirror, external contributions **refused**, 1 bot contributor, last push 2026-09-29 | 06 |
| §17 Dots as single-model | Multi-model routing (Terra) since Nov 3; Enterprise/Edu/Healthcare + residency + EKM Nov 4; ChatGPT Wallet Nov 13 | 07 |
| — | OpenCode canonical repo is `anomalyco/opencode` (227.5k★); Codex 135.7k★ / 139 crates; grok-build 27,799★ | 01, 03, 06 |

---

## 6. New step required by this pass

**Gap found by OMP research:** the field's *model-role routing + fallback chains + path-scoped models* has no home in `IMPLEMENTATION-PHASES.md`. Added as **S1.11** there. It operationalizes `PLAN.md` §14 and is distinct from S3.7 (provider-supply failover mid-task) and S8.9 (benchmark CI).

- Nine roles: `default | smol | slow | plan | commit | vision | task | advisor | tiny`.
- Per-role/per-model fallback chains on 429/quota with cooldown restore; path-scoped `enabledModels`/`disabledProviders`; round-robin credentials with session affinity and per-key backoff.
- Visible model/effort to the user, published supported-model matrix, team allowlists honoured.

---

## 7. Phase integration matrix

| Step | Mechanisms adopted from this pass |
|---|---|
| **S0.2** | Code Mode VM, uniform envelope comparison (do not copy OMP's missing correlationId), branded tool declarations, OSV install gate, tool-risk annotations |
| **S0.3** | OMP live steering + pause gate + bounded provider-leak recovery; Grok 4-state plan-mode machine; Dots 4 behavior levels |
| **S0.4** | Codex two-axis model, T3 fail-closed rule algebra + TOCTOU recheck, Grok classifier provenance in receipts, redacted types |
| **S0.5** | OMP ask-forms/Agent Hub, cost/context/token-rate status line, generated completions |
| **S0.7** | OpenCode deprecated aliases + experimental namespace + generated clients; Codex handshake/capabilities/AHP; Zed ACP negotiation |
| **S0.9** | Guardian semantics, reviewability, elicitation |
| **S1.1** | Two-repo snapshots, three-axis rewind, T3 restore-safety gating, hashline-style anchored edits |
| **S1.2** | TTSR stream-time rules, Codex Starlark prefix rules + tree-sitter splitting, path canonicalization, plan gate in effect layer |
| **S1.3** | Ordered decision menus, exact-scope cards, diff previews, session/always grants |
| **S1.4** | Bounded overflow-fail streams, egress DLP inside sandboxes, tool-output caps |
| **S1.5** | Finish gate as TTSR mirror, Aider architect/editor split, 3-turn blocked heuristic, tree-sitter semantic refactor rule |
| **S1.6** | Remote compaction blob as opaque integrity-bound item, invalidate approvals on compaction |
| **S1.7** | Guardian circuit breaker, fatigue KPIs, consent ledger with TTL/revoke, graduated handover, duress mode |
| **S1.8** | Capability isolation, bubblewrap+seccomp + MXC tiers, kernel-sandbox-as-data, guard evolution propose→verify→activate |
| **S1.9** | Reject borrowed savings numbers; benchmark version disclosure |
| **S1.11** *(new)* | Model-role routing + fallback chains + path scoping |
| **S2.1** | MCP 2026-07-28 dual transport, revision negotiation, reserved metadata, fail-closed required servers |
| **S2.2** | Hash-trusted hooks, managed-only channel, branded declarations, plugin scope/revision/auto-disable, OSV gate, permission-bearing Markdown agents |
| **S2.3** | Schema yields + JSON-path reads, worktree isolation, park/revive, delivery quotas, cache-inheriting forks |
| **S2.4** | Measured-token compaction + `keepID`, Aider ranked repo map, Dots read-only private notes, cross-vendor memory staleness posture |
| **S2.5 / S2.9** | T3 outbox + receipts + lease worker, immutable `parent_id` event tree, bounded terminals |
| **S2.7** | VS Code diagnostics/getDiagnostics (pattern from v1), ACP elicitation, LSP semantic tools |
| **S2.8** | OMP OTel adapter, Grok emit-time validator, content capture OFF |
| **S2.12** | DPoP + PKCE S256 + one-use WS tickets + JWKS rotation/revocation, zero standing privileges |
| **S2.14** | Provenance binding to exact tool-call hashes, guard evolution |
| **S3.6** | Manipulation signal in reader digest (complements the Reader/Actor work) |
| **S3.7** | Provider-agnostic adapters + visible model matrix |
| **S4.2** | Never-elevated daemon, WFP/restricted-token ladder |
| **S4.6** | Latency budgets against Dots' "shocking" slowness |
| **S5.3** | Event-driven subscription agents, HMAC webhooks, scheduled-work cancellation receipts |
| **S5.4** | Activity View / Watch / Take Over / tray notification centre |
| **S5.6** | One-shot payment tokens + Windows Hello confirm (Dots pattern, our sovereign implementation) |
| **S6.5** | Two-key gate against Grok's 500-payment/24h approval |
| **S8.5** | Own-UI a11y (WCAG 2.2 AA) |
| **S8.6** | Secret rotation + one-use refs + `Redacted` types |
| **S8.7** | TUF/Sigstore + SLSA; T3 blue-green update pattern with our signature trust root |
| **S8.8** | Checkpoint/storage caps, protocol/transport decoupling + replay-on-reconnect |
| **S8.9** | Pass^k + CIs, no borrowed numbers |
| **S9.3–S9.8** | Skill provenance, rulepack marketplace, pricing/licensing, real OSS governance, deprecation policy, support SLA |

---

## 7a. Hermes addendum (2026-11-18)

Three more subagents audited **`NousResearch/hermes-agent`** (264k★, MIT, source + community + peer comparison), consolidated in `AGENT-RESEARCH-09-HERMES.md`. It is the strongest local-first peer studied and sharpens our differentiation.

**New Tier-1 mechanisms to adopt** (each with a file ref in the consolidated report):
1. **Consent allow-list with normalize-then-check ordering** — transport aliases normalized first, every affirmative alias collapses to `once` (narrowest scope) → **O1.3**.
2. **Denial result carries anti-retry / anti-route-around text in the tool result** (the model sees it) → **O1.3, O1.6**.
3. **Unconditional floors before every bypass + block runtime self-deletion** → **O0.4, O1.5** (concrete instance of "agent cannot rewrite its own tooling").
4. **Iron-proxy egress as enforced data** — half-config sandbox refuses to start, proxy-control env protected; extend ours with L7 + hot-reload + **inference-routing** policy → **O1.4**.
5. **WS Host/Origin validation** (CVE-2026-53869 DNS-rebinding RCE, fixed 0.16.0) → **O0.6, O7.1** (P0).
6. **Reviewable self-improvement** — staged skill writes, `/skills diff`, per-skill lock, hash-keyed scan cache, first-run deferral for the curator → **O3.2, O4.4, O2.4** (invert their `write_approval: false` default).
7. **Bounded frozen memory injection** (2,200/1,375 char caps, frozen at session start) → **O2.4** (architectural, not measured ASR reduction).
8. **Managed offline llama.cpp** (SHA-256 verified, ≥64K context, idle unload) → **O7.3/O7.4** (add our trace-proven zero-outbound assertion).
9. **Model-scoped credential cooldowns** → **S1.11** detail (per-model, not per-account).
10. **`execute_code` RPC** — constant-time token, refusal-free budget, protected-param stripping → **O4.5**.
11. **Cron assembled-prompt injection scan + toolset denylist** → **O6.1/O6.5** (their skew `None` fails open — invert).
12. **Split telemetry collection/upload consent + pinned HTTPS endpoint** → **O8.4/O2.5**.
13. **Update receipts + verified-vs-attempted vocabulary** → **O7.1** (add our TUF/Sigstore trust root).
14. **Force-redaction at persistence boundary** → **O0.5**.
15. **Bounded fail-closed probes with flapping grace** → **O0.2/O4.3**.
16. **Loop threshold ladder + identical-call stubbing** → **O1.6/O1.7** exact numbers.
17. **Windows reparse-point refusal + elevation detection** → **O0.1/O1.5/O4.1**.
18. **Sandbox identity + cross-agent handle refusal** → **O5.4**.
19. **Migration importer + honest SECURITY.md** (OS as only boundary, heuristics declared) → **O8.3/O8.4**.

**Six more "do NOT copy" rows** (full table in `AGENT-RESEARCH-09-HERMES.md §3`): one-click approvals-off / non-interactive auto-approve; auxiliary-model auto-approve (`smart`); approval-history mining into permanent allowlists; silent memory/skill writes; unvetted Skills Hub registries (CVE-2026-82021, secondary-source); look-alike domains / `irm…|iex` / AV-whitelist strategy; project-plugin auto-loading; "local-first" cloud defaults; fail-open skew probe; maintainer suppression (issue #10232).

**Corrections:** MantisClaw is a **15★** stale archetype, not a Hermes peer; Hermes has **no winget/scoop** manifest; "requires tech investment" is time-bounded (setup easy, time-to-value ~50h/3 weeks); Hermes is **not** a coding harness — users route code to Claude Code/Codex, which validates our decision to keep the coding wedge.

---

## 8. Sources

Full file-level references and URLs are in `AGENT-RESEARCH-01-OPENCODE.md` … `08-T3CODE.md`. Every claim above is traced there with a repo path, line range, or dated official-doc URL. Coverage caveats (shallow Grok/Timon tree access, closed-harness docs-only evidence, star counts as a 2026-11-17 snapshot) are stated in the individual reports and are not silently upgraded here.

**One-line takeaway:** *the field has already solved the mechanics; the differentiator is doing them with deny-by-default, real isolation, inspectable state, and receipts — which is exactly the plan we are building.*
