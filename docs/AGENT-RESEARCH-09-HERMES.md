# Hermes — consolidated research & best-of (2026-11-18)
### Merges `AGENT-RESEARCH-09A-IDENTITY` (source audit) · `09B-PRODUCT` (community evidence) · `09C-PEERS` (disambiguation + comparison)

> **Identity (verified three ways).** `PLAN.md` §25.4's "Hermes" is **`NousResearch/hermes-agent`** — "The agent that grows with you", a local-first personal/desktop agent. Ruled out: the Nous Hermes **model family** (official docs say Hermes 4 is not for the agent loop), `hermes-mcp` (Elixir SDK), third-party `hermes-desktop`/`hermes-webui` wrappers, IM bots, Meta's Hermes JS engine, and the look-alike `hermes-agent.org` site. Original Reddit thread unretrievable — stated caveat.
> **Freshness.** MIT · **264,162★ / 59,589 forks / 53,475 open issues** · created 2025-07-22 · pushed 2026-11-17 · clone HEAD `37b3e405`, API HEAD `b356613` · **v0.23.2 (2026-11-16)** · ~1,989 PRs per window — one of the fastest-moving repos audited. Python core, gateway, TUI, native Desktop, optional OpenAI-compatible API on `:8642`.

---

## 1. What it actually is

A **loop-centric** agent (not gateway-centric like OpenClaw): the Python agent loop is the centre of gravity, and **where the agent talks** is separated from **where commands run** via a first-class terminal-backend abstraction: `local | Docker | SSH | Singularity | Modal | Daytona | Vercel`, with OpenShell whole-process wrapping. Profiles are isolated homes. Native Desktop ships signed MSIX (Windows x64/ARM64) and signed+notarized macOS Apple Silicon build (no Intel Mac build); it carries its own Python + Node so first launch builds nothing.

**Most-loved property in 395 curated stories plus independent Reddit/HN/X reports:** *one state substrate across Desktop / CLI / gateway*. Real use is follow-through, not code: unattended briefs, invoice drafts, reconciliation, family/SMB admin. Users route actual coding to Claude Code / Codex. **This validates our O6 follow-through track and says keep the coding wedge (O1–O4) rather than chasing generic-companion framing.**

---

## 2. The best parts — what to adopt

Each item carries the source file reference from the per-report audits and the `Optimal` step it lands on.

### Tier 1 — adopt now

| Mechanism | Source ref | What it does | Land in |
|---|---|---|---|
| **Consent allow-list with normalize-then-check ordering** | `tools/approval.py:878-908` | Historical deny-list bug let any unexpected value become "yes"; now only `once|session|always` accepted, transport aliases normalized **before** checking, every affirmative alias collapses to `once` (narrowest scope), non-strings refused | **O1.3**, plus ADR-014 |
| **Denial result carries anti-retry / anti-route-around text** | `tools/approval.py:911-927,929-978` | The *tool result the model sees* says: "Do NOT retry this command, do NOT rephrase it, do NOT attempt the same outcome via a different command. Stop and wait." | **O1.3, O1.6** — concrete gap in our current card wording |
| **Unconditional floors before every bypass; runtime self-deletion blocked** | `tools/approval.py:1299-1369` | Hardline commands, user `deny`, and deletion of the interpreter/venv the runtime boots from are refused **before** yolo/mode-off/cron-approve; user deny "never bypassable… not by an isolated container either" | **O0.4, O1.3, O1.5** — the concrete instance of "agent cannot rewrite its own tooling" |
| **Iron-proxy egress as enforced data** | `tools/environments/docker_egress.py:20-136,214-275` | Local CONNECT listener, read-only CA, per-provider proxy tokens under real env names; half-configured proxy **refuses sandbox start**; proxy-control env vars protected from user overrides; config read failures fail safe to enforcing default | **O1.4** — strongest egress implementation found; extend ours with L7 + hot-reload + inference-routing policy |
| **WS upgrade Host/Origin validation** (CVE-2026-53869 DNS-rebinding RCE, fixed 0.16.0) | community/product evidence | Loopback bind + Host/Origin validation + fail-closed auth before any PTY/event surface | **O0.6, O1.4, O7.1** — P0 |
| **Reviewable self-improvement with per-skill lock** | `tools/skills_guard.py:788-1081`, `tools/skill_provenance.py:12-43`, `agent/curator.py:158-217` | Skill writes staged to `pending/skills/`, `/skills diff` + approve; locked skills refuse all programmatic writes; hash-keyed scan cache + source-derived trust + write-origin provenance; curator **defers one interval on first observation**, pinned/cron-referenced skills never touched | **O3.2, O4.4, O2.4** — invert Hermes' `write_approval: false` default |
| **Bounded, frozen memory injection** | peer comparison + VDF | `MEMORY.md` 2,200 chars + `USER.md` 1,375, injected once at session start and **frozen**; recall via SQLite FTS5 over full history with LLM summaries | **O2.4** — adopt caps + freeze, keep our re-validation (architectural argument, **not** measured ASR reduction) |
| **Managed one-click local inference** | peer comparison + official docs | Pinned SHA-256-verified llama.cpp build matched to hardware, ≥64K context, ≥4-bit floor, 20 tok/s recommendation, least-harm spill order (never attention cache), 15-min idle unload, "no account, no API key, no network after download" | **O7.3, O7.4, O8.4** — best offline reference design; add our trace-proven zero-outbound assertion |
| **Model-scoped credential cooldowns** | `agent/credential_pool_model_cooldowns.py:20-94` | A 402 benches only the refused model; other models on the same credential stay usable; per-model 429/400 classified as model-scoped rather than credential-scoped | **S1.11** — fills the per-model-vs-per-credential detail of our new routing step |
| **`execute_code` RPC with constant-time token, refusal-free budget, protected-param stripping** | `tools/code_execution_rpc.py:85-123` | Fail-closed empty-token check, allow-listed tools, `max_tool_calls`, refusals don't consume budget, blocked params stripped before dispatch, every call logged | **O4.5** — combine with our `rhai` confinement |
| **Cron assembled-prompt injection scan + toolset denylist** | `cron/scheduler.py:167-236,407-476` | Scans the **assembled** job prompt including runtime-loaded skills; `messaging`/`clarify` always disabled, `cronjob` disabled by default; per-job lists cannot widen the denylist; lookup failures fail closed. Caveat: skew `None` fails **open** — invert | **O6.1, O6.5, O1.6** |
| **Split collection/upload telemetry consent + pinned endpoint** | `hermes_cli/config_defaults.py:2274-2287` | `enabled` and `send` independent; a packet sent only if its whole period is in a consent window; endpoint hardcoded HTTPS, non-env-overridable, non-HTTPS refused except localhost | **O8.4, O2.5** |
| **Update receipts + conservative outcome vocabulary** | `hermes_cli/update_receipt.py:144-226`, `desktop_update_verify.py:17-70` | `running|success|partial|failed`; downgrades `success`→`partial` when user action remains; distinguishes `"verified"` from `"relaunch_attempted"`; ASAR header + entry-point verification, path traversal rejected. Verify structure, **not signatures** — add our TUF/Sigstore | **O7.1** |
| **Force-redaction at the persistence boundary** | `agent/conversation_loop.py:1879-1917` | `force=True` redaction independent of the user toggle, "keeps the boundary closed even when redaction is off" | **O0.5, O2.5** |
| **Bounded fail-closed tool probes with flapping grace** | `tools/registry.py:260-289,368-445` | Hard timeout, dedupe concurrent identical probes, 30s TTL + 60s last-good grace so a flapping probe can't silently strip a toolset; plugin override of built-in needs operator opt-in | **O0.2, O4.3** |
| **Loop guardrail threshold ladder + identical-call stubbing** | `agent/tool_guardrails.py:93-195` | warn-after-2 / block-after-5 identical failures; same-tool 3/8; no-progress 2/5; hard stops **default on** for unattended; canonical sorted-JSON arg hashing, byte-identical duplicate results replaced with stub | **O1.6, O1.7** — exact numbers for our "≤3 reps" rule |
| **Windows reparse-point refusal + elevation detection** | `hermes_dir_acl.py:57-129,328-381` | Refuse reparse points in tree walks, check process elevation, verify expected owner before granting access | **O0.1, O1.5, O4.1** |
| **Sandbox identity + cross-agent handle refusal** | `tools/environments/sandbox_scope.py:33-140` | Handles stamped with tenant+agent; `CrossAgentHandleRefused`; cached environment refused if not current agent's | **O5.4** (single-desktop lock), **O1.5** — adopt handle-refusal pattern |
| **Anti-retry denial + alias normalization + floors + proxy + skill lock + memory caps + cooldowns + cron scan + telemetry consent** (summary) | — | — | see per-report 09A §4, 09C §3 |

### Tier 2 — study / adopt selectively
- **Backend taxonomy as blast-radius selection** (`local|Docker|SSH|Modal|Daytona|Vercel` + OpenShell) → **O1.5, O5.2, O6.3** taxonomy; keep MXC as the strongest rung, do not adopt remote defaults.
- **Migration importer** `hermes claw migrate --dry-run` (persona, memories, user skills, allowlist, messaging settings, selected keys — one-way) → **O8.3/O8.4** build a clean importer into our governed pipeline.
- **Honest SECURITY.md as an organizational mechanism** — names exactly one boundary (OS), declares in-process controls heuristics, "profiles are not tenants", 90-day disclosure, 13-step hardening checklist → **O0.4/O7.5/O8.4**.
- **CaMeL-trust-boundary community fork** (`nativ3ai/hermes-agent-camel`, 194★) → watch for **O4.7** TTSR/injection spine.
- A2A v1.0 plugin, HMAC-signed lifecycle webhooks, MCP client **and** server mode, `/goal` focus, cron monitor-mode skipping the LLM when nothing changed → study-grade for **O6/O4**.

---

## 3. Do NOT copy (Hermes-specific additions to `BEST-OF-ADOPTION.md §4`)

| Anti-pattern | Evidence | Our guard |
|---|---|---|
| **One-click "Approvals: off" / session `yolo` / default auto-approve when no human bridge** | `tools/approval.py:1228-1273,1368` — log literally "AUTO-APPROVED dangerous command in non-interactive non-gateway context" | **O0.4** unattended ⇒ deny; no global bypass; two-key for downgrades |
| **Auxiliary model auto-approving "low-risk" commands (`approvals.mode: smart`)** | SECURITY.md + NeuralTrust | **O6.2** guardian advisory only; deterministic policy first |
| **Approval-history mining into permanent allowlists** (`hermes approvals suggest`) | report 09B | **O2.6** never derive grants from rubber-stamped approvals; TTL + revoke |
| **Silent memory/skill writes (`write_approval: false`)** | 09B hidden-state complaints | **O2.6/O4.4** stage + sign autonomous writes by default |
| **Skills Hub importing unvetted registries (incl. ClawHub / 800+ malicious skills)** | CVE-2026-82021 (critical, MCP catalog mutable branch, fixed 0.19.0, **secondary-source only**) plus scanner-bypass CVEs | **O4.4/O4.6** vet + provenance + OSV gate on catalog sources too; `--force` cannot override a dangerous verdict |
| **Look-alike download domains / `irm … \| iex` install / AV-exclusion strategy for unsigned bundled `uv.exe`** | README `:40,50,72-103` | **O8.1/O7.1** signed `.exe/.msi`, winget/scoop, SBOM; hash changes invalidate trust, not require an AV whitelist |
| **Project-directory plugin/skill auto-loading** | `plugins/plugin_loader.py:81-125` | **O4.4** opt-in for foreign sources, signed provenance |
| **"Local-first" that is actually cloud-first** (six remote backends, "$5 VPS / GPU cluster", talk-to-it-on-Telegram-while-on-cloud-VM) | README `:19-20,29` | **O7.3/O1.4** air-gapped profile + egress receipts; no remote-execution defaults |
| **Fail-open code-skew probe** | `cron/scheduler.py:167-194` — skew `None` never yields | **O7.1** version pinning + verify-before-apply; failed skew probe stops the ticker |
| **Maintainer suppression of good-faith security/origin concerns** (issue #10232) | 09B | **O8.2/O8.4** published governance, CoC, response SLA |

---

## 4. Corrections to existing docs

- `PLAN.md` §25.4 names **MantisClaw** and Hermes as local-first desktop peers. Only Hermes is substantive: MantisClaw is a **15★, 9-fork, MPL-2.0 Go project**, last push 2026-07-03, single author — treat as an *archetype*, not a peer. Its "passes Cloudflare/bot verifications" claim is README marketing, untested.
- `BLIND-SPOTS-04`'s Hermes = MIT is **confirmed** (GitHub SPDX = MIT).
- `PLAN.md` §23.3's OpenClaw figures (42k exposed installs / 512-vuln audit / 900+ malicious skills) were **not re-derived**; independent article gives 722 advisories / 14 critical / 249 high as of 2026-09-27 — treat as secondary. Advisory counts are **not** a like-for-like security ranking (Hermes is younger, has no bug bounty, and CVE-2026-82021 appears only in secondary CVE/GitLab records).
- "Hermes requires tech investment" is **partially true but time-bounded**: one-click install/local models refute installation effort; ~50h / 2–3 weeks of tuning, memory-off-by-default, and 64k-context / 24GB-VRAM hardware needs verify time-to-value.
- No winget/scoop manifest found for Hermes (signed MSIX + Store + self-update + Nix flake only) — do not assume **O8.1** parity.

---

## 5. Net effect on our competitive story

Hermes is the strongest *local-first* peer we have studied, and it sharpens — not changes — our differentiation:

| They do well | We do better |
|---|---|
| 7 execution backends + OpenShell sandbox | **Windows MXC/restricted-token boundary with fail-closed unenforceable policy + no elevated daemon**, and OS is the only boundary in both |
| Bounded frozen memory | **Re-validation against current files** (their caps reduce surface, ours detects staleness) |
| Reviewable skill writes + locks | **Default staging + two-key activation + hash-trusted hooks + managed-only channel** (their `write_approval` defaults off) |
| Managed offline llama.cpp | **Trace-proven zero outbound calls + signed offline bundle** (they publish no zero-egress proof) |
| Honest SECURITY.md | **Receipts for every effect + hash-chained audit + "what left the machine"** (documentation vs evidence) |
| 395 stories of follow-through | **Follow-through plus a real coding harness** (they route code to Claude Code/Codex; our O1–O4 keep it) |
| Mature ecosystem | **No silent model swaps, no trajectory harvesting, no cloud-portal coupling, signed distribution** |

**One-line:** *Hermes proves the local-first follow-through market and supplies a dozen excellent mechanisms — but its defaults auto-approve, write skills silently, and couple to a vendor cloud; Optimal ports the mechanisms and inverts every one of those defaults behind a real OS boundary and receipts.*
