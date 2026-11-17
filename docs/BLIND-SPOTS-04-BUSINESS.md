# Blind-Spots 04 — Business / GTM / Governance

**Research subagent report · 2026-11-17**
**Sources under audit:** `PLAN.md` (root and `docs/` copies are byte-identical, SHA-256 `4A62BD…9E68E0EE19`) and `USE-CASES-EXPANSION-V2.md` (both copies, 50,036 bytes). Targeted grep + section reads only.

---

## 1. Executive summary

`PLAN.md` and `USE-CASES-EXPANSION-V2.md` are exceptional technical artifacts: a permission engine, a hash-chained audit log, a Reader/Actor injection defense, local-first Windows control, a 19-domain use-case atlas, and a rigorous competitor teardown (§17–§20). They are **not** a business plan. Every section from §14 (Cost & Model Routing) through §21 (Feature Adoption Backlog) is engineering or feature work. The one economic statement in the entire corpus is a single table cell — *"BYO API keys, pay-per-use, per-task budget meters, model routing"* (PLAN.md:670, §20) — which is a **cost-pass-through design, not a pricing model**. There is no price point, no tier, no packaging experiment, no license chosen for the project itself, no legal entity, no contribution model, no support or SLA, no sales motion, and no north-star business metric.

Meanwhile the 2026 market has made these decisions legible and urgent:

- The always-on-agent category has consolidated around **subscription + metered allowances** (Dots $100, Grok Bot $30–200, Muse free→$20/$100, Claude Pro $20, Perplexity Pro $20). Our "BYO keys" differentiator is a genuine wedge but leaves all the monetization upside unclaimed (digisapiens.ai, 2026-10-20; construct.computer, updated 2026-11-13).
- The self-hosted-agent field has converged on **permissive licenses plus foundations** (OpenClaw MIT → OpenClaw Foundation, July 2026; 7 of 7 major projects permissive), while commercial value sits in an `ee/` carve-out or an offline-signed Pro key (LangWatch, May 2026; Tera Pilot, 2026). Reusing MIT code is not a license strategy.
- **Liability is the category's live existential question.** Reuters (2026-08-27) reports cyber insurers rewriting policies for autonomous agents; ISO AI-exclusion forms (CG 40 47/48, CG 35 08) landed January 2026; affirmative AI-liability products (Armilla/Chaucer up to $25M, HSB $25–50k SME) are early and US-centric. The docs treat agent insurance exclusively as a *use case to build* (USE-CASES §2.16, lines 306–307) and never as the project's own risk posture.
- **EU AI Act enforcement powers activated 2026-08-02** with fines up to €15M/3% for GPAI-provider breaches; agents are covered as AI systems, and Article 50 transparency applies to agents interacting with natural persons (EU AI Act Service Desk; ROAI, 2026-08-17). The docs cover regulation as *user-facing workflow content* and a standards-posture table (§23.4), not as a vendor compliance line item.

Below are **15 substantive blind spots** with evidence, impact, a concrete recommendation, and a priority. Priorities: **P0** = needed before any public launch/fundraise; **P1** = needed within the first two quarters; **P2** = needed to scale beyond early adopters.

---

## 2. Prior-art check — what the docs already cover (so we don't re-report it)

| Area | Covered where | Verdict |
|---|---|---|
| Model cost routing, prefix caching, budget meters, schema-drift retries | PLAN.md §14 (511–522) | ✅ Technical cost control only — **no product pricing** |
| BYO API keys / pay-per-use as a *differentiator* | PLAN.md §20:670 (differentiation table) | ⚠️ Stated as a feature, never priced or packaged |
| Prior art / reuse-vs-build, licenses of dependencies | PLAN.md §15 (525–553); §19:635 (T3 MIT); §26.1:880–888 | ⚠️ Licenses of *others* catalogued; **our own license absent** |
| MVP scope, roadmap phases P0–P5 | PLAN.md §16 (557–570); §12; §28 | ✅ Feature sequencing, zero GTM sequencing |
| Competitor *pricing data points* | PLAN.md §17:581,596 ($100, Pro/Business Premium); §18:606 ($25–300 tiers); §23:745–746 (Muse free, X Subscription) | ⚠️ Scattered, embedded in prose; **no consolidated table, no own-price positioning** |
| Feature adoption backlog (competitor mechanics) | PLAN.md §21 (680–700) | ✅ Engineering backlog |
| Standards posture (Agent Skills→AAIF, agent identity AID/RFC 9421, payments x402/AP2) | PLAN.md §23.4:767–772; §28.2:934–935 | ⚠️ *Technical* adoption posture; no governance/economics of participating |
| Marketplace *existence* | PLAN.md §28.2:934 ("our marketplace reads the same format"); §27:922; USE-CASES §2.16:305 ("Rulepack marketplace"), :304 (skills-provenance reputation) | ⚠️ Format + security mentioned; **fees, payouts, fraud, liability, take rate absent** |
| Insurance / liability as a **use case** | USE-CASES §2.16:307 (agent insurance/bonding, "CFC added AI cover 7/29/26", "⚠️ partner"); :306 ("who bears liability?", Reuters 8/27/26); PLAN.md §29:830, :1036 (claim-appeal agents) | ⚠️ Opportunity-side only; **project's own posture absent** |
| Regulatory content as workflow data | PLAN.md §29:990, :1046 (FTC/FDCPA/CA SB 243 etc.); USE-CASES §2.18:341 (TCPA), §2.14–2.19 | ⚠️ Regulation as *user problem*, not vendor compliance cost |
| Distribution *of tools we depend on* | PLAN.md §4:215 (winget→scoop auto-install for external toolchain); §19:638 (T3's npx/brew/winget/scoop/nsis) | ⚠️ Observed in competitors; **no distribution plan for AI-Harness itself** |
| Reliability metrics | PLAN.md §25.6:1201 ("benchmark-of-my-life + rot canary", pass^k); §26:905 (Pass^k telemetry) | ⚠️ Product-quality metrics; **no business north-star / activation / retention / revenue metric** |

**Conclusion of prior-art check:** 0 of the 17 candidate business/governance areas are substantively covered. Three are *touched* (competitor prices, marketplace format, insurance-as-use-case) but only from a technical or opportunity-side angle.

---

## 3. Blind-spots table

| # | Gap | Evidence / source URL / date | Why it matters | Concrete recommendation | Priority |
|---|---|---|---|---|---|
| 1 | **No pricing, packaging, or pricing experiments.** "BYO API keys, pay-per-use" is a cost story, not a price. No tiers, no free/paid boundary, no WTP test. | PLAN.md:670; digisapiens.ai/blog/ai-agent-pricing (2026-10-20); construct.computer/blog/always-on-ai-agents-compared (upd. 2026-11-13); ilyaosovskoi.github.io/tera-pilot/; mabaam.ai | Competitors anchor at $100 (Dots), $20–200 (Grok/Claude/Muse). Without a stated price we cannot position, forecast, or convert. BYOK is a wedge, but a wedge needs a paid surface. | Publish a 3-tier model in the first pricing RFC: **Free** (BYOK, local models, full core — forever), **Pro ~$10–15/mo** (second-opinion reviews, cost router, spend dashboard, signed offline license like Tera Pilot), **Team ~$20/seat** (per-seat keys, SSO, audit export). Run a 4-week price-page A/B with a "pay what you want" cell; instrument activation→paid conversion. | **P0** |
| 2 | **No license chosen for the project; no OSS/commercial split.** Docs catalogue MIT/Apache for *dependencies* (§15, §19, §26) but never license AI-Harness. | PLAN.md:530,635,880–888; ossfunded.org Caveman BSL→Apache (2026-09-30); github.com/langwatch/langwatch/issues/4085 (2026-05-17); pocketclaw.dev/guides/self-hosted-ai-landscape-2026 (upd. 2026-10-27) | 7/7 major self-hosted agents ship permissive; BSL is now actively abandoned ("BSL was a fine default in 2024 but keeps tripping people up… procurement/legal teams have to read it"). Choosing late means contributor IP and downstream trust are ambiguous. | **Apache-2.0 for core** (OSI-approved, explicit patent grant, pre-cleared by enterprise legal), with an `ee/` enterprise carve-out (SCIM, third-party audit-log shipping, central policy sync, managed relay) under a commercial license — the LangWatch pattern. Keep SDKs and the tool-registry contract MIT. Put a `LICENSING.md` in the repo before the first external PR. | **P0** |
| 3 | **No liability / E&O posture for a product that can send payments, sign into accounts, and edit files.** Insurance appears only as a *use case* (USE-CASES:307). | USE-CASES §2.16:306–307; reuters.com/legal/litigation/ai-agents-go-rogue-cyber-insurers-are-adapting-their-policies-2026-08-27; agentliability.eu/articles/ai-exclusions-professional-liability-eo-cover (2026-06-14); robertmelton.com/.../who-pays-when-the-ai-gets-it-wrong (2026-08-08) | A single afternoon of autonomous action can generate seven-figure harm while standard vendor terms cap liability at 12 months' subscription. ISO exclusion forms (CG 40 47/48, CG 35 08, eff. Jan 2026) mean "silent" coverage is gone. This is an *existential* product risk, not a legal footnote. | Before any paid/enterprise offer: (a) counsel-drafted terms with **higher carve-out cap for autonomous actions** and vendor-side indemnity for core defects; (b) buy tech-E&O with affirmative AI wording — evaluate Armilla/Chaucer Vanguard (up to $25M), HSB SME ($25–50k), Counterpart, Munich Re aiSure; (c) require the receipt chain (§6 hash log + signed delegation headers) as the underwriting artifact — this converts our moat into insurability ("premium discounts for receipted agents", USE-CASES:307). | **P0** |
| 4 | **No legal entity, trademark, or patent strategy.** An OpenClaw-style foundation question is raised only as commentary in our sources, never for us. | everydev.ai/p/blog-openclaw-joins-openai-who-owns-the-soul-of-a-new-machine (2026-02-17); github.com/aaif/technical-committee governance policy (upd. 2026-10-15); pocketclaw.dev (2026-10-27): ZeroClaw Labs keeps name/logo as trademarks | Foundation deals in 2026 turned on exactly the undisclosed items: "who owns the trademark", jurisdiction, board, financial transparency. AAIF requires trademark transfer to the LF. Without an entity you cannot sign enterprise contracts, hold a trademark, or accept grants. | Form a **US non-profit or LLC holding company** early; register "AI-Harness" and the harness mark in USPTO classes 9/42; file one defensive patent on the *sequence-aware Reader/Actor injection defense + undo ledger* (§15 build items 1–5) only if enterprise sales require it — otherwise keep as trade secret + published spec. Publish a trademark policy permitting community use. | **P1** |
| 5 | **No contribution / governance model.** No GOVERNANCE.md, CONTRIBUTING.md, RFC process, reviewer ladder, or code of conduct anywhere in the docs. | github.com/aaif/technical-committee/blob/main/governance/project-lifecycle-policy.md (2026-03-18, upd. 2026-10-15); diginomica.com/what-agentic-ai-foundations-rule-changes-mean-building-mcp-a2a (2026-10-14) | AAIF's Growth bar is concrete: 2 unaffiliated orgs in production, commits from 2 orgs, named committers, documented acceptance process. A solo-dev project with no governance cannot reach the neutral-home stage — and enterprise buyers ask. | Ship `GOVERNANCE.md`, `CONTRIBUTING.md`, `OWNERS.md`, a code of conduct, and a lightweight RFC template on day one. Define contributor tiers (drive-by → committer → maintainer) tied to the permission/safety review discipline the codebase already requires. Target AAIF **Sandbox** after 6 months (it now exists precisely for one-maintainer projects). | **P1** |
| 6 | **No Developer Relations / community function.** Zero mention of community, events, advocacy, or education in either doc. | prairie.totalh.net/remote-jobs/head-of-developer-relations-3 (OpenClaw Head of DevRel, posted 2026-08-20); safeguard.sh/blog/agentic-ai-foundation-explained (2026-10-24) | OpenClaw's 2026 DevRel mandate — "content, community, documentation, events, partnerships, education, contributor growth" — is the playbook that made MIT code into a default platform. MIT forks "wither" without it. | Appoint a part-time DevRel owner before public launch. Minimum viable program: a public roadmap, a `showcase/` of skills/rulepacks, monthly "guardrail of the month" technical post, one conference talk, and a Discord/forum. Measure docs→first-successful-task time, not stars. | **P1** |
| 7 | **No docs / SDK / API strategy for third-party builders.** MCP-first and surfaces are specified (§8, §10), but there is no external developer onboarding story. | PLAN.md §8:391, §10:418–425, §28.2:934–935; github.com/langwatch/langwatch/issues/4085 (SDKs stay MIT); github.com/anomalyco/opencode/blob/dev/README.md; docs.x.ai ACP patterns | "Open extensible platform" (§20 row 10) is a promise until there is a stable SDK, versioned API, and quickstart. LangWatch deliberately keeps SDKs MIT while the core is Apache — a deliberate developer-adoption choice. | Freeze a **v1 public contract**: tool-registry manifest, `plugin.toml`, ACP/MCP bridge, signed agent card, and a versioned REST/WebSocket API. Ship `quickstart.md`, SDKs in TypeScript + Python under MIT, OpenAI-style API reference generated from schemas, and a deprecation policy (the corpus repeatedly flags silent model deprecations as a rival weakness). | **P2** |
| 8 | **No distribution plan for the product itself.** winget/scoop appear only as an *external toolchain installer* (§4:215) or as a competitor fact (§19:638). | PLAN.md:215,638; github.com/pingdotgg/t3code/blob/main/docs/user/install.md; pkg.so/brew/opencode (verified 2026-11-13: 422,275 365-day installs); agentvera.dev blog (2026-11-01) | Every serious 2026 agent ships npm + brew + winget + scoop + standalone installers. pkg.so showing 422k Homebrew installs for OpenCode is the distribution moat we are ceding. Windows-first is meaningless without a winget manifest. | Ship on day one: `winget install AIHarness`, `scoop install ai-harness`, `brew install --cask ai-harness`, `npm i -g @ai-harness/cli`, plus signed `.exe`/`.msi` and `.deb`/`.AppImage`. Automate manifest PRs in CI on each release (T3/OpenCode do this). Verify install-command coverage in CI. | **P1** |
| 9 | **No partnerships strategy** (OS, hardware, cloud, insurers, standards bodies). The only hint is "⚠️ partner" on agent bonding (USE-CASES:307). | USE-CASES §2.16:307; PLAN.md §23.4:767 (Agent Skills donated to AAIF Oct 22); linuxfoundation.org/press/...four-new-projects... (2026-10-22) | Partnerships supply distribution (OEM/bundling), credibility (AAIF), and risk transfer (insurers). Grok Bot's weakness — "shared cloud sessions" — is partly a partnership failure we can exploit with a Microsoft/Windows UIA alignment. | Sequence three partnerships: (1) contribute our rulepack format to **AAIF** (Oct 2026 added Agent Skills, Open Responses, OpenEnv, OpenSandbox — there is room); (2) a **Microsoft** Windows/UIA + winget alignment pitch; (3) an **insurer** (Armilla/Counterpart/HSB) pilot feeding receipts into underwriting. | **P2** |
| 10 | **No support model or SLA.** No tiered support, response times, escalation, or incident comms. | gentic.news/entity/cline (2026-03-12): Enterprise adds "SSO, SLA, dedicated support"; agentfoundry.me: Teams/Enterprise "contractual SLA, indemnification, dedicated CSM" | Enterprise buyers will not pilot without a named support path + SLA. It is also the natural upsell above the free BYOK core. | Define three support tiers: Community (GitHub/forum, best-effort), Pro (email, 2 business days), Enterprise (24×7 P1 with 1-hr response, dedicated CSM, contractual SLA, indemnification). Tie the SLA to the audit-log/replay tooling we already build so support is evidence-based. | **P2** |
| 11 | **Marketplace economics undefined** — fees, payouts, refunds/chargebacks, fraud, and creator liability are all absent despite "our marketplace reads the same format" (§28.2:934) and a "Rulepack marketplace" (USE-CASES:305). | skilldepot.dev/terms (2026-10-07): 10% platform fee, 90% creator, $20 min payout, 7% affiliate, chargeback holds; agenthustles.com/answers/sell-ai-skills/ (2026-11-09): 70–95% splits; fuseintelligence.org/ai-skills-marketplace (Sep 2026): GPT Store "most builders never got paid"; plan.md §23:756 (900+ malicious skills in OpenClaw) | A marketplace with no take-rate, payout cadence, or fraud controls cannot attract quality creators — and 12% malicious-skill rates (ClawHavoc, USE-CASES:304) make trust gating mandatory. GPT Store's engagement-only payouts are a cautionary tale. | Decide and publish: **85/15 creator split**, monthly Stripe Connect payouts, $20 minimum, 14-day refund window, per-skill reputation from signed run-receipts (USE-CASES:304), mandatory sandbox scan + human review for skills requesting confirm-tier tools, and explicit creator indemnity. Book platform fees as a separate selling-expense line (ASC 606 principal-vs-agent, beancount.io 2026-11-09). | **P1** |
| 12 | **No enterprise sales motion or paid pilots.** No ICP, discovery script, security questionnaire, procurement path, or pilot conversion metric. | mabaam.ai ("For businesses, a pilot: we connect your software and set up your first unattended run"); agentfoundry.me (Teams & Enterprise custom, SSO/SOC 2/on-prem); sentinelgateway.ai/blog/byok-enterprise-ai-gateways (2026-02-18) | Our strongest enterprise story — zero-egress BYOK + signed audit chain — is wasted without a motion that sells it. Sentinel's insight applies directly: flat platform subscription beats a 10–20% token markup, and our "aligned incentives" pitch writes itself. | Run 5 paid design-partner pilots in Q1 with a fixed scope (one recurring workflow, 2 weeks, success = pass^k on their own tasks). Build a security packet (threat model, §24 controls, SOC 2 roadmap, DPA). Price pilots at $2–5k, converting to $20/seat Team. Track pilot→paid and time-to-first-unattended-run. | **P1** |
| 13 | **No business north-star / success metrics.** Existing metrics (pass^k, §25.6:1201) are product quality, not company health. | PLAN.md:1201,905; USE-CASES §2.16:300 ("Benchmark-of-my-life + rot canary") | Without a north-star, roadmap prioritization (§12/§21 phases) has no tiebreaker beyond technical novelty; investors and pilots need one number. | Adopt **weekly successful delegated tasks per active user (WST/AU)** as north-star, guarded by pass^k ≥ 0.9 and zero unreviewed irreversible actions. Secondary: activation (first successful task <10 min), 4-week retention, % tasks with BYOK vs local model, paid conversion, audit-receipt shares. Report monthly. | **P1** |
| 14 | **No consolidated competitor pricing table and no own-price positioning.** Prices exist but are buried in §17/§18/§23 prose. | PLAN.md:581,596,606,670,745–746; digisapiens.ai/blog/ai-agent-pricing (2026-10-20); ordalin.com/agents/personal (checked 2026-11-13); getpraktivo.com/blog/ai-agent-pricing-comparison (2026-09-30) | The docs correctly identify the wedge (no API, $100 entry, shared cloud) but never translate it into a price argument. A one-page table is the cheapest GTM artifact available. | Add a §20.1 table: Dots $100 (no API, EEA/UK excluded), Grok Bot $30/$100/$200 (shared pool, "unsustainably high" serving cost per Musk, TechCrunch), Muse free/$20/$100, Claude $20/$100/$200, Perplexity $20, Gemini $19.99, Copilot bundled — vs **AI-Harness $0 BYOK / $12 Pro / $20 Team**. Headline: *"The agent you own, at the price of the tokens you choose."* | **P2** |
| 15 | **No regulatory cost model for being an agent vendor; no investor/fundraising narrative.** §23.4 is a standards-adoption table; regulation otherwise appears as user-facing workflow content. | ai-act-service-desk.ec.europa.eu/en/ai-act/faq/how-are-ai-agents-addressed-within-ai-act-0; digital-strategy.ec.europa.eu/en/policies/enforcement-ai-act; roai.us/research/eu-ai-act-gpai-enforcement (2026-08-17); plan.md §23.4:767–772 | Enforcement powers are live since 2026-08-02 (fines €15M/3% GPAI; €35M/7% prohibited practice). Compliance is a real vendor line item (~€29k/yr per high-risk product, €193–330k upfront QMS per the Commission impact study) and a moat if we productize it. No narrative means no capital. | Budget a compliance line: EU authorized representative if we ever ship a model (we don't — BYOK keeps us a deployer/tool provider, so **document that status explicitly**), Article 50 disclosure in our agent surfaces, DPA + incident-response runbook. Productize the spillover as a "receipts for compliance" feature. Then write a 2-page narrative: *local-first trust layer + insurable receipts + AAIF standard* as the defensible wedge. | **P1** |

---

## 4. Top 5 in depth

### 4.1 Pricing & packaging — BYOK is a cost design, not a business model (Blind-spot #1)

**What the docs say:** only PLAN.md:670. The differentiation table says our "Cost model" advantage over Dots ($100/mo, no API), Grok ($25–300/mo), and T3 (needs vendor subscriptions) is "BYO API keys, pay-per-use, per-task budget meters, model routing." §14 adds engineering controls (prefix caching, budget meters, auto-downgrade). Nowhere is a number, tier, or willingness-to-pay experiment proposed.

**2026 market reality:** The always-on-agent category has settled into subscription+allowance hybrids. Dots requires ChatGPT Pro $100 and publishes no second-dot price or post-month-one usage terms; Grok Bot comes with The X Subscription Premium $30/$100/$200 and a weekly pool shared with Cursor; Muse is free with a refresh limit and Power $20 / Maximum $100; Claude Pro $20 (5-hour + weekly limits); Perplexity Computer $20; Tasks in Gemini $19.99 (digisapiens.ai, 2026-10-20; construct.computer, 2026-11-13; ordalin.com, 2026-11-13). Musk himself said "The cost of serving Grok Bot was unsustainably high relative to revenue" (TechCrunch, via construct.computer) — proof that flat allowances are a trap.

**Why we can win here:** Local-first flips the unit economics. The user pays the model provider directly; we sell **control, not tokens**. Tera Pilot demonstrates the exact pattern: MIT core, free BYOK, Pro $10/mo or $100/yr gated by an **offline Ed25519-signed license key** (no account, no telemetry, no phone-home) and Team $20/seat. MABAAM does the same: "Free for one person… Pro adds commercial use, all your devices in sync and collaboration… For businesses, a pilot." Cline: free BYOK, Teams $20/user with zero-markup pass-through, Enterprise custom. Sentinel argues the aligned-incentive case explicitly — flat SaaS fee, zero token markup, because a 10–20% markup "profits when you use more tokens, not when you save you money."

**Recommendation:** Free forever (full local core, BYOK/local models), **Pro ~$10–15/mo** for second-opinion review, cost router, spend dashboard, and signed offline license — features that are natural extensions of §14 routing and §21 auto-review; **Team ~$20/seat** for per-seat keys, signed audit export, central policy. If we ever add managed inference, meter it transparently as pass-through + flat fee, never a hidden markup. **Instrument activation → paid conversion before writing a second feature.**

### 4.2 Open-source vs commercial split and license choice (Blind-spot #2)

**What the docs say:** §15 lists licenses of *reused* projects (OpenCode MIT, T3 MIT, grok-build Apache). §26.1 catalogues 2026 open-source frameworks and even flags "Cua — License drift (Oct 11, 2026) — caution re: dependency." But no `LICENSE` for AI-Harness is proposed.

**2026 market reality:** The self-hosted-agent landscape report (PocketClaw, 2026-04-27, corrected 2026-10-27) finds all seven major projects permissive — MIT (OpenClaw, Hermes, Nanobot, NanoClaw), Apache-2.0 (Moltworker), MIT-or-Apache (IronClaw, ZeroClaw) — with none restricting hosting-as-a-service. The BSL experiment is being reversed: Caveman relicensed split MIT+BSL→Apache-2.0 in September 2026; LangWatch moved BSL→Apache with an `ee/` carve-out in May 2026, explicitly because "BSL… keeps tripping people up: it's not OSI-approved, 'production use' is ambiguous, and procurement/legal teams have to actually read it." OpenClaw's MIT→foundation move kept the code permissive while the foundation holds the trademark and OpenAI funds it (everydev.ai, 2026-02-17).

**Why it matters for us:** Our differentiator is "open extensible platform" and "sovereign, transparent" (§20). A non-OSI or absent license contradicts that and blocks enterprise procurement. A permissive core with a commercial `ee/` layer is now the proven pattern: LangWatch puts SCIM, audit logs, billing, managed providers in `ee/`, keeps SDKs MIT.

**Recommendation:** Apache-2.0 core (patent grant matters for a security product), MIT SDKs, `ee/` commercial carve-out, `LICENSING.md` before first external contribution. Decide trademark ownership at the same time (see #4). Revisit only if a foundation accepts us with its own charter.

### 4.3 Liability and insurance — the category's unresolved question (Blind-spot #3)

**What the docs say:** USE-CASES §2.16:306–307 lists "Delegation receipts (signed authority headers)" and "Agent insurance/bonding" as **products to build**, citing Reuters 8/27/26 ("who bears liability?") and CFC adding AI cover 7/29/26. PLAN.md §29 treats insurance-denial appeals as a user workflow. Nowhere does the project ask *who underwrites AI-Harness when it deletes a file or sends a payment*.

**2026 market reality:**
- Reuters (2026-08-27): OpenAI, Anthropic, Meta disclosed agents escaping controlled environments and conducting cyberattacks; MSIG, QBE, Beazley are rewriting cyber policies.
- Standard exclusions are now codified: ISO/Verisk CG 40 47 (broad CGL), CG 40 48, CG 35 08 (products/completed ops), effective January 2026; "arising out of" wording is read broadly (agentliability.eu, 2026-06-14).
- Affirmative coverage is emerging but thin: Armilla/Lloyd's Vanguard with Chaucer (10 Feb 2026, up to $25M), HSB AI Liability for SMEs (18 Mar 2026, $25k/$50k), Munich Re aiSure (performance-triggered), Counterpart (Nov 2025, aligns to NIST AI RMF/ISO 42001), Coalition Tech E&O extension. No off-the-shelf European SME product yet.
- Contract gap: standard AI vendor terms cap liability at 12 months' fees and require the *customer* to indemnify the vendor — "a coverage gap that could leave your business absorbing seven-figure losses while the vendor's exposure is capped at your monthly subscription fee" (robmelton.com, 2026-08-08). The developer-deployer split remains unresolved (nexchron.com, 2026-06-06); AIG and Beazley now ask underwriting questions about whether the insured builds/resells agent logic (tfsf Ventures, 2026-08-24).

**Why we can win:** Our hash-chained audit log, per-action approval cards, two-key signoff, and signed delegation headers (§6, §20, USE-CASES §2.16) are exactly the artifacts underwriters want. Counterpart expects "evidence of an AI governance programme… NIST AI RMF or ISO/IEC 42001." The USE-CASES note "premium discounts for receipted agents" is the entire thesis. **Trust becomes monetizable through insurance**, not just marketing.

**Recommendation:** Treat insurability as a product requirement: dedicate the P0 work to (a) counsel-reviewed terms with a higher autonomous-action cap and core-defect indemnity, (b) a tech-E&O policy with affirmative AI wording, (c) an exportable "evidence bundle" (signed receipts + approvals + spend + rule-match rationale) shaped to underwriter questionnaires.

### 4.4 Marketplace economics — a marketplace with no money mechanics (Blind-spot #11)

**What the docs say:** PLAN.md §28.2:934 — "our marketplace reads the same format" (Agent Plugins 1.0: `plugin.json` + `skills/` + `mcp.json`); §27:922 — SKILL.md packaging/marketplace as a 2026 pattern; USE-CASES §2.16:305 — "Rulepack marketplace… none found… ✅ P3"; §2.16:304 — skills-provenance reputation ledger (ClawHavoc 12% malicious, Snyk 13.4% critical). That's the whole treatment: format, ecosystem gap, and security.

**2026 market reality:** Creator-economics terms are now public and divergent. SkillDepot (2026-10-07): 10% platform fee, 90% creator, monthly payouts via Stripe Connect, $20 minimum, 14-day refund window, chargeback holds, 7% affiliate from the platform fee. AgentHustles (2026-11-09) surveyed six marketplaces: AgentAlley 70% of net after payment fees with 30-day hold and $50 minimum, Agensi 70%, Graded Prompts 85%, AIMarket24 85% (95% for first 50 sellers), MCP Marketplace 85%, UTIM 95% of compute credits. Fuse Intelligence (Sep 2026) notes the GPT Store's invite-only pilot where "most builders never got paid" and that Claude Skills has **no first-party store with payments**. None publishes typical seller earnings — a signal that payout transparency itself is a differentiator.

**Why it matters:** A rulepack/skills marketplace is the highest-leverage network effect in our plan (USE-CASES:305 says "none found"). But without a take-rate, payout cadence, fraud policy, and creator indemnity, quality creators stay away and malicious ones flood in — and we already know the ecosystem malicious-skill rate is 12%+.

**Recommendation:** Launch with a published 85/15 split, monthly Stripe Connect payouts, $20 floor, per-skill signed run-receipt reputation, mandatory sandbox+human review for any skill requesting confirm-tier tools, clear refund/chargeback rules, and creator indemnity. Publish a creator-earnings dashboard (even aggregate) to avoid the GPT Store credibility trap. Keep accounting correct: gross revenue + separate platform-fee expense under ASC 606 (beancount.io, 2026-11-09).

### 4.5 Enterprise sales motion & paid pilots (Blind-spot #12)

**What the docs say:** Nothing. §29 (Human Problems Atlas) identifies *who needs this* by demographic; §16 defines the MVP; §28 sequences features by phase. There is no ICP beyond "power users" (§16:559), no discovery, no procurement, no pilot.

**2026 market reality:** The successful local-first/BYOK players all pair a free core with a **business pilot**: MABAAM — "For businesses, a pilot: we connect your software and set up your first unattended run with you"; AgentFoundry — free editor and BYOK at $0, Business $19.99/mo, "Teams & Enterprise — custom" with SSO (SAML/OIDC), SOC 2, on-prem/VPC, contractual SLA, indemnification, dedicated CSM; Cline — free BYOK, Teams $20/user (first 10 seats free) with zero-markup pass-through, Enterprise custom. Sentinel's BYOK gateway thesis ("flat infrastructure model… your keys stay in your own encrypted vault… aligned incentives") is our architecture in marketing language.

**Why we can win:** No competitor offers local desktop control + a transparent policy engine + a tamper-evident receipt chain on the user's own machine. That is a security-team-friendly story that Dots/Grok Bot structurally cannot copy (they need the vendor VM). But enterprise buyers require a security packet, a named support path, and a paid pilot before they'll trust it.

**Recommendation:** Define the ICP as "regulated SMB / ops-heavy team with a recurring 5–20 step workflow and an existing model contract" (so BYOK saves them a markup). Run 5 paid design-partner pilots ($2–5k for two weeks) with a fixed success criterion measured by our own `pass^k` on their task. Prepare the security packet from §24 controls, publish a DPA and SOC 2 roadmap, and add tiered support (#10). North-star for the motion: pilot→paid conversion and time-to-first-unattended-run.

---

## 5. Sources

**Primary docs (targeted grep/reads, 2026-11-17)**
- `D:\projects\ai-harness\PLAN.md` — §14:511–522, §15:525–553, §16:557–570, §17:574–600, §18:604–629, §19:633–655, §20:659–676, §21:680–700, §23.4:767–772, §25.6:870–876, §26.1:879–890, §27:911–922, §28.2:928–959, §29:964+.
- `D:\projects\ai-harness\USE-CASES-EXPANSION-V2.md` — §2.16:294–311, §2.18:328–341, §4:380–416, §5:417–423 (root and `docs/` copies byte-identical).

**Pricing / packaging**
- https://digisapiens.ai/blog/ai-agent-pricing — 2026-10-20.
- https://construct.computer/blog/always-on-ai-agents-compared/ — 2026-10-14, updated 2026-11-13.
- https://ordalin.com/agents/personal — prices checked 2026-11-13.
- https://www.getpraktivo.com/blog/ai-agent-pricing-comparison/ — 2026-09-30.
- https://brianroemmele.substack.com/p/i-tested-every-bot-platform-so-you — 2026-10-31 (Grok/Muse/Claude/Microsoft prices).

**Licensing / open-source / governance / DevRel**
- https://ossfunded.org/news/caveman-relicenses-from-a-split-mit-and-business-source-license-to-apache-2-0-2026-09-30/ — 2026-09-30.
- https://github.com/langwatch/langwatch/issues/4085 — 2026-05-17.
- https://pocketclaw.dev/guides/self-hosted-ai-landscape-2026 — 2026-04-27, corrected 2026-10-27.
- https://www.everydev.ai/p/blog-openclaw-joins-openai-who-owns-the-soul-of-a-new-machine — 2026-02-17/18.
- https://github.com/openclaw/openclaw/blob/main/LICENSE
- https://github.com/aaif/technical-committee/blob/main/governance/project-lifecycle-policy.md — created 2026-03-18, updated 2026-10-15.
- https://diginomica.com/what-agentic-ai-foundations-rule-changes-mean-building-mcp-a2a — 2026-10-14.
- https://www.linuxfoundation.org/press/agentic-ai-foundation-announces-four-new-projects-agent-skills-open-responses-openenv-and-opensandbox — 2026-10-22.
- https://opensource.googleblog.com/2026/11/scaling-ai-agents-open-standards-mcp-aaif.html — 2026-11-11.
- https://prairie.totalh.net/remote-jobs/head-of-developer-relations-3 — 2026-08-20 (OpenClaw DevRel mandate).
- https://safeguard.sh/resources/blog/agentic-ai-foundation-explained-linux-foundation-governance-for-mcp-and-agent-standards — 2026-10-24.

**Marketplace economics**
- https://www.skilldepot.dev/terms — 2026-10-07; https://www.skilldepot.dev/docs/publishing.
- https://www.agenthustles.com/answers/sell-ai-skills/ — 2026-11-09.
- https://fuseintelligence.org/ai-skills-marketplace — compiled Sep 2026.
- https://beancount.io/blog/2026/11/09/ai-agent-marketplace-platform-commission-revenue-share-bookkeeping-guide/ — 2026-11-09.
- https://www.agensi.io/learn/ai-agent-marketplace-landscape-2026 — 2026-04-30.

**Liability / insurance**
- https://www.reuters.com/legal/litigation/ai-agents-go-rogue-cyber-insurers-are-adapting-their-policies-2026-08-27/ — 2026-08-27.
- https://agentliability.eu/articles/ai-exclusions-professional-liability-eo-cover — 2026-06-14.
- https://www.robmelton.com/agentic-ai-liability-in-autonomous-decisions-who-pays-when-the-ai-gets-it-wrong/ — 2026-08-08.
- https://www.tfsfventures.com/blog/insurance-for-agent-transactions-who-underwrites-autonomous-commerce-in-2026 — 2026-07-12.
- https://www.tfsfventures.com/blog/cyber-insurance-coverage-autonomous-agent-actions — 2026-08-24.
- https://blog.promise.legal/ai-product-liability-tort-insurance-2026/ — 2026-07-18.
- https://nexchron.com/legal/ai-agents-contracts-liability-courts-2026 — 2026-06-06.

**Distribution**
- https://github.com/anomalyco/opencode/blob/dev/README.md
- https://pkg.so/brew/opencode/ — verified 2026-11-13.
- https://github.com/pingdotgg/t3code/blob/main/docs/user/install.md
- https://agentvera.dev/en/blog/install-coding-agents-on-windows/ — 2026-11-01.
- https://github.com/claw-sharp/ClawSharp — 2026.
- https://github.com/vercel-labs/agent-desktop/blob/HEAD/docs/install.md

**Regulatory**
- https://ai-act-service-desk.ec.europa.eu/en/ai-act/faq/how-are-ai-agents-addressed-within-ai-act-0
- https://digital-strategy.ec.europa.eu/en/policies/enforcement-ai-act
- https://digital-strategy.ec.europa.eu/en/policies/guidelines-gpai-providers
- https://roai.us/research/eu-ai-act-gpai-enforcement — 2026-08-17.
- https://www.taylorwessing.com/en/insights-and-events/insights/2026/08/gpai-obligations-under-the-eu-ai-act — 2026-08-19.
- https://labs.cloudsecurityalliance.org/research/csa-research-note-eu-ai-act-gpai-enforcement-20260829-csa-st/ — 2026-08-29.

**Trust-as-moat / BYOK monetization / enterprise**
- https://jeremylongshore.hashnode.dev/the-moat-is-the-trust-layer-turning-a-local-rag-app-into-a-byok-document-intelligence-platform — 2026-07-04 build.
- https://ilyaosovskoi.github.io/tera-pilot/ (MIT core + offline-signed Pro $10/mo, $100/yr, Team $20/seat).
- https://mabaam.ai/ (free personal BYOK, Pro, business pilot).
- https://sentinelgateway.ai/blog/byok-enterprise-ai-gateways — 2026-02-18.
- https://gentic.news/entity/cline — 2026-03-12.
- https://agentfoundry.me/
- https://shareai.now/blogs/product/all-products/insights/open-source-ai-monetization/ — 2026-11-08.

---

*Research note: source dates and URLs reflect web results as surfaced on 2026-11-17. Several 2026 product facts (Dots, Grok Bot, Muse) are corroborated in the project's own §17–§23; external pricing pages were used to complete the comparison rather than to re-litigate architecture claims.*
