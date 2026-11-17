# AI-Harness documentation

Research and planning corpus, assembled 2026-11-17.

## Source documents
| File | What it is |
|---|---|
| `PLAN.md` | Original architecture/strategy plan (31 sections, Oct 2026). Unmodified copy. |
| `USE-CASES-EXPANSION-V2.md` | Original 130+ venture / 22-primitives expansion. Unmodified copy. |

## Blind-spot research
Seven parallel research subagents were asked to find important things **not covered** by the two source documents (or facts in them that are now stale). Each read both source docs first and excluded anything already present. Every finding is dated and URL-cited.

| File | Area | Findings |
|---|---|---|
| `BLIND-SPOTS-01-TEMPORAL.md` | Developments after the 2026-10-26 cutoff | 15 |
| `BLIND-SPOTS-02-INTERNATIONAL.md` | EU/UK/APAC/Global South, law, i18n, payment rails | 13 |
| `BLIND-SPOTS-03-ENGINEERING.md` | Telemetry, PKI, updates, durability, testing, i18n ops | 15 |
| `BLIND-SPOTS-04-BUSINESS.md` | Pricing, license, entity, insurance, GTM, governance | 15 |
| `BLIND-SPOTS-05-HUMAN.md` | Trust calibration, HITL, consent, fatigue, a11y, coercion | 15 |
| `BLIND-SPOTS-06-VERTICALS.md` | Uncovered regulated-SMB verticals | 15 |
| `BLIND-SPOTS-07-SAFETY.md` | Modern PI defenses, shielding, containment, IAM, eval integrity | 14 |
| **`BLIND-SPOTS-INDEX.md`** | **Consolidated ranking, clusters, gap→step map, stale-fact table** | **102 total** |

## Coding-agent source research (8 subagents, 2026-11-17)
Each report audited actual source (official docs for closed harnesses), with repo paths / line refs and an `Sx.y` adoption mapping.

| File | Source audited |
|---|---|
| `AGENT-RESEARCH-01-OPENCODE.md` | `anomalyco/opencode` (227.5k★, MIT) |
| `AGENT-RESEARCH-02-OMP.md` | `can1357/oh-my-pi` (39.3k★, MIT; fork of `badlogic/pi-mono`) |
| `AGENT-RESEARCH-03-CODEX.md` | `openai/codex` (135.7k★, Apache-2.0, 139 crates) |
| `AGENT-RESEARCH-04-CLOSED-HARNESSES.md` | Claude Code, Cursor 3 / VS Code 1.139 AHP, Gemini→Antigravity, Amp, Windsurf→Devin, Copilot |
| `AGENT-RESEARCH-05-OSS-AGENTS.md` | Cline, Kilo, Goose, Aider, OpenHands, Crush, Continue, Zed, Roo |
| `AGENT-RESEARCH-06-GROK.md` | `xai-org/grok-build` + Grok Bot current-state docs |
| `AGENT-RESEARCH-07-DOTS.md` | OpenAI Dots + ChatGPT Wallet current state |
| `AGENT-RESEARCH-08-T3CODE.md` | `pingdotgg/t3code` v0.1.10 (refreshed deep source audit) |
| `AGENT-RESEARCH-09A-HERMES-IDENTITY.md` | `NousResearch/hermes-agent` — identity resolution + 18-mechanism source audit |
| `AGENT-RESEARCH-09B-HERMES-PRODUCT.md` | Hermes — 395 user stories, setup-time verification, quit/failure patterns |
| `AGENT-RESEARCH-09C-HERMES-PEERS.md` | Hermes vs OpenClaw vs MantisClaw; namesake disambiguation |
| **`AGENT-RESEARCH-09-HERMES.md`** | **Consolidated Hermes best-of, do-NOT-copy list, corrections, `Optimal` mapping** |
| **`BEST-OF-ADOPTION.md`** | **Ranked "steal this" list, "do NOT copy" list, `PLAN.md` corrections, phase mapping (incl. Hermes §7a)** |

## Implementation
| File | What it is |
|---|---|
| `IMPLEMENTATION-PHASES.md` | Phased build plan: P0–P9, discrete `Sx.y` steps, exit gates, cross-cutting workstreams, risk register, blind-spot integration (§14) and best-of coding-agent integration (§15). Includes new **S1.11** model-role routing. |

## Start here
1. `BLIND-SPOTS-INDEX.md` §3 — ranked must-fix items.
2. `BEST-OF-ADOPTION.md` §3 — ranked mechanisms to adopt, with exact source refs.
3. `BEST-OF-ADOPTION.md` §4 — anti-patterns never to copy.
4. `BLIND-SPOTS-INDEX.md` §5 / `BEST-OF-ADOPTION.md` §5 — facts in `PLAN.md` that are now wrong.
5. `IMPLEMENTATION-PHASES.md` §1 — phase map; then the phase you are building.
6. The individual report for any finding you want to act on.

**Core unchanged bet:** safety core → coding → extensibility → browser → GUI → comms → obligation custody. The research adds the missing human, jurisdictional, lifecycle, and product layers; it does not change that order.
