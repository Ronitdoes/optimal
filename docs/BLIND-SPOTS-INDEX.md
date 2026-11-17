# Blind-Spot Index — consolidated findings (2026-11-17)

> **Consolidates:** `BLIND-SPOTS-01-TEMPORAL.md` (15) · `02-INTERNATIONAL.md` (13) · `03-ENGINEERING.md` (15) · `04-BUSINESS.md` (15) · `05-HUMAN.md` (15) · `06-VERTICALS.md` (15) · `07-SAFETY.md` (14) = **102 findings**, each evidence-cited in its source report.
> **Method:** 7 parallel research subagents, each instructed to read the two source docs first and report only what is *not* covered or is now stale. Every report contains a prior-art check that explicitly excludes already-covered material. Source docs were authored through **2026-10-26**; research date **2026-11-17**.
> **How to use:** §2 gives the themes; §3 is the must-fix list; §4 maps every finding to a build step in `IMPLEMENTATION-PHASES.md`; §5 lists facts in the source docs that are now wrong.

---

## 1. Headline

The source docs are unusually strong on **mechanisms** (permission pipeline, threat model, receipts, social OS) and **use cases** (145+ vetted rows). The blind spots are not "more features" — they fall into five structural gaps:

1. **Oversight is treated as free.** Nothing measures whether a human confirm actually catches errors; PNAS 2024 shows HITL *raises uptake while lowering accuracy*; consent is a button, not a lifecycle; the harness's own UI has no accessibility target.
2. **The security boundary is in the wrong place.** Reader/Actor is called the strongest defense, but every in-band defense exceeds 90% ASR under adaptive attack. Capability isolation, SMT/temporal shielding, and kernel-boundary containment must be the real wall.
3. **Operational maturity is named but not specified.** The docs name AID/JWS/RFC 9421, event sourcing, "signed binary", and "full observability" — but omit key lifecycles, TUF/Sigstore updates, event schemas, migrations, protocol versioning, and OTel wire contracts.
4. **The product is US-only and unfundable as written.** Zero GDPR/EU AI Act/ISO posture, zero i18n/time-zone support, zero pricing/license/lawyer/insurer/entity — yet it is a product that can pay money from your PC.
5. **There is a three-week temporal hole.** GPT-6.2, Fable 5.2, OSWorld 2.1, the EU agent rules proposal, Linux Foundation Agent Name Service, ChatGPT Wallet, and a two-week autonomous intrusion all landed after the docs' cutoff.

**One-sentence synthesis:** *the architecture bet is right (persistence + clocks + receipts, local-first, deterministic gates), but the plan omits the human-factors, jurisdictional, lifecycle, and product layers that decide whether anyone can safely own and operate it.*

---

## 2. Cross-cutting blind-spot clusters

| # | Cluster | Reports | What is missing |
|---|---|---|---|
| **C1** | **Meaningful oversight** | 05, 07 | Trust calibration, `reviewability` test, consent ledger, fatigue KPIs, ambient channel, takeover budget, contestability, duress mode |
| **C2** | **Real security boundary** | 07, 03, 01 | Capability isolation over in-band filters; SMT/temporal policy monitor; gVisor/microVM tiers; anti-self-modifying guards; RAG retrieval security; provenance binding |
| **C3** | **Operational contracts** | 03, 07 | OTel GenAI (pinned), key lifecycle + JWKS rotation/revocation, TUF/Sigstore/SLSA updates, event-store schema, migrations, protocol versioning, actor locks, secret rotation |
| **C4** | **Jurisdiction & compliance** | 02, 05, 07, 01 | EU AI Act + Omnibus dates, GDPR/UK DUAA Art. 22, Data Act/DSA, EU KIDS Act, ISO 42001/27001 + NIST AI RMF, cross-border transfer gates, APAC regimes, compliance evidence mapping |
| **C5** | **Global capability** | 02, 03 | i18n/RTL/CJK, IANA tzdb + DST + non-Gregorian calendars, UPI/Pix/SEPA/FedNow/M-Pesa rails, India Stack + wallets, international rule-change sources |
| **C6** | **Product viability** | 04 | Price tiers, Apache-2.0 + `ee/` split, legal entity/trademark, E&O insurance, governance/DevRel/SDK, distribution, support/SLA, marketplace economics, north-star metric |
| **C7** | **Temporal freshness** | 01 | New models/benchmarks/standards/incidents invalidate specific facts and one differentiator claim ("per-task routing") |
| **C8** | **Uncovered verticals** | 06 | FFL, aviation, FMCSA, mechanics liens, OSHA, CPA notices, FINRA U4, FDA QMSR, Amazon seller, I-9, PSLF, RCRA, youth leagues, hotel OTA — all ClockBook + receipts plays |
| **C9** | **Eval discipline** | 07, 01, 03 | Benchmark version pinning, contamination audit, Pass^k+CIs, property-based/fuzz/cut-point-replay CI, CI cost control |
| **C10** | **Long-run operations** | 03, 07 | Resource envelope/GC, offline/air-gapped profile, model-supply hot failover, energy/water/carbon telemetry |

---

## 3. Must-fix before the next capability phase (P0)

| # | Finding | Source | Why now |
|---|---|---|---|
| 1 | **Capability isolation is the boundary**, not Reader/Actor — in-band defenses >90% ASR adaptive | 07 B-01 | The plan's flagship injection defense is not a security property |
| 2 | **SMT/temporal policy monitor** for multi-step standing obligations; Z3 policy validation at load | 07 B-02 | Per-call tiers can't express "never send after vault.get" |
| 3 | **Kernel boundary tiers** gVisor + Firecracker/Kata microVM; no long-lived creds inside | 07 B-07 | All cited sandbox escapes were kernel/credential events |
| 4 | **Adversarial audit of our own benchmarks** (BenchJack-style) + version pin | 07 B-09, 01 #3#4 | Every design number may be inflated; OSWorld 2.0→2.1 changed the headline |
| 5 | **Meaningful Oversight Test + `reviewability` field** — HITL often degrades accuracy | 05 #1#2 | Otherwise confirms are rubber stamps with legal exposure |
| 6 | **Consent Ledger** with scope/TTL/revocation/signed receipts | 05 #6 | "Read mail, never send, revoke Friday" is inexpressible today |
| 7 | **Fatigue KPIs + ambient 4-state channel** | 05 #8 | "Fatigue-aware design" is currently unfalsifiable |
| 8 | **WCAG 2.2 AA for the harness's own UI** + low-stimulation mode | 05 #10, 03 #10 | ADA/EAA exposure and a self-own for an augmentation product |
| 9 | **Training Wheels onboarding** + reset-trust affordance | 05 #3 | First run sets the trust trajectory; none exists |
| 10 | **EU AI Act classifier + correct post-Omnibus dates** | 02 #1, 01 #5, 07 B-10 | Generally applicable since 2 Aug 2026; we are in scope as deployer/GPAI-based tool |
| 11 | **GDPR/UK DUAA ADM register + meaningful-involvement artifact** | 02 #2 | Core loop is the "solely automated decision" fact pattern; UK s.80 live 5 Feb 2026 |
| 12 | **Price tiers, license, legal entity, E&O insurance** | 04 #1#2#3#4 | Existential risk: standard terms cap liability at 12 months' fees |
| 13 | **TUF + Sigstore + SLSA updates with signature *verification*** | 03 #3 | "Signed binary" that clients never verify defends nothing |
| 14 | **OTel GenAI spans with pinned internal schema** | 03 #1, 07 B-06 | Long-horizon failures are undiagnosable without per-step spans |
| 15 | **Guard evolution without self-modification** (propose→verify→activate) | 07 B-03 | "Model can never dismiss its own gate" is a principle, not a mechanism |
| 16 | **EU KIDS Act / age-signal abstraction** (unknown = minor-safe) | 05 #12 | COM(2026) 681, 17 Sep 2026; OS age-signal duty contemplated |
| 17 | **Operation Snowball in threat model** + no-agent-rewrites-its-own-tooling guard | 01 #9 | 2-week adaptive real-world intrusion, no human in loop |
| 18 | **Stale-frontier corrections** (GPT-6.2/Fable 5.2/OSWorld 2.1/ANS/payments) | 01 #1#2#3#6#7§5 | Concrete facts in §§17–28 and Appendix C are now wrong |

---

## 4. Full gap → phase → step map

Step IDs below are the † additions in `IMPLEMENTATION-PHASES.md`. "Existing" = fold into an already-defined step.

### Report 01 — Temporal (15)
| Gap | Pri | Destination |
|---|---|---|
| GPT-6.2 Astra/Sol + blocker-deception threat | P0 | §14 routing; S1.5 finish gate; `PLAN.md` §22.2/App.C |
| Fable 5.2/Mythos 5.2; 1,000-agent proofs; API breaks | P0 | §31 TeamWork-Bench; pin tool versions |
| OSWorld v2.1 (binary 20.6→44.3%, partial 81.8–89.3%) | P0 | S1.9 eval integrity |
| Terminal-Bench 4.0/4.1 | P1 | S1.9 benchmark pin |
| EU agent-regulation proposal (11/16) | P0 | S7.5 compliance mapping |
| Linux Foundation Agent Name Service (11/10) | P1 | S2.12 identity lifecycle |
| ChatGPT Wallet one-shot token pattern | P1 | S5.6 payment rails |
| Dots multi-model + enterprise residency | P1 | `PLAN.md` §17/§20 corrections |
| Operation Snowball 2-week intrusion | P0 | S1.8 boundary; §24 threat update |
| ClawHavoc 1,100+ skills / 18.7% / scanner evasion | P1 | S2.2 plugin vetting |
| AI vuln-discovery flood / alert fatigue | P2 | S1.7 fatigue KPIs |
| T3 v0.1.10 Windows service + remote update | P1 | S8.7 updates; §19 re-audit |
| Muse Ember / Gemini 4 Carbon cheap tiers | P2 | §14 model routing |
| OpenAI→Cursor model shutoff (11/12) | P2 | S8.8 model-supply failover |
| MCP Tasks shipped; `denyInsteadOfPrompting` | P2 | S1.3 permission UX |

### Report 02 — International (13)
| Gap | Pri | Destination |
|---|---|---|
| EU AI Act + Omnibus dates + GPAI + Art. 50 | P0 | S7.5 |
| GDPR/UK DUAA ADM + DPIA/FRIA | P0 | S7.5 |
| EU Data Act + DSA agent applicability | P1 | S7.5 |
| Global payment rails (UPI/Pix/SEPA/FedNow) | P1 | S5.6 |
| India Stack + DPDP consent objects | P1 | S7.6 |
| ISO 42001/27001 + NIST AI RMF + ITU | P1 | S7.8 |
| Cross-border transfer / residency | P1 | S7.5 |
| i18n / locale / non-English semantic anchors | P1 | S7.7 |
| Time zones, DST, non-Gregorian calendars | P2 | S7.7 |
| APAC law patchwork (Korea/Japan/China/Canada/AU) | P2 | S7.5 |
| Non-US identity/wallet/benefit systems | P2 | S7.6 |
| Africa mobile money + LATAM rails | P2 | S7.6 |
| International rule-change registries | P3 | S6.8 |

### Report 03 — Engineering (15)
| Gap | Pri | Destination |
|---|---|---|
| OTel GenAI + cross-surface trace propagation | P1 | S2.8 |
| Key lifecycle for AID/JWS/RFC 9421 | P1 | S2.12 / S8.6 |
| TUF + Sigstore + SLSA secure updates | P1 | S8.7 |
| Event-store / durability implementation contract | P2 | S2.9 |
| DB schema versioning & migrations | P2 | S2.9 |
| Protocol versioning & compatibility | P2 | S0.7 / S2.9 |
| Actor concurrency + cross-surface locks | P2 | S2.10 |
| Property-based / fuzz / cut-point-replay testing | P2 | S2.11 |
| Secret rotation / short-lived credentials | P2 | S8.6 |
| Own-UI accessibility | P2 | S8.5 (pulled to P0 via 05 #10) |
| Latency budgets / p95 regression gates | P3 | S4.6 |
| i18n / localization architecture | P3 | S7.7 |
| Offline / air-gapped profile | P3 | S8.8 |
| Long-run memory / resource envelope | P4 | S8.8 |
| Benchmark CI cost + variance control | P4 | S8.9 |

### Report 04 — Business (15)
| Gap | Pri | Destination |
|---|---|---|
| Pricing / packaging / WTP experiments | P0 | S9.6 |
| License + OSS/commercial split | P0 | S9.5 |
| Liability / E&O posture | P0 | S9.7 |
| Legal entity / trademark / patent | P1 | S9.5 |
| Governance model | P1 | S9.8 |
| DevRel / community | P1 | S9.8 |
| SDK / docs / API strategy | P2 | S9.8 |
| Distribution channels | P1 | S9.9 |
| Partnerships (AAIF/MS/insurer) | P2 | S9.9 |
| Support model / SLA | P2 | S9.9 |
| Marketplace economics | P1 | S9.10 |
| Enterprise pilots / sales motion | P1 | S9.9 |
| North-star metric (WST/AU) | P1 | S9.10 |
| Competitor pricing table | P2 | S9.6 |
| Regulatory cost + investor narrative | P1 | S9.7 |

### Report 05 — Human (15)
| Gap | Pri | Destination |
|---|---|---|
| Trust calibration / reliance accuracy | P0 | S1.7 |
| HITL not a safety property; reviewability | P0 | S0.9 |
| Onboarding / Training Wheels | P0 | S0.8 |
| Mental models / Capability Card | P1 | S0.8 |
| Cognitive forcing functions | P1 | S1.7 |
| Consent Ledger lifecycle | P0 | S1.7 |
| Counterfactual cards + Contest action | P1 | S1.7 |
| Fatigue KPIs + ambient channel | P0 | S1.7 |
| Takeover Time Budget + context packet | P1 | S1.7 |
| Own-UI WCAG 2.2 AA + low-stimulation | P0 | S8.5 |
| Dark-pattern manipulation signal | P1 | S3.6 |
| EU KIDS Act / age-signal layer | P1 | S6.9 |
| Coercion-aware confirm + duress mode | P1 | S1.7 |
| Household Boundary Map | P1 | S6.9 |
| Capability-retention KPI / labor ethics | P2 | S9.10 |

### Report 06 — Verticals (15)
All → **S6.10** (ClockBook + receipts slices): pilot medical/currency · FFL · FMCSA · mechanics liens · OSHA · CPA notice clocks · FINRA U4 · insurance commissions · FDA QMSR · Amazon/FBA seller · I-9/E-Verify · PSLF/IDR · RCRA · youth sports · hotel OTA recon.

### Report 07 — Safety (14)
| Gap | Pri | Destination |
|---|---|---|
| B-01 capability isolation over in-band PI defenses | P0 | S1.8 / S3.2 |
| B-02 SMT/temporal shielding + Z3 policy validation | P0 | S1.8 |
| B-03 guard evolution without self-modification | P1 | S2.14 |
| B-04 activation probes for deception (soft signal) | P1 | S1.10 |
| B-05 conformal abstention / calibrated deferral | P1 | S1.10 |
| B-06 OTel GenAI wire format | P1 | S2.8 |
| B-07 gVisor + microVM containment tiers | P0 | S1.8 |
| B-08 RAG retrieval-path security | P1 | S2.13 |
| B-09 benchmark contamination audit | P0 | S1.9 |
| B-10 EU AI Act technical evidence mapping | P1 | S7.8 |
| B-11 energy / water / carbon telemetry | P2 | S7.9 |
| B-12 agentic IAM / zero standing privilege | P1 | S2.12 |
| B-13 A2A v1.1 / AP2 v0.2 / x402 V2 deltas | P2 | S7.9 |
| B-14 model/tool/trace provenance binding | P1 | S2.14 |

---

## 5. Facts in the source docs that are now wrong

| Source doc claim | Correct as of 2026-11-17 |
|---|---|
| "Frontier = GPT-6 Astra (9/3); GPT-6.1 shelved" | GPT-6.2 Astra/Sol shipped 11/17 (first "High self-improvement"); GPT-6-Cyber 11/5 |
| "Claude Opus 5 / Fable 5" | Fable 5.2 / Mythos 5.2 (11/9), 1M context, breaking API changes |
| "OSWorld 2.0 binary ~20.6%" | v2.1 recommended; ~44.3% binary, 81.8–89.3% partial |
| "Terminal-Bench 2.0" | 4.0 / 4.1 current |
| "No ratified agent-identity standard" | Linux Foundation Agent Name Service (11/10), 11 supporters |
| "Design a payments seam, never auto-pay" | ChatGPT Wallet (11/13) ships substitute token + biometric confirm — the pattern is now table stakes |
| "Per-task model routing is our differentiator" | Dots added in-task multi-model (11/3); differentiate on *inspectability* + BYOK instead |
| "Worst case = 34.5h AISI attack" | Operation Snowball: 2+ weeks, 14 orgs, adaptive self-repair |
| "T3 v0.1.3, read-only Windows snapshots" | v0.1.10 (11/17) Windows service + remote update; re-audit |
| "900+ malicious skills" | 1,100+; 18.7% of popular skills; scanner-evasive |
| "Reader/Actor is the strongest published defense" | Contestable: in-band defenses collapse under adaptive attack; capability isolation is the boundary |

---

## 6. Net roadmap impact

1. **Pull forward** into P0/P1: real security boundary (S1.8), meaningful oversight + consent (S0.9/S1.7), eval integrity (S1.9), licensing/pricing/liability (S9.5–S9.7, start as a parallel track).
2. **Add** two whole new tracks: **S7.x jurisdiction/global** and **S8.x lifecycle/hardening**, formerly implicit.
3. **Expand** P6 with 15 new regulated-SMB verticals (S6.10) and the household/age boundary (S6.9).
4. **Correct** the stale facts in §5 inside `PLAN.md` before the next published revision.
5. **Keep** the original build order intact — none of these findings changes the core sequence (safety → coding → extensibility → browser → GUI → comms → obligation custody); they add the missing layers around it.

---

## 7. Files in `docs/`

| File | Contents |
|---|---|
| `PLAN.md` | Original architecture/strategy source (unmodified copy) |
| `USE-CASES-EXPANSION-V2.md` | Original use-case expansion source (unmodified copy) |
| `BLIND-SPOTS-01-TEMPORAL.md` … `07-SAFETY.md` | Seven full research reports with evidence and sources |
| `BLIND-SPOTS-INDEX.md` | This consolidated index |
| `IMPLEMENTATION-PHASES.md` | Phased build plan with discrete steps, gates, and the † integration |

*Every priority label and concrete recommendation in this index traces to a dated, URL-cited finding in the per-report files. Nothing here is speculative beyond what those reports flag as such.*
