# AGENT-RESEARCH-09B — Hermes Product / Community / Real-User Evidence

**Subagent mandate:** HERMES product/community/real-user evidence for AI-Harness Optimal.
**Research date:** 2026-11-18. **Overlap check done first:** `docs/PLAN.md` §25.4, §25.5, §29 and `docs/BEST-OF-ADOPTION.md` §4; existing Hermes mentions are only PLAN.md:860 ("local-first desktop agents rising as a Reddit-documented pattern — MantisClaw, Hermes — 'requires tech investment'") and BLIND-SPOTS-04-BUSINESS.md:82 (MIT self-hosted-agent landscape). This report adds first-hand product/community evidence; it does not restate §29's human-pain atlas or §4's anti-pattern list.

---

## 1. Identity + evidence-quality legend

### 1.1 Which Hermes? (namesake resolution)

| Candidate | What it is | Verdict |
|---|---|---|
| **Hermes 3 / Hermes 4** (NousResearch open-weight LLMs; Hugging Face `NousResearch/Hermes-3-*`, `--tool-call-parser hermes`) | Instruction-tuned model family; also a vLLM tool-call format | **Ruled out as the subject.** Same org, different product. Official docs even warn the Hermes 4 models are *not* recommended inside Hermes Agent (optimised for chat, not actions) [V, preuve.ai 2026-10-14] |
| `hermes-agent.org` | Unofficial "information site", not affiliated with Nous Research; HN callers called it "suspicious as hell" / "let me steal your secret keys" | **Ruled out.** Look-alike-domain risk, see HN item 48419000 (~Jun 2026) and Reddit "Fake Hermes website" (r/hermesagent, 2026) |
| **Hermes Agent / Hermes Desktop** — `NousResearch/hermes-agent`, site `hermes-agent.nousresearch.com` | MIT-licensed, Python, self-improving *personal agent harness*; repo created 2025-07-22; ~264k stars by Nov 2026; native Desktop shipped public preview Jun 2026 | **THE SUBJECT.** This is the local-first desktop/personal agent grouped with OpenClaw in PLAN §25.4 |

**Identity note [V, official site + README, checked 2026-11-18]:** Hermes Agent is built by Nous Research — the same lab that makes the Hermes models — so "rule out NousResearch Hermes models" resolves to *rule out the models, not the company*. One substrate, multiple surfaces: CLI/TUI, native Desktop (macOS/Windows/Linux; "no Electron, no browser tab, no cloud dependency" per Plain AI 2026-06-03), and a messaging gateway (Telegram, Discord, Slack, WhatsApp, Signal, email, Home Assistant). Conversations, memories and skills live in `~/.hermes`. Paid peripheral services exist (Nous Portal $0/$20/$100/$200 per month, Hermes Cloud from ~$0.56/instance-day, Business/Enterprise) but the core is MIT and free.

### 1.2 Evidence-quality legend (used on every finding below)

- **[F] First-hand** — a named/anonymous person describing their own deployment with specifics (Reddit, HN, X, blog, GitHub issue). Self-reported, often unverifiable, enthusiasm/venting bias.
- **[V] Vendor/official** — Nous Research site, docs, README, compare pages, curated "User Stories". Authoritative on defaults and interfaces; *not* evidence of outcomes; curated tiles have survivorship bias.
- **[C] Credible third-party** — structured review/analysis with declared methodology (VDF AI 2026-09-27; Preuve AI 2026-10-14; AIE Network 2026-07-07; Medium S. Raju 2026-04-22). **Commercial-bias caveat:** VDF sells a governed rival, Preuve sells a validation tool.
- **[T] Forum-tier** — thread titles, snippets, aggregate comment sentiment. Directional only, never a measurement.
- **[S] Single-source** — one report or issue; treat as anecdote until corroborated.
- **[I] Inference** — this researcher's conclusion from ≥2 independently sourced items; flagged as inference.

**Corroboration rule applied:** a claim is upgraded only when [F]+[F] across different venues or [F]+[V]/[C]. Where sources conflict (memory transparency, setup difficulty, star counts), the conflict is stated explicitly rather than smoothed over.

---

## 2. Real-user use cases (what people actually do)

### 2.1 Evidence base and its bias

The official "User Stories & Use Cases" page [V] aggregates **395 stories across 15 categories from 11 sources** (X 85, Reddit 85, Discord 116, GitHub 40, Blog 30, YouTube 19, HN 9, …), each tile linking to the original post (checked 2026-11-18: `https://hermes-agent.nousresearch.com/docs/user-stories`). This is the largest single corpus of real Hermes usage, but it is **vendor-selected** — happy deployments are over-represented. Independent Reddit/HN threads below are used to counterbalance.

### 2.2 Use-case clusters (with dated source and quality)

| Cluster | Concrete real-user behaviour | Dated source | Quality |
|---|---|---|---|
| **Family / household admin** | Local home-security system for an 89-year-old grandfather: installed Home Assistant on a Mac Mini, Zigbee USB radio, tablet dashboard, voice feedback, panic buttons at stairs/bedroom | X @AlexanderMann 2026-10-05, tile on user-stories page | [F] via [V] curation |
| | Parent syncs school, tutor and home plans for a child with learning disabilities; tracks tutoring sessions, spots patterns, tweaks the plan | Reddit r/hermesagent, 2026-09-27 | [F] |
| | Recipe box cleaned up in Mealie; matched ingredients, scaled to 3 adult meals, fills the weekly Kroger pickup cart ("wife tested and approved") | Reddit, 2026-10-09 | [F] |
| | Family budget bot on WhatsApp: reads credit-card SMSs, tracks categories, answers in Hebrew/English, own gateway + number | Blog technodrone.cloud, 2026-10-05 | [F] |
| | A 12-year-old gets her own Hermes *profile* rather than the parent's general-purpose agent | X @trevin 2026-08-21 | [F] |
| **SMB back office / follow-through** | Drain-laying business: quotes (PDF), invoices, CRM, job/project tracker, equipment servicing and stock — all via Telegram from the truck | Reddit, 2026-09-24 | [F] |
| | Pool service (450 pools, 400/week): "QC Now" watches service reports live, flags bad tech reports, messages owner on Telegram and tech on Slack while the route is still live | Reddit, 2026-06-14 | [F] |
| | Bookstore: two agents (customer-facing, staff on Discord with terminal tools locked down), ~$10/mo in tokens | Reddit, 2026-09-17 | [F] |
| | Swiss web agency: automated *drafting* of invoices from Harvest/ClickUp — "the AI agent never sends invoices on its own. It's always reviewed by a human first" | X @corxntyn 2026-07-30 | [F] |
| | Solo IT dept for a 165-person travel company moved ~80% of daily work into Hermes; ~30% solved outright, ~50% needs a yes/no | Reddit, 2026-10-15 | [F] |
| | French small-business operator: ~8,000 scheduled runs (Messenger lead summaries, CRM-dictated quotes, catalogue PDFs); a peer reports €2,700/mo installing and adapting Hermes | X @julientalbot974 2026-10-05; Reddit 2026-06-13 | [F]+[F] |
| **Dev / ops** | Homelab sysadmin: read-only MCP for k8s status + own SSH, fixes after per-session approval, morning backup/pod report, GitOps makes every change reversible, voice note → screenshot proof in 30s ("It's magical") | HN 48419000 comment by apexalpha (~Jun 2026) | [F] |
| | ~50-dev team: multi-agent code review, internal tools, incident/postmortem stenography, preliminary RCA, Linear triage | Reddit, 2026-08-04 | [F] |
| | Two-person video startup: Hermes runs most of SDLC — grooming, pick up, implement, test, PR, gate, ship; PMs own Azure DevOps Boards | Reddit, 2026-09-29 | [F] |
| | Nontechnical teammates query BigQuery/Stripe/Amplitude through Hermes in Slack ("works great") | HN 48587502, 2026-06-18 | [F] |
| **Enterprise / team** | Razorpay: **220+ employees** each with an always-on agent (~84 active/day), own Kubernetes namespace, encrypted storage, cloud identity and network policy | engineering.razorpay.com, 2026-07-12 | [F]/[C] |
| | Energy advisory firm: one 24/7 gateway, strict role-based capability layer (sales scoped to own leads; no financial/commissions or infra access) | X @MichaelVersluis 2026-09-18 | [F] |
| | 17-person startup: one physical machine in Germany, per-team profiles, "dreams at night", BizOps handles reconciliation/exception-finding | X @JacquelineSYC19 2026-09-16 | [F] |
| **Research / professional** | Maternal-fetal medicine specialist runs clinical documentation through **94 skills**, replacing 30–45 min of documentation per case | doctorswhocode.blog, 2026-05-17 | [F] |
| | Consultant gets an 8-page meeting-prep brief from CRM + own strategy docs via DeepSeek V4 Pro | Reddit, 2026-06-26 | [F] |
| **Creative / physical world** | Photo + spoken dimensions → parametric FreeCAD drawing and model ("Game changer in the shop") | X @Gsabin86 2026-07-17 | [F] |
| | Mushroom tent on a Raspberry Pi: 30-minute sense/decide/relay/verify loop, two Lion's Mane harvests | GitHub setasoma/mycodo-hermes-skill, 2026-05-24 | [F] |
| **Scheduling / compounding** | 28 cron jobs, 30+ self-built skills, 3am "Dreaming" job that reads every conversation and extracts decisions, bugs and mistakes to write tomorrow's context | Reddit, 2026-06-23 | [F] |

**Pattern [I]:** the dominant real use is **unattended follow-through with a human in the loop only for the irreversible step** — briefs, reconciliations, quote/invoice drafts, deadline watches, escalation flags. This is the same "tools wait for the user — nobody follows through" spine as PLAN §29.1, and maps directly to §29.10 bets #1 (follow-through engine), #2 (morning brief), #6 (paperwork shield) and #9 (living achievement journal). Several users independently arrive at our own §29 consent tiers: invoice drafts with human send, read-only health/status MCP, per-role profiles, separate kid profile.

### 2.3 What users say they love (with quality)

- **State convergence across surfaces** — "I have started a task in the desktop app at my desk and picked it up from the CLI in a repo twenty minutes later without re-explaining anything" (Anthony Maio, 2026-06-05 [C], echoing the README [V]). Design pattern: *one agent substrate, multiple surfaces* — matches our ADR-004 (one local core behind every surface).
- **The learning loop** — after 20+ self-generated skills, ~40% task-time reduction on domain-similar tasks (S. Raju 2026-04-22 [C], vendor-origin claim repeated, **unverified benchmark**). Treat as motivation, not evidence.
- **Desktop over terminal** — "The windows desktop app, this was a game changer for me and why I stuck with Hermes. I just couldn't deal with the DOS like screen of openclaw" (Reddit 1vmfy1z [T]).
- **One-click local models** — "Reading hardware → picking a quantisation → downloading weights → wiring a runtime" collapsed into one automatic flow (Reddit, 2026-09-07, 144 points [T]; official desktop docs [V]).
- **Reversible, approved action with proof** — HN homelab sysadmin quote above: GitOps reversibility + per-session SSH approval + screenshot proof [F].
- **Cheapness at the low end** — median ~$20/month, 18/24 respondents ≤$30, 5 pay $0 via local models/free endpoints (Preuve tally of Reddit thread 1vqo5wh, read 2026-10-04 [C]+[F]).

---

## 3. Setup effort / "requires tech investment" — verification

**The claim under test (PLAN.md:860 [project internal]):** local-first desktop agents "MantisClaw, Hermes — *requires tech investment*" (Reddit-documented pattern). The actual Reddit prompt is r/hermesagent "How to use it? Docker? VPS? Locally?" (2026-10-26): *"it confuses me all the warning not to run it locally, or the need to have it running on a VPS or Docker… is it better to invest money and time on setting up Hermes or GrokBot or any of the other 15 alternatives?"*

### 3.1 Evidence supporting the claim

| Finding | Source (dated) | Quality |
|---|---|---|
| "If you are starting for the first time, please start with a good Frontier Cloud model to set up initially; you will save a lot of frustration… it's not 100% true or a full picture" about free/local | Reddit "Running Locally, Really?", 2026-04-16 | [F] |
| "Don't start off by buying a Mac mini. Get a Mini Max sub for $10/month to start and figure out your needs" | Reddit "How do people use Hermes agent?", 2026-05-09 | [F] |
| "Hermes is quieter but greener: its headline memory feature is off by default, and users regularly report it silently failing to persist until configured correctly… **Self-host for control and privacy, not because it is less work.**" | AIE Network, 2026-07-07 | [C] |
| "It's now week 3. I've spent about 50 hours altogether from my first ever contact with Hermes" | Reddit 1tbj6cx "My frustrating experience with Hermes" | [F]/[T] |
| Non-developer setting up 6 agents for an agency: "almost 2 weeks of work and they were not there yet to start producing" | Reddit 1wswbe5, 2026-09-29, cited by Preuve 2026-10-14 | [F]+[C] |
| "Don't run Hermes Agent if you're not technical. It expects you to be okay with managing a server." | Preuve AI review, 2026-10-14 | [C] |
| Local model quality tax: Qwen 3.8 slow, Qwen 3.6 "makes too many mistakes for an agent I want running tasks without me watching"; second Spark ⇒ **$16k** total | Reddit "One DGX Spark for Hermes", 2026-10-25 | [F] |

### 3.2 Evidence refuting / time-bounding the claim

| Finding | Source (dated) | Quality |
|---|---|---|
| "One command to install", no sign-up for base agent, macOS/Windows/Linux; Desktop is "the same open-source agent with a graphical interface" | Official site + README, checked 2026-11-18 | [V] |
| "Hermes Desktop now sets up local models in one click… reads your hardware, picks the best model… downloads and configures the runtime… nothing to configure by hand" | Reddit 1w9hp7f, 2026-09-07, 144 points | [T]+[V] |
| Native Desktop public preview Jun 2026; "no Electron, no browser tab, no cloud dependency"; one-click install, in-app self-update | Plain AI 2026-06-03; Ry Walker Research 2026-02-27 | [C] |
| 2014 Mac mini (i5/4GB) sufficient *if using a cloud API for inference*; "you do not need a powerhouse to run it" | Reddit 1vgjlee, 2026-08-05 | [F]/[T] |
| Pi 5 / Pi 4 deployments; one reporter: "no update broke anything in 90 days" | Reddit 2026-06-30; Preuve 2026-10-14 citing Reddit 1u8fm0t | [F]+[C] |
| "Ollama launch hermes" one-command path; `$5 VPS` explicitly the floor in the README | Official README / Ollama docs, 2026 | [V] |
| "Never used any agent system before but gave Hermes a try — and it just worked!" | Reddit 1ssphkt | [F] |

### 3.3 Verdict [I]

**Partially verified, strongly time-dependent — and the claim conflates three different costs.**

1. *Installation* was never the hard part after the one-line installer; by Sep–Nov 2026 it is close to trivial (one click for local models). **Refuted for installation effort.**
2. *Time-to-useful-value* remains real: 50 hours over 3 weeks, 2 weeks for 6 agents, "don't buy hardware before you know your use case". **Verified.**
3. *Operational cognitive investment* is the real tax: memory is off by default, requires `/new` session boundaries to work, self-improvement writes freely by default, model quality determines whether `memory` is actually called, hardware sizing for 64k-context local models, allowlist/channel config. **Verified, and heavier than the original one-line claim implies.**

**Hardware reality:** local inference needs ~24GB VRAM for a 27B 4-bit quant (~17GB download) and Hermes asks for ≥64k tokens of context for agentic work; below ~30B parameters the model frequently *claims* to have saved memory without calling the tool (official troubleshooting, see §4.1). So "fully local" is attainable but carries a capability tax that pushes many users back to cloud models — **refuting any assumption that local-first is automatically cheap or easy.**

**Design implication →** O8.1 (winget/scoop signed one-command install), O3.1/O3.3 (progress + ambient state so setup is legible), O0.1 (clean-clone build), O6.1 (scheduler must work without a VPS), O7.3 (offline profile), O8.3 (Free BYOK / Pro pricing that does not require a server purchase).

---

## 4. Failure & quit patterns

### 4.1 Memory that silently doesn't persist — the #1 observed failure

- **First-hand bug [F]:** Issue #56461 (opened 2026-07-01, still open Nov 2026; labels incl. `area/local-models`, `provider/ollama`, `type/bug`): with Ollama `qwen3.5:9b`, "agent replies that it has saved the memory but immediately forgets it"; it claimed a path `~/.herMES/profiles/default/memories/preferences.txt` that did not exist; `memory` tool listed in `/tools` but model later said none available. Triage PR #56555 (2026-07-28) only adds diagnostics for "stripped pseudo tool calls" — **explicitly does not fix persistence**.
- **Official docs [V] confirm the mechanism:** "Memory only persists when the model *calls the `memory` tool*; a sentence like 'I've added that to my memory' is just text. **Small local models (roughly under 30B) and models with weak tool-calling do this often.**" Also: memory needs session boundaries; on a messaging gateway a chat is one continuous session for weeks, so the forget→recall→search learning loop "almost never gets to fire" and compaction runs repeatedly over ever-longer history; run `/new` at boundaries. Frozen snapshot means saves are invisible to already-running sessions.
- **Default trap [V]:** `memory_enabled` / `user_profile_enabled` must be explicitly enabled; `write_approval: false` and `skills.write_approval: false` by default (writes freely). "Many reviewers who dismissed Hermes as 'nothing special' had never enabled the learning loop" (S. Raju 2026-04-22 [C]).
- **Conflicting evidence resolved [I]:** Medium (2026-04-22) called memory "opaque SQLite"; HN (airstrike, ~Jun 2026) said "markdown all the way down"; official docs show **both** — bounded `MEMORY.md` (2,200 chars) + `USER.md` (1,375 chars) as markdown snapshots, plus SQLite FTS5 `state.db` for session search. The "opaque" critique applies to the session store, not the curated memory files.

**Implication → O2.4 (file-first memory + TTL/staleness/re-validation), O2.3 (compaction), O2.6 (consent ledger with signed receipts for autonomous writes), O3.2 (card shows what the agent actually did, not what it claims), ADR-005 ("why did you do that" first-class).**

### 4.2 Silent self-improvement, hidden state and token burn

- **Measured overhead [F]+[C]:** Issue #4379 (opened 2026-04-01, closed 2026-09-12): Hermes v0.6.0 sends **~13,900 fixed tokens per request — 73% of an average call** (tool definitions + system prompt); one evening of chat ≈3.9M input tokens; Preuve's worked example: ~$12/mo input on a cheap model vs **~$234/mo on Sonnet 5.5**, before output tokens. Caveat: measured on an old version.
- **Background forks [V]:** "The review fork can burn a meaningful share of total tokens on busy hosts"; replays the whole conversation; default budget 75% of context capped at 600k tokens; now has `enabled: false`, `max_input_tokens`, cheaper-model routing and idle-GPU deferral — i.e. the cost was real enough that the project built controls.
- **Quit signal [T]:** "Unpopular opinion: Hermes Agent takes the path of Openclaw" (1tx1jt9) — discomfort with hidden self-improvement calls and hidden state/cost; "Hermes burnout" (1xj6mxz) — "context tokens and chat tokens being devoured… it just eats the 5 hours limit on Codex in 20 minutes"; HN mmusc: "They're all very good at spending all your tokens. They are useful but way too expensive to run."

**Implication → O1.7 (per-task token/$ ceiling, breach → pause), O3.1 (cost footer), O2.3 (measured-token compaction), O4.5 (Code Mode `rhai` to collapse tool schemas), O4.2 (tool search), O7.4 (resource envelope), O8.3 ("why did you do that" export), ADR-008 + ADR-016.**

### 4.3 Instability, self-modification and "ops not agents"

- "10 Days Into Hermes Agent: Workflows Keep Breaking After Reboot" (Reddit 1t0qg4b [T]).
- "Hermes is worth anything?" (1ufaut8 [T]): "90% of the last 2 weeks was Hermes fixing its shit. Sessions break and disappear, gateway stops working, cli breaks… in the end it became a completely unusable and broken thing. I reinstalled the VPS and I'm trying again, after all, I've already invested too much time."
- "Hermes is too much work" (1xjc4s5 [T]): users "still find themselves babysitting the agent"; "the most practical advice is to avoid having Hermes edit its own runtime: keep a restorable backup."
- HN jdiff (~Jun 2026) on OpenClaw: "one day it killed itself by rebuilding the container it was inside"; on Hermes: "It constantly (wants to) run from upstream master which is unsettling" [F].
- Note this is the same failure class as PLAN §25.5's "environment instability, not stupidity — predictability > autonomy."

**Implication → O1.1/O1.2 (snapshots in a separate repo + three-axis rewind), O2.1/O2.2 (event store + receipts), O2.6 (guardian/consent), O7.1 (pinned signed updates, blue-green rollback, migration preflight), ADR-001 (pinned deps), ADR-004, ADR-011, ADR-013.**

### 4.4 Sycophancy / over-compliance / trust anxiety

- "Hermes is too much a yes-sir" (1tfzki3 [T]): "tells you you're wrong for everything… ask it for relationship advice and it will gaslight you into thinking you need therapy."
- DeepSeek V4 Flash on clustered Sparks "overthinks on Hermes and sometimes **moves ahead without asking for approval**" (Reddit 2026-10-25 [F]).
- Official default `approvals.mode: smart` uses an **auxiliary LLM to auto-approve low-risk commands** [V, security docs] — efficient, but it is exactly the "who decided this was low-risk?" trust question.
- Allowlist semantics: "if you do add someone to the allowlist keep in mind your agent will treat everyone on the allowlist the same, so if you have an agent with access to your keys, don't add other people" (Preuve 2026-10-14, citing Reddit 1vva7ei [C]+[F]). WhatsApp can be set to allow *everyone*.
- Positive counterweight [F]: users deliberately combine read-only MCP + per-session approval + fully reversible GitOps (HN apexalpha), and invoice drafts that are never auto-sent (X @corxntyn). The community's own best practice converges on our ADR-014.

**Implication → O1.3 (exact-scope confirm cards, ordered decisions), O3.2 (counterfactual "why / what would change this"), O3.5 (takeover protocol), O2.6 (fatigue KPIs + rubber-stamp rate), O6.2 (guardian advisory, deterministic fallback, two-key for irreversible), ADR-006, ADR-014, ADR-017.**

### 4.5 Comparison threads — OpenClaw, MantisClaw, Claude Code

| Comparator | What real users / analysts say (dated) | Quality |
|---|---|---|
| **OpenClaw** | VDF 2026-09-27: OpenClaw ~391k★ vs Hermes ~249k★; OpenClaw gateway/control-plane with 20+ channels + phone apps; Hermes = learning loop + movable execution backend. Hermes denies every messaging user until allowlisted; OpenClaw host commands run without approval for trusted operator, sandbox off by default. | [C] |
| | Security record: OpenClaw 722 published advisories (14 critical, 249 high), CVE-2026-25253 one-click token theft; ClawHub ClawHavoc **341 malicious skills initially → >800**; 42k–135k exposed instances reported. Hermes: no repo advisories; 10 reviewed GHSA entries (1 high CVE-2026-53869, 6 medium, 3 low). VDF explicitly warns the comparison is confounded by exposure time and disclosure practice. | [C] |
| | Reddit sentiment: "users seem to like Hermes more, simply because it's more stable"; "every time they update OpenClaw, something is broken"; counter-claim that pro-Hermes comments are astroturfing (r/openclaw, read 2026-10-04) | [T] |
| | "I gave my brother an OpenClaw… his final feedback before abandoning it: **It feels like I need to know programming. I ask it for the daily weather report**" (HN sshine) | [F] |
| **MantisClaw** | Different product: `karayakar/MantisClaw`, MPL-2.0, Go, "the only true Desktop agent… No complex setup — just download and run"; local portable PostgreSQL data ownership; third-party audit listing shows **License / Security Audit: Warn**. Included in PLAN §25.4's same "local-first desktop agents" pattern; no substantive head-to-head evidence found — **single-source / thin**. | [S] |
| **Claude Code / Codex** | Consistent finding: Hermes is *not* a coding harness. "Pick Claude Code or Codex for coding, and a ready-made app like Muse if you do not want to run a server" (Preuve 2026-10-14); "For software engineering… Cursor, Windsurf, or Claude Code outperform it" (S. Raju 2026-04-22); r/hermesagent users "send website and code work to Claude Code or Codex instead". Our own docs already note Hermes-style agents delegate to Claude Code/Codex. | [C]+[T]+[F] |
| | `hermes claw migrate` imports OpenClaw persona/memories/skills/allowlist/platforms/selected keys (one direction); some users found starting fresh easier. Official compare page also lists comparisons vs dots, Muse, Instinct, Grok Bot, Claude Cowork, Manus, Claude Code, Codex, OpenCode. | [V]+[F] |

### 4.6 Security incidents, privacy posture and offline operation

- **CVE-2026-53869 / GHSA-4pqm-j46f-795x [C]:** Hermes Agent **before 0.16.0** had a DNS-rebinding flaw in WebSocket endpoints; FastAPI HTTP middleware does not run on WS upgrade for `/api/pty`, `/api/ws`, `/api/pub`, `/api/events`, letting a remote page bypass Host/Origin validation, inject commands or read terminal output. CVSS 7.5 (High, KBrain); fixed in 0.16.0. Direct lesson for our local JSON-RPC/WS gateway.
- **Supply chain / look-alikes:** skills can execute arbitrary Python on import (official SECURITY.md); ClawHub malware campaign affects Hermes too because it can install from ClawHub; a user ran an installer from a fake Hermes site (Reddit 1uh4coe); HN flagged `hermes-agent.org` as an unaffiliated look-alike.
- **Official threat model [V]:** eight layers (authorization, dangerous-command approval, write denylist, container isolation, MCP credential filtering, context-file injection scanning, cross-session isolation, input sanitisation); hardline blocklist survives `--yolo` and `mode: off`; approval timeout (300s) fails closed. **But explicitly:** "Deny rules are a shell-command policy, not a complete shell interpreter or an OS capability sandbox… For containment, use OS permissions and an isolated backend"; write guards "do not sandbox a hostile or compromised agent"; "the operating system is the only security boundary against an adversarial model"; single-tenant by design. Dangerous-command checks are **skipped** in Docker/Modal/Daytona/Vercel backends because the container is the boundary.
- **Privacy posture [I from V+C]:** data at rest is local (`~/.hermes`: skills, memories, sessions, config, vault); prompts/tool output go to the configured provider. Messaging channels are third-party by default (Telegram criticised on HN as FSB-linked; users self-host Matrix instead). External memory plugins (Honcho, Hindsight, Mem0, RetainDB, ByteRover, …) can move user modelling off-machine. Telemetry/opt-out specifics were **not found** in the official docs reviewed — an explicit evidence gap.
- **Fully offline? [I, confidence medium]:** inference *can* be fully local (managed llama.cpp runtime, Ollama, one-click local models; HN dumbmrblah: "The Matrix server is local, the local AI inference is local… everything is local"). But no official air-gapped/offline guarantee was found, and default paths (cloud model, Nous Portal, Skills Hub, remote memory providers, update checks) are online. **Verdict: capable of local-only operation, not architected as an offline-first/air-gapped product.** Do not claim parity with O7.3.

### 4.7 Release cadence and maintainer responsiveness — mixed

- **Fast cadence [V]+[C]:** repo created 2025-07-22; v0.2.0 2026-03-12; v0.17.0 2026-06-19; v0.21.5 2026-09-24 (5 releases that month); v0.23.0 "The Hearth Release" 2026-11-05. "Both ship releases every few days" (VDF 2026-09-27). GitHub UI shows **5k+ open issues and 5k+ PRs** (checked 2026-11-18) — scale, not necessarily neglect.
- **Responsive on engineering triage [F]:** issue #56461 received maintainer labels (teknium1 added `area/memory`, 2026-07-19) and automated cross-PR triage; the security docs show unusually detailed, fast-moving fixes (approval semantics, pairing lockout, NT-path NTLM leak guard, self-restart guard).
- **Trust-negative incident [F]:** Issue #10232 "Genuine Question: Did We Just Witness the World's Most Sophisticated 'Ctrl+C, Ctrl+V'?" opened 2026-04-15 (19 👍, alleging a copied feature from EvoMap/evolver). Closed "not planned" by teknium1 on 2026-04-27; on 2026-05-17 Nous Research **blocked four commenters, deleted their comments, and renamed the issue title to "."**. A remaining comment (2026-05-28): "the record of your actions stays forever… engage with the claims in good faith or… do what appears to be a very clumsy attempt to sweep things under the carpet." HN: "The handling of that issue looks incredibly shady. Not touching this project with a ten foot pole." This is a real governance signal, not just a rumour.
- **Implication [I]:** governance and conflict handling are product trust surface, not overhead — see O8.2 (CONTRIBUTING/OWNERS/DCO), O8.4 (changelog + deprecation policy), ADR-018 (docs as release gates), and BEST §4's "license/governance surprises" row.

### 4.8 Quit / burnout signals (consolidated)

| Signal | Source (dated) | Quality |
|---|---|---|
| "Thinking of stopping using Hermes" | Reddit 1wwqi51 | [T] |
| "Hermes burnout" — "I work more on the Hermes Agent, that Hermes Agent for me… probably for my user case it is not the right tool" | Reddit 1xj6mxz | [F]/[T] |
| "My frustrating experience with Hermes" — 50h in 3 weeks | Reddit 1tbj6cx | [F] |
| "Hermes is too much work" — still babysitting; don't let it edit its own runtime | Reddit 1xjc4s5 | [T] |
| "Hermes is worth anything?" — sessions break/gateway dies; "already invested too much time" (sunk-cost trap) | Reddit 1ufaut8 | [F] |
| "Am I expecting too much?" — "whenever Hermes seems to run into a limitation it recommends a third party tool" | Reddit 1uef8rv | [F] |
| "Is Hermes worth it over Codex or Claude Code?" — non-dev, 2 weeks, not producing | Reddit 1wswbe5, 2026-09-29 | [F] |
| Sluggish/over-engineered HN verdict; "I'd rather have full session logs than MEMORY.md summaries" | HN sshine (~Jun 2026) | [F] |

**Meta-pattern [I]:** users quit when the agent becomes a *second job* (tuning, memory plumbing, self-repair, token gardening) — the same "delegation fatigue" and "environment instability" anti-patterns already recorded in PLAN §25.5 and BEST §4. The differentiator is not more autonomy; it is **predictability, cheapness and reversibility**.

---

## 5. Design implications mapped to Optimal steps / ADRs

| # | Hermes finding (source anchor) | Design implication for AI-Harness | Optimal step / ADR | Priority |
|---|---|---|---|---|
| D1 | Local WS gateway DNS-rebinding RCE, CVE-2026-53869 (VDF 2026-09-27; NVD/GHSA) | Validate Host/Origin on WS upgrade, loopback bind, fail-closed auth before any PTY/event surface | **O0.6** (gateway handshake), **O1.4** (egress DLP), **O7.1** (signed updates) | P0 |
| D2 | OS is the only boundary; single tenant; container backends skip approval (official SECURITY.md) | Never-elevated daemon, MXC/restricted-token ladder, refuse unenforceable policy; 5k+ issue backlog means do not rely on upstream fixes | **O1.5**, **ADR-004**, **ADR-006**, **ADR-007** | P0 |
| D3 | Skills execute arbitrary code on import; ClawHub malware; fake installer domain | Vetted install pipeline, signed provenance, hash-trusted hooks, managed-only channel, auto-disable on revision change; OSV fail-closed | **O4.4**, **O4.6**, **O7.1**, **O8.2**, **ADR-011** | P0 |
| D4 | Memory silently not saved; weak local models fake success; frozen snapshot; needs session boundaries (issue #56461 + official memory docs) | File-first memory with re-validation; card shows *actual effect*; receipts make "I saved it" verifiable; staleness TTL | **O2.4**, **O2.1/O2.2**, **O3.2**, **ADR-005** | P1 |
| D5 | `write_approval` / `skills.write_approval` default OFF; learning loop writes freely; "hidden state" complaints | Invert default: autonomous memory/skill writes staged and signed by default; consent scope/TTL/revoke | **O2.6**, **O6.2**, **ADR-014**, **ADR-006** | P1 |
| D6 | 13.9k fixed tokens/call = 73% overhead; background review replays whole transcript | Measured-token compaction, Code Mode tool namespace, tool search, budget breach pauses, cost footer includes background work | **O1.7**, **O2.3**, **O3.1**, **O4.2**, **O4.5**, **O7.4**, **ADR-008**, **ADR-016** | P1 |
| D7 | Confirm fatigue risk: `approvals suggest` mines history into allowlist; "smart" mode auto-approves via auxiliary model; yes-sir behaviour | Fatigue KPIs + rubber-stamp rate; exact-scope cards with counterfactual; guardian advisory only; two-key irreversible | **O1.3**, **O3.2**, **O3.5**, **O2.6**, **O6.2**, **ADR-014** | P1 |
| D8 | "Workflows keep breaking after reboot"; agent edits own runtime; upstream-master churn | Separate snapshot repo, three-axis rewind, signed pinned blue-green updates with rollback + migration preflight | **O1.1**, **O1.2**, **O7.1**, **O7.5**, **ADR-001**, **ADR-011**, **ADR-013** | P1 |
| D9 | Unattended briefs, invoice drafts, reconciliation, deadline watches dominate real use (395 stories; Reddit/HN/X) | Follow-through engine + morning brief + draft-only event-driven agents + durable scheduler | **O6.1**, **O6.4**, **O6.5**, **O3.3**, **ADR-014** | P2 |
| D10 | State convergence across CLI/Desktop/gateway is the most-loved property (Maio 2026-06-05) | One local core behind every surface with capabilities handshake; sessions survive window close | **O0.6**, **O4.3**, **O6.3**, **ADR-004** | P2 |
| D11 | Role-scoped gateways and per-profile isolation in real deployments (Razorpay 220+, energy advisory, bookstore) | Per-agent/per-sender policy scoping, capability labels, consent ledger; single-desktop lock | **O0.4**, **O2.6**, **O5.4**, **ADR-006**, **ADR-014** | P2 |
| D12 | Local-only possible but not an offline product; external memory plugins move data off-machine; gateways depend on third-party chat | Offline profile with zero-outbound proof; content capture OFF; "what left the machine" receipt; OS vault refs | **O1.4**, **O0.5**, **O2.5**, **O7.3**, **O8.4**, **ADR-010**, **ADR-017** | P2 |
| D13 | Setup/time-to-value tax (50h / 2 weeks; memory off by default; 64k context, 24GB VRAM) | One-command signed install; legible progress; no mandatory VPS; Free BYOK pricing; small-local-model support | **O0.1**, **O3.1**, **O3.3**, **O6.1**, **O7.3**, **O8.1**, **O8.3** | P2 |
| D14 | Not a coding harness — users route code to Claude Code/Codex | Keep our code-agent wedge (git, rewind, diff, LSP) while adding follow-through; do not chase generic companion framing | **O1.1–O1.3**, **O4.2**, **O6.4** | P2 |
| D15 | Maintainer blocked/deleted critics in issue #10232; MIT licence but company-controlled project | Governance as trust feature: CONTRIBUTING/OWNERS/DCO, code of conduct, public changelog + deprecation policy, explainable export | **O8.2**, **O8.3**, **O8.4**, **ADR-012**, **ADR-018** | P3 |
| D16 | Rapid releases break setups; upstream-master churn | Pinned deps, verify-before-apply, DB backup + rollback, benchmark-version pin in CI | **O7.1**, **O7.5**, **ADR-001**, **ADR-011** | P2 |

### 5.1 Addendum to BEST §4 "do NOT copy" (proposed new rows)

| Anti-pattern | Evidence from this pass | Suggested guard |
|---|---|---|
| **"Learning loop" that writes memory/skills silently by default** | Hermes `write_approval: false` + background review; hidden-state complaints (Reddit 1tx1jt9) | Stage autonomous writes; signed consent grant; visible cost — O2.6, ADR-014 |
| **Auxiliary model auto-approving "low-risk" commands as the default** | `approvals.mode: smart` auto-approves via LLM | Deterministic policy first; classifier advisory + exact-scope card — O1.3/O6.2, ADR-006/007 |
| **Approval-history mining into permanent allowlists** | `hermes approvals suggest` | Never derive grants from rubber-stamped approvals; TTL + revoke — O2.6, S1.7 |
| **Skill marketplace that imports unvetted registries (incl. one with 800+ malicious skills)** | ClawHub/ClawHavoc; Hermes Skills Hub pulls from ClawHub | Vetted install + provenance + `--force` cannot override a dangerous verdict — O4.4/O4.6 |
| **Look-alike download domains / unverified install scripts** | `hermes-agent.org` HN backlash; fake-site Reddit thread | Single signed distribution channel, winget/scoop manifests, in-app verify — O8.1/O7.1 |
| **Maintainer suppression of good-faith security/origin concerns** | Issue #10232 (blocking + title erasure) | Published governance, code of conduct, response SLA — O8.2/O8.4 |

---

## 6. Sources

**Official / vendor [V]**
1. Hermes Agent site & install/pricing/FAQ — https://hermes-agent.nousresearch.com/ (checked 2026-11-18)
2. Open-source/self-host page — https://hermes-agent.nousresearch.com/open-source-ai-agent (checked 2026-11-18)
3. Repo `NousResearch/hermes-agent` (MIT; created 2025-07-22; ~264k★) — https://github.com/NousResearch/hermes-agent (checked 2026-11-18)
4. Security guide (8 layers, approval modes, hardline blocklist, container bypass) — https://hermes-agent.nousresearch.com/docs/user-guide/security (checked 2026-11-18)
5. Persistent memory docs (2,200/1,375 char caps, frozen snapshot, troubleshooting, background review cost) — https://hermes-agent.nousresearch.com/docs/user-guide/features/memory (checked 2026-11-18)
6. User Stories & Use Cases (395 tiles) — https://hermes-agent.nousresearch.com/docs/user-stories (checked 2026-11-18)
7. Official vs OpenClaw comparison (updated 2026-11-11; 1,000+ marketplace plugins) — https://hermes-agent.nousresearch.com/compare/openclaw
8. Release v0.23.0 "The Hearth Release", 2026-11-05 — https://github.com/NousResearch/hermes-agent/releases/tag/v0.23.0
9. Issue #56461 "Built-in memory never persists with Ollama", 2026-07-01 (open) — https://github.com/NousResearch/hermes-agent/issues/56461
10. Issue #4379 "73% of each API call is fixed overhead", 2026-04-01 → closed 2026-09-12 — https://github.com/NousResearch/hermes-agent/issues/4379
11. Issue #10232 origin/plagiarism dispute, 2026-04-15 → blocking/deletions 2026-05-17 — https://github.com/NousResearch/hermes-agent/issues/10232

**Third-party analysis [C]**
12. VDF AI, "Hermes Agent vs OpenClaw: A Neutral 2026 Comparison", 2026-09-27 — https://vdf.ai/blog/hermes-agent-vs-openclaw/
13. Preuve AI (V. Forat), "Hermes Agent Review (2026): $29 or $251 a Month", 2026-10-14 — https://preuve.ai/blog/hermes-agent-review
14. AIE Network (M. R. Hinkle), "Run Your Own Local AI Agent", 2026-07-07 — https://www.theaienterprise.io/p/self-host-your-own-ai-agent
15. Sathish Raju, "I Switched from OpenClaw to Hermes Agent…", Medium, 2026-04-22 — https://medium.com/@sathishkraju/i-switched-from-openclaw-to-hermes-agent-heres-what-nobody-told-me-5f33a746b6ca
16. Anthony Maio, "Hermes Agent Got Serious" (state convergence / Desktop), 2026-06-05 — https://anthonymaio.substack.com/p/hermes-agent-got-serious
17. Plain AI, "Nous Research Ships Hermes Desktop", 2026-06-03 — https://plainai.tech/articles/hermes-desktop-nous-research-ai-agent-app
18. Razorpay Engineering, "Running Hermes at Razorpay", 2026-07-12 — https://engineering.razorpay.com/running-hermes-at-razorpay-a-network-isolated-self-improving-second-brain-for-every-employee-f91d56bea3f1
19. ishchuk.eu, "What It Costs to Run an AI Agent", 2026 (first-hand Mac Mini invoices) — https://ishchuk.eu/blog/what-it-costs-to-run-an-ai-agent-monthly-numbers-small-business
20. NVD / GHSA-4pqm-j46f-795x (CVE-2026-53869, DNS rebinding, fixed 0.16.0) — https://nvd.nist.gov/vuln/detail/CVE-2026-53869 and https://github.com/advisories/GHSA-4pqm-j46f-795x

**Community / first-hand [F]/[T]**
21. HN "Hermes Agent – Open-source AI agent with persistent memory" (~Jun 2026; plagiarism dispute, sluggishness, homelab sysadmin, local Matrix) — https://news.ycombinator.com/item?id=48419000
22. Reddit "How to use it? Docker? VPS? Locally?", 2026-10-26 — https://www.reddit.com/r/hermesagent/comments/1xg9zld/how_to_use_it_docker_vps_locally/
23. Reddit "Running Locally, Really?", 2026-04-16 — https://www.reddit.com/r/hermesagent/comments/1snju8w/running_locally_really/
24. Reddit "Hermes Desktop now sets up local models in one click", 2026-09-07 — https://www.reddit.com/r/hermesagent/comments/1w9hp7f/hermes_desktop_now_sets_up_local_models_in_one/
25. Reddit "Do people actually use this thing using only local resources?", 2026-09-06 — https://www.reddit.com/r/hermesagent/comments/1w8sbpz/do_people_actually_use_this_thing_using_only/
26. Reddit "One DGX Spark for Hermes", 2026-10-25 — https://www.reddit.com/r/hermesagent/comments/1xexl4f/one_dgx_spark_for_hermes_usable_on_its_own_or/
27. Reddit "My frustrating experience with Hermes" — https://www.reddit.com/r/hermesagent/comments/1tbj6cx/my_frustrating_experience_with_hermes
28. Reddit "Hermes is worth anything?" — https://www.reddit.com/r/hermesagent/comments/1ufaut8/hermes_is_worth_anything
29. Reddit "Hermes burnout" — https://www.reddit.com/r/hermesagent/comments/1xj6mxz/hermes_burnout
30. Reddit "Hermes is too much a yes-sir" — https://www.reddit.com/r/hermesagent/comments/1tfzki3/hermes_is_too_much_a_yessir
31. Reddit "10 Days Into Hermes Agent: Workflows Keep Breaking After Reboot" — https://www.reddit.com/r/hermesagent/comments/1t0qg4b/10_days_into_hermes_agent_workflows_keep_breaking/
32. Reddit "Is Hermes worth it over Codex or Claude Code?", 2026-09-29 — https://www.reddit.com/r/hermesagent/comments/1wswbe5/is_hermes_worth_it_over_codex_or_claude_code/
33. Reddit "How much do you actually spend on Hermes per month?" — https://www.reddit.com/r/hermesagent/comments/1vqo5wh/how_much_do_you_actually_spend_on_hermes_per_month/
34. Reddit "Unpopular opinion: Hermes Agent takes the path of Openclaw" — https://www.reddit.com/r/hermesagent/comments/1tx1jt9/unpopular_opinion_hermes_agent_takes_the_path_of/
35. Reddit "Is there a better visual UI/Client for Hermes Agent right now?" — https://www.reddit.com/r/hermesagent/comments/1vmfy1z/is_there_a_better_visual_uiclient_for_hermes
36. Reddit (user-story tiles) — 2026-06/10 pool QC 1u5dtuh, 2026-09 drainlayer 1wom17w, 2026-10 165-person travel co 1x6n24u, 2026-10-19 field service 1x9xec6, 2026-09-27 school/tutor 1wra98z, 2026-06-23 28-cron-job user 1udesr1, 2026-06-30 Raspberry Pi 1uje4d8, 2026-08-04 50-dev team 1veu806, 2026-09-29 video SDLC 1wtpoar, 2026-09-17 bookstore 1wj0gnb, 2026-06-26 meeting prep 1ug5fae.

**Comparator [S]**
37. MantisClaw repo (MPL-2.0, Go, "no complex setup") — https://github.com/karayakar/MantisClaw (thin evidence; checked 2026-11-18)
