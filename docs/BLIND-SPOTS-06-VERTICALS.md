# Blind Spots #06 — Genuinely Uncovered Verticals (2026-11-17)

> Research subagent report. Baseline: `PLAN.md` §25 (Use-Case Library) / §29 (Human Problems Atlas) and `USE-CASES-EXPANSION-V2.md` (19 domains, 130+ rows, Oct 26 2026). Method: grep every candidate domain against both docs (root + `docs/` copies), then websearch/webfetch for 2025–2026 evidence. **Only rows that survived the prior-art check are below.**
> Market status legend: ✅ = no incumbent found this pass · ⚠️ = partial incumbent exists (named; wedge stated) · 💀 = saturated, dropped.
> Evidence honesty: figures are drawn from the primary/secondary sources in §5; months cited inline. No 2024-or-earlier-only claims were used. Where an incumbent exists it is named, not hidden.

---

## 1. Executive summary

The expansion doc is exhaustive on *consumer* pain, *family* paperwork, and the *agent-economy meta-layer*. It is thin on **regulated small-enterprise operations** — the owner-operator, the 2-person shop, the volunteer-run league — where a clock is statutory, the portal is a government mainframe, and the penalty is license loss rather than an annoyance.

The surviving gaps share the doc's own winning shape (**persistence + clocks + receipts**, §0 of the expansion doc), but pushed into *occupational licensing and compliance*:

- **Calendar-month clocks nobody computes for you.** FAA flight review (24 *calendar* months), instrument currency (6 approaches / 6 calendar months + a second 6-month self-cure window), FFL 3-year renewal, FINRA U4's 10-day vs 30-day amendment split, 1031's 45/180-day pair, RCRA's 45-day inquiry / 60-day exception report. Every one is a `ClockBook` row citing a governing text.
- **Multi-party receipt chains with real money at the end.** Mechanics-lien preliminary notice → filing → enforcement; FMCSA clearinghouse query → DQF → CDL; carrier-portal commission statements → agency ledger; ATF 4473 retention → inspection. These are `Relay Baton` / `Chase Ledger` / `Evidence Locket` shapes, not drafting.
- **Small operators are structurally skipped.** The incumbents that exist (Levelset, FleetRabbit, CCH, DeedSpring, eZOnlineFFL, Docyt) sell to fleets, enterprises, title plants, or generic coordinators. The 1–10-person licensee still works from paper, spreadsheets, and phone calls — exactly the GUI/telephony + local-vault wedge this harness is built for.
- **2026 regulatory forcing functions create urgency.** FDA QMSR took effect **Feb 2, 2026**; FMCSA full enforcement landed **Feb 2026**; IRS finalized PSLF rules **Oct 30, 2025** (effective **Jul 1, 2026**); ICE rewrote I-9 inspection policy **Mar 16, 2026**; ATF proposed new 4473 retention terms **May 6, 2026**; EPA moved all discrepancy/exception reports into e-Manifest **Dec 1, 2025**. Each forces a one-time re-tooling decision — the best possible entry moment for a local agent.

**Recommendation:** the top four (FFL, aviation, FMCSA, mechanics-lien clocks) are the cleanest fits: statutory calendars, zero-to-weak incumbents, four-figure-to-six-figure stakes, and sensitive records that argue for local-only custody.

---

## 2. Prior-art check — domains ruled OUT

These candidate domains from the mandate were **checked and rejected** because they are already in the docs (or saturated). "Covered by" cites the section/row.

| Candidate domain tested | Verdict | Covered by / why dropped |
|---|---|---|
| Agriculture — FSA deadlines, crop insurance | OUT | Expansion §1 #4 + §2.1 "Farm **NAD appeal office**" (FarmRaise, Aug 7 / Sep 30 2026 windows). Explicitly downgraded already. |
| Construction — open permits, inspections, change orders | OUT | §2.1 "**Open-permit closer** + inspection chase" (MA $1k/day, 5–10 day inspection delay) and "Change-order + slow-pay ledger." The *lien* and *OSHA* sub-domains are NOT covered → survive as rows 4–5. |
| Logistics — consumer package/customs claims | OUT | §2.3 "Missing-package claim choreography" (USPS 15/60-day), §2.17 "Small-parcel **customs appeals**" (de-minimis 8/29/25). FMCSA carrier side is NOT covered → row 3. |
| Hospitality — front desk / housekeeping | PARTIAL | No row at all; but PMS vendors + OTA-recon services (Docyt) make it an operator-tooling sale. Kept as a thin row 15, deprioritized. |
| Restaurant — tips, bar, food truck, market EBT | OUT | §2.14 "Restaurant **tip-compliance audit**", "Bar compliance stack", "Food-truck renewal filing", "Market EBT reconciliation". |
| E-commerce seller — takedowns, returns | OUT | §2.8 "False-takedown defense", §2.3 "Returns/refund-window ledger", §2.12 "Reseller part-out engine". Amazon *account-health/SAFE-T* clocks are NOT covered → row 10. |
| Real estate — escrow, HOA, title release, property tax | OUT | §2.3 "Escrow audit + PMI", §2.5 "HOA assessment-defense / pre-lien / violation-fairness", "Lien-release/title-in-hand chase", "Property-tax DIY". Generic transaction-coordination is now SaaS'd (DeedSpring/ListedKit/Qualia); only the **1031 exchange clock** survives → row 9 (⚠️). |
| Professional services — paralegal e-filing, dunning, ERP re-key | OUT | §2.7 rows 161–172. CPA **client-doc chase + IRS notice clocks** and **FINRA U4** are NOT covered → rows 6–7. |
| Scientific lab — grants/IRB | OUT | §2.7 "Faculty **grant/IRB clock chip**" (lapsed renewal = collect-nothing). Instrument calibration / IACUC re-review NOT covered → folded into row 9's manufacturing/quality note but not a standalone row (thin). |
| Music/film — PRO, royalties, rights clearance, E&O | OUT | §2.8 PRO/MLC, clip rights-clearance, royalty audit (💀 dropped), E&O/COI; §2.13 community-theater royalties. |
| Sports — race entry, season tickets, physicals, guide recerts | OUT | §2.11 rows 225, 233–234; §2.6 physicals-clearance. **Youth-league volunteer credentials + field permits** NOT covered → row 14. |
| Maritime/boating | OUT | §2.11 "**Marina boater-side agent**" (Dockwa), §2.14 "**Charter-fishing compliance**" (NOAA HMS). No further maritime gap found. |
| Medical/pharmacy — refills, prior auth, DME | OUT | §2.2 "Fertility pharmacy race", "CPAP/DME resupply clocks"; §29.6 "Pill-chain collapse"; §29.7 prior-auth monitor. Pharmacy *operations* side is enterprise SaaS-saturated — dropped. |
| Utilities/telecom | OUT | §2.5 "**Utility-bill auditor + solar True-Up audit**"; §2.3 "Utility/IRA rebate radar". |
| Fundraising/nonprofits | OUT | §2.13 Form 990 watchdog, grant post-award reports, 501(h) clock, church admin, volunteer no-show; §2.6 booster treasurer. |
| Emergency management / disaster | OUT | §2.13 SBA disaster-loan appeal, pre-emergency claim-prep, insurance underpayment file; §2.19 archive-before-redaction. |
| Public-sector / clerk ops | OUT | §2.18 county-clerk status calls, registrar phone-chase; §2.4 Regulations.gov comments, jury-duty. Citizen FOIA **appeal** follow-through is thin — not promoted. |
| Cemetery/funeral | OUT | §2.14 "**Funeral-doc chain**"; §2.4 "Digital afterlife wind-down", "Executor/probate console". |
| Cannabis compliance | SATURATED (💀) | Metrc/Leaflogix/Dutchie/Greenbits are well-funded seed-to-sale incumbents; NY/IL/OK all mandate Metrc (NY inventory-in-Metrc deadline **Jan 12, 2026**, delinquent-payment reporting moved to Metrc **9/15/26**). A local agent cannot win the system-of-record layer; dropped. |
| Waste/recycling — consumer recycling | OUT | No consumer gap; but **RCRA generator clocks** for small SQGs survive → row 13. |

---

## 3. New use-case table — 15 surviving rows

| # | Use case | Who | 2025–26 evidence (source, date) | Annual value | Market status | Why local agent wins |
|---|---|---|---|---|---|---|
| 1 | **Pilot medical + currency clock desk** (flight review 24 cal-mo; IFR 6/6/6; BasicMed exam 48mo + course 24mo; AME Special Issuance chase) | ~300k active US pilots; CFIs, Part 91 owners | 14 CFR 61.56(c) 24-calendar-month rule; 61.57(c)/(d) 6-approach + second-6-month + IPC ladder (eCFR, updated 2026); FAA opened BasicMed-renewal comment 6/25/26, ~50k users (Skyfarer 6/25/26); medical cert 1st/2nd/3rd-class tiers 6/12/24mo (FAA) | $300–800/review + $75–200 exam; grounded = lost charter/CFI income $1k+/day | ⚠️ Logbook apps (ForeFlight, LogTen) *display* currency; none chase AME/Special Issuance or renew BasicMed | Local logbook/PDF vault + calendar-month math + AME/FAA portal chasing; no cloud medical data |
| 2 | **FFL compliance desk** (3-yr renewal; ATF Form 8 Part II; 4473/A&D retention; state registry/CCW clocks; inspection-ready binder) | ~128,960 active FFLs (FY24); gunsmiths, pawnbrokers, collectors | ATF: renewal every 3 yrs ($90 Type 01), mailed ~90 days out, Form 8 Part II; ATF proposed 20/30-yr 4473 retention 5/6/26 (comments 8/4/26); A&D 20 yrs; 5 willful violations → revocation; unannounced inspections ≤1/12mo (ATF Quick Reference) | $90 renewal + $200–600 software; avoided revocation = business $50k–500k | ⚠️ eZOnlineFFL/RocketFFL sell forms + compliance software to dealers; none runs the *deadline + inspection-receipt* office on the dealer's own machine | Sensitive A&D/4473 data argues for local vault; agent phones FFLC when renewal packet is late (§2.18 telephony) |
| 3 | **FMCSA carrier compliance calendar** (annual Clearinghouse query, DQF, UCR $69, BOC-3, drug/alcohol program, CSA/IFTA) | Owner-operators, 1–10-truck carriers | FMCSA full enforcement Feb 2026 for new rules (CNS 2026); Clearinghouse "prohibited" status now triggers CDL revocation (Fleetrabbit 2026); annual query $1.25/driver; test rates 50% drug/10% alcohol; UCR $69/yr 0–2 vehicles (OTRucking 2026); civil penalties schedule update 5/30/25 | $1.25–$3k/yr per carrier; out-of-service = $500–2k/day revenue | ⚠️ FleetRabbit/Innovative Logistics publish checklists; tools target fleets, not the 1-truck owner | Multi-portal, multi-clock persistence + document receipts; phone (FMCSA/state DMV) as first-class surface |
| 4 | **Mechanics-lien preliminary-notice + filing clock** (state-by-state notice → lien → enforcement; waiver forms) | Subcontractors, suppliers, small GCs | CA 20-day preliminary notice — late notice loses work older than 20 days (MechanicsLienMgmt 2026); TX monthly second-tier notice; 90-day filing; CA 90-day enforcement after recording; 12 states require statutory waiver forms (Levelset 2026) | $5k–50k unpaid claims protected; 33.4% of contract in disputes (expansion §2.1) | ⚠️ Levelset = notice/waiver SaaS for contractors; no cross-state *detect-stall → file → collect* lifecycle with receipts | Deadline Ladder + Evidence Locket; local proof of delivery/note to owner survives vendor churn |
| 5 | **OSHA construction recordkeeping + safety-card expiry desk** (300/300A/301 logs, 7-day entry, Feb 1–Apr 30 posting, e-submit, OSHA-10/30, state cards) | 1–20-employee contractors | OSHA 300 log entry within 7 business days of learning of incident; records retained 5 yrs (1904.33(a)); 300A posted Feb 1–Apr 30; e-submission for NAICS-listed employers; OSHA-10/30 cards (Miter, Raken, ABC Carolinas 2026 guides) | Fines $1k–15k/citation; bid ineligibility | ⚠️ Raken/Miter/SafetyClerk sell safety apps to mid-size firms; owner-run crews still use spreadsheets | ClockBook for 7-day/annual cycles + local incident-evidence vault; drafts only, no automated filings |
| 6 | **CPA practice client-document chase + IRS notice clock** (CP2000 30-day response, 90-day Tax Court window, extension dates, PBC lists) | 1–10-person tax/accounting practices and their clients | IRS Topic 652: respond to CP2000 within 30 days (60 overseas); missing it → Statutory Notice of Deficiency + 90-day Tax Court petition (Uncle Kam 2026; IRS); Wolters Kluwer CCH document-analysis exists 2025–26 | $500–5k per notice; 60–100 hrs/season saved | ⚠️ CCH/Arvori = enterprise/analysis; no local practice-side chase-and-receipt office | Sensitive client books stay local; automation of the *chase* (not the drafting) is the wedge |
| 7 | **FINRA U4/U5 + CE clock desk** (30-day general amendment, **10-day** statutory-disqualification amendment, CE cycles, exam-retake windows) | Registered reps, small BD/RIA firms | FINRA: U4 generally 30 days, 10 days for statutory disqualification (finra.org); Regulatory Notice 25-13 effective **10/10/25** allows CRD recordkeeping; CE/registration amendments adopted 2025 (Legal500); reduced exam-retake waiting periods 7/1/26 | Registration lapse = $10k–100k lost commissions; fines | ✅ Web CRD is firm-facing; no rep-side personal deadline agent found this pass | Pure ClockBook + Chase Ledger; 10-day window is fatal if missed; local record of disclosures |
| 8 | **Insurance agency commission reconciliation + appointment/CE clocks** (carrier statements → agency ledger → chargebacks → 1099; license renewal, carrier appointments) | Independent agencies with 10–30 carrier appointments | "No. 1 tech problem … multiple carrier interfaces" + manual missing-payment search (FinAcct Solutions 2026); appointment timing a reported challenge (FirstConnect/Insurance Business 2026); Wingspan automates *carrier-side* onboarding/1099, not the agency's own multi-carrier reconciliation | 1–3% of commission revenue leak; $2k–20k/yr recoverable | ⚠️ Agency management systems (Applied etc.) and Wingspan exist at carrier/FMO layer; the small-agency reconciliation desk is open | GUI/portal scraping across dozens of carrier sites + local ledger = perfect harness task; no cloud commission data |
| 9 | **Small-device-manufacturer QMS clocks** (FDA QMSR transition, calibration/PM, CAPA due dates, ISO 13485:2016 records) | 1–50-person medical-device manufacturers, contract labs | FDA QMSR effective **2/2/26**, replaces QSIT with risk-based inspections under CP 7382.850; ISO 13485:2016 incorporated (FDA, AAMI, NAMSA 2026) | $10k–100k/yr audit readiness; warning-letter remediation $50k+ | ⚠️ QMS SaaS (MasterControl, Qualio) sells to funded manufacturers; micro-shops use spreadsheets | Local SOP/evidence vault + clock ledger for calibration and CAPA; inspection prep = receipts, not opinions |
| 10 | **Amazon/FBA seller account-health + reimbursement clock** (POA/appeal windows, SAFE-T, FBA lost/damage claims) | 1–5-person FBA/FBM sellers | Amazon shortened SAFE-T filing window from 60 → **30 days on 2/16/26** (Nova Analytics 2026); A-to-z appeal 30 calendar days; FBA reimbursement windows revised 3/31/25 and 10/23/24 (Seller Central) | $1k–20k/yr in recoverable reimbursements; account suspension = business-stop | ⚠️ Appeal agencies ("theappealguru") + reimbursement services exist; crowded but mostly human/outsourced | Local evidence vault (invoices, photos, case logs) + multi-portal clock/chase; don't outsource account credentials |
| 11 | **I-9 / E-Verify new-hire file audit** (3-yr/1-yr retention, §274A, audit-trail/e-sign rules, re-verification) | Small employers, HR-of-one, staffing agencies | ICE revised I-9 inspection fact sheet **3/16/26**, expanded "substantive violations"; paperwork penalties **$288–$2,861/form**, hiring violations $716–$28,619 (NFIB, Holland & Knight, Dentons 2026); retention 3 yrs from hire / 1 yr after termination (ICE) | $288–$2,861 per form; audit remediation 20–60 hrs | ⚠️ I-9 SaaS exists but enterprise-priced; small employers still paper/shoebox | Local document vault + self-audit checklist + deadline chase; employee PII never leaves machine |
| 12 | **PSLF / IDR recertification + servicer-transition clock** (120 qualifying payments, buyback, IDR recert, ECF employer signatures) | ~ millions of federal student-loan borrowers; public-service workers | ED final PSLF rules published **10/30/25**, effective **7/1/26** (MOHELA); PSLF buyback continues under Oct 2025 AFT settlement (TIAA/ProtectBorrowers 2025–26); IDR court actions shift dates (studentaid.gov) | $5k–50k forgiveness at stake; $100s/mo in wrong-plan overpayment | ⚠️ Student-loan trackers display balances; none runs the multi-servicer ECF/recert receipt chain | Local ledger of payments + employer certifications + deadline ladder; sensitive financial data stays local |
| 13 | **RCRA hazardous-waste generator clock desk** (e-Manifest exception/discrepancy, 45-day inquiry, 60-day report, biennial 3/1, SQG re-notify) | Small Quantity Generators (auto shops, labs, printers, manufacturers) | LQGs/SQGs must register for e-Manifest from **1/22/25**; all Exception/Discrepancy reports electronic via e-Manifest from **12/1/25**; 45-day inquiry / 60-day Exception Report; biennial report due **3/1/26**; SQG re-notify 9/1/25 cycle (EPA) | $2.5k–50k/day willful RCRA violations; avoid $10k+ consent orders | ⚠️ EHS SaaS (e.g., Trihaz, USWI) targets LQGs; SQGs underserved | ClockBook for 45/60-day + biennial + 4-yr re-notify; local manifest/receipt archive; agent monitors portal for overdue copies |
| 14 | **Youth-sports league volunteer-credential + field-permit desk** (annual background checks, SafeSport, concussion/SCA every 2 yrs, COI, city field reservation/RFP) | Volunteer-run leagues, 501(c)(3) clubs, rec departments | Annual JDP background check + SafeSport + biennial concussion/SCA certificates (Gilroy LL, EHYLLL 2026 handbooks); town rules: annual statewide criminal checks, coaching certification, COI, field-use contract, RFP selection (Flower Mound 2026; Voorhees NJ 2025) | 10–40 hrs/season admin; lost field access / insurance gaps = $1k–10k | ✅ Registration platforms (SportsConnect/PlayMetrics) handle signups, not the *expiry + city-compliance* chase | Receipted credential vault + phone/fax chase to towns and background-check vendors; volunteer-run = no IT staff |
| 15 | **Independent hotel OTA reconciliation + license renewal** (PMS↔OTA line-item match, VCC windows, state hotel license) | 1–20-room independent motels/hotels | OTA commissions 15–30% (20% typical independents, 2026); monthly recon too late — disputes must be caught inside VCC/dispute windows (Docyt, Evention, Innrly 2026); state license filing/renewal (FL DBPR hotel guide) | 1–3% of OTA revenue recovered; $500–5k/yr | ⚠️ PMS + paid recon services exist (Docyt); thin — deprioritized | Local PMS/statement vault + daily exception detection + receipted dispute drafts |

---

## 4. Top 5 in depth

### 4.1 FFL compliance desk (row 2) — *best overall*
- **Who:** ~128,960 active federal firearms licensees (ATF FY2024, cited in the 5/6/26 proposed rule), overwhelmingly 1–5-person dealers/gunsmiths.
- **Clock anatomy:** 3-year license term; ATF mails Form 8 Part II ~90 days before expiry; if not received by T-30 the dealer must call the FFLC (1-866-662-2750); file before expiry or switch to Form 7 + lose ability to operate; a Letter of Authorization can bridge pending renewals. 4473s retained until discontinuance under current 27 CFR 478.129(b) (ATF proposes 20/30 years as of 5/6/26, comments closed 8/4/26); A&D records 20 years; multiple-sale/theft forms 5 years.
- **Evidence:** ATF FFL page + FFL Quick Reference + 91 FR 24413 (5/6/26). ATF inspections are unannounced, generally ≤1 per 12 months, and **a single willful violation can revoke the license**.
- **Value:** $90 renewal is trivial; the real value is avoiding revocation of a business worth $50k–500k and surviving an inspection without a 30-day records scramble.
- **Market:** eZOnlineFFL/RocketFFL sell forms and recordkeeping software, but they are *vendors*, not an agent that watches the calendar, calls the FFLC, and assembles the inspection binder. ⚠️ with a clean wedge.
- **Why local wins:** A&D/4473 data is the most sensitive possible — a local-first vault is a feature, not a compromise. Maps directly to `ClockBook` + `Evidence Locket` + the telephony `Hold-forest` (call FFLC on T-30) from expansion §3/§2.18.

### 4.2 Aviation pilot medical + currency clock desk (row 1)
- **Who:** ~300k active pilots; the acute segment is instrument-rated private/commercial pilots and CFIs whose income depends on being legal *today*.
- **Clock anatomy:** 14 CFR 61.56(c) — flight review within the preceding **24 calendar months** (mid-month completion is valid through month-end, a classic miscount); 61.57(c) — 6 approaches + holding + course tracking within 6 calendar months for IFR; 61.57(d) — a second 6-month self-cure window, then a mandatory IPC; medical certificate tiers (1st 6mo / 2nd 12mo / 3rd 24mo under 40, 12mo at 40+); BasicMed physician exam every 48 months + online course every 24 months, with FAA reopening the BasicMed-renewal comment period 6/25/26 (~50,000 users).
- **Evidence:** eCFR 14 CFR 61.56 / 61.57 (2026), FAA BasicMed pages, Skyfarer 6/25/26, AOPA FAQ.
- **Value:** $300–800/flight review, $75–200/medical exam, $400–800/IPC; the real loss is being grounded (CFI/charter income $1k+/day).
- **Market:** ⚠️ ForeFlight/LogTen display currency and log approaches; none manages the AME Special Issuance chase, the BasicMed course/exam pair, or the airport/FBO scheduling.
- **Why local wins:** the agent reads the *pilot's own logbook and medical PDFs*, computes calendar-month math locally, and stages confirm-gated bookings — no cloud service should hold medical data.

### 4.3 FMCSA carrier compliance calendar (row 3)
- **Who:** owner-operators and 1–10-truck carriers, the segment too small for a compliance department.
- **Clock anatomy:** annual Drug & Alcohol Clearinghouse query ($1.25/driver), driver qualification file, UCR ($69/yr for 0–2 vehicles), BOC-3, annual/periodic drug (50%) and alcohol (10%) testing, IFTA quarterly, CSA scoring under the reorganized categories, and — new in 2026 — state DMVs actively revoking CDLs for drivers with unresolved "prohibited" Clearinghouse status.
- **Evidence:** FMCSA Clearinghouse/CSA pages, Federal Register 5/30/25 civil-penalty update, CNS "FMCSA Rule Changes 2026" (full enforcement Feb 2026), Fleetrabbit/OTRucking 2026 checklists.
- **Value:** $1.25–$3k/yr direct cost; out-of-service orders cost $500–2k/day.
- **Market:** ⚠️ checklists and fleet tools (FleetRabbit) exist; the single-truck operator is unserved.
- **Why local wins:** a `Deadline Ladder` per driver per regulation, with receipts from FMCSA/state portals and a phone bridge for the Clearinghouse/DMV, all on the operator's machine.

### 4.4 Mechanics-lien preliminary-notice + filing clock (row 4)
- **Who:** subcontractors and material suppliers who can lose lien rights purely by missing a notice date.
- **Clock anatomy:** CA 20-day preliminary notice (a notice sent on day 40 protects only the prior 20 days of work); TX monthly second-tier notice; lien filing ~90 days from last work; CA 90 days to enforce after recording; 12 states mandate statutory waiver forms.
- **Evidence:** MechanicsLienManagement "Preliminary Notice — All 50 States Compared (2026)", TerraPC state-deadline roundup, Levelset waiver-form guide (2026).
- **Value:** $5k–50k of unpaid work preserved; the expansion doc already quantified 33.4% of contract value in disputes and 38% waiting on payment.
- **Market:** ⚠️ Levelset is a real incumbent for sending notices and waivers — but it is contractor-facing notice SaaS, not a cross-state *detect-stall → file → enforce → collect* lifecycle owned by the subcontractor with local proof-of-delivery.
- **Why local wins:** `Deadline Ladder` + `Evidence Locket` (certified-mail receipts, job photos, GPS) + `Chase Ledger` against the GC — the same primitives the expansion doc already credits with winning the wage-claim and lemon lanes.

### 4.5 FINRA U4/U5 + CE clock desk (row 7)
- **Who:** registered representatives and 1–20-person broker-dealer/RIA firms self-managing compliance.
- **Clock anatomy:** Form U4 must be amended within **30 days** of learning of most reportable events, but within **10 days** for a statutory disqualifying event; Art. 5 §2(c) of the FINRA By-Laws; Regulatory Notice 25-13 (effective 10/10/25) permits CRD recordkeeping; CE/registration amendments adopted in 2025; reduced exam-retake waiting periods announced 7/1/26.
- **Evidence:** finra.org "Update an Individual's Registration" + eligibility guidance + RN 25-13; Legal500 2025 CE amendment summary.
- **Value:** a missed 10-day window can trigger a registration defect and fines; a lapse costs $10k–100k in commissions.
- **Market:** ✅ no rep-side personal deadline agent found — Web CRD is firm-facing and drafting services are not execution agents.
- **Why local wins:** the 10-vs-30-day split is exactly the kind of rule the harness's `Statute-Recompute Kernel` / `ClockBook` derives from governing text, with a local record of which disclosures were filed when.

---

## 5. Sources

**Regulatory primary (aviation, firearms, trucking, construction):**
- eCFR 14 CFR §61.56 (Flight review), §61.57 (Recent flight experience) — ecfr.gov/current/title-14/.../section-61.56, .57 (2026 editions)
- FAA — Medical Certification; BasicMed; "FAA Updates BasicMed Program"; AOPA BasicMed FAQ; Skyfarer "FAA Opens Public Comment on BasicMed Renewal" (6/25/26)
- ATF — Federal Firearms Licenses (renewal terms/fees); FFL Quick Reference and Best Practices Guide (4473/A&D retention, inspections, revocation triggers); 91 FR 24413 "Firearm Records Retention Periods" proposed rule (5/6/26, comments 8/4/26); 27 CFR 478.129
- FMCSA — Drug & Alcohol Clearinghouse (query plan $1.25); CSA Safety Planner drug/alcohol program; "Drug & Alcohol Testing Program"; Federal Register 5/30/25 Civil Penalties Schedule Update; CNS "FMCSA Rule Changes That Could Reshape Trucking in 2026" (full enforcement Feb 2026); Fleetrabbit 2026 compliance checklists; OTRucking new-carrier checklist (UCR $69)
- OSHA — 29 CFR 1904 / 1904.33; OSHA Recordkeeping page; Miter (300 log 7-day entry; 5-yr retention), Raken e-submission guide, ABC Carolinas "OSHA Construction Compliance 2026" (300A posting Feb 1–Apr 30)
- MechanicsLienManagement "How to Send a Preliminary Notice — 2026" and "All 50 States Compared (2026)"; TerraPC "Mechanics Lien Deadlines by State (2026)"; Levelset "Lien Waivers: The 12 States With Required Forms"

**Financial / professional services:**
- IRS Topic 652 / "Understanding your CP2000 series notice"; Uncle Kam "IRS CP2000 Response 2026" (30-day → 90-day Tax Court ladder); Wolters Kluwer notice-response workflow (2025–26); FinAcct Solutions "Independent Agencies' No. 1 Tech Problem" (2026); Insurance Business/FirstConnect agency survey (2026); Wingspan insurance-distribution platform
- FINRA — "Update an Individual's Registration" (30/10-day), Eligibility Proceedings guidance, Regulatory Notice 25-13 (10/10/25), "FINRA Reduces Waiting Periods for Retaking Qualification Exams" (7/1/26); Legal500 "FINRA Amends Continuing Education And Registration Requirements" (2025)
- Amazon Seller Central — FBA reimbursement policy (10/23/24, 3/31/25); Nova Analytics "SAFE-T Claims Window Shrinks to 30 Days" (2/16/26); A-to-z appeal 30-day guidance

**Manufacturing / environment / workforce / education:**
- FDA — QMSR page + FAQs; "QMSR became effective February 2, 2026"; CP 7382.850 risk-based inspections; AAMI / NAMSA / NSF QMSR 2026 summaries (ISO 13485:2016 incorporation)
- US EPA — e-Manifest FAQ (1/22/25 registration; 12/1/25 electronic exception/discrepancy reports; 45-day inquiry, 60-day exception report); Biennial RCRA Report (3/1/26); SQG re-notification (9/1/25 cycle); RCRA criminal provisions (up to $50k/day); TriHaz 2026 e-Manifest guide
- ICE — I-9 Inspection fact sheet (revised 3/16/26); Holland & Knight (4/2026), Dentons (4/17/26), Lexology, NFIB (penalties $288–$2,861 / $716–$28,619); NATP I-9 compliance changes
- MOHELA/ED — PSLF final regs published 10/30/25, effective 7/1/26; studentaid.gov IDR account adjustment, IDR court actions, PSLF buyback; TIAA and ProtectBorrowers 2025–26 borrower summaries
- Youth sports 2026 handbooks: Flower Mound TX Youth Sports Handbook (3/5/26); Gilroy Little League, EHYLLL, WYSA (SafeSport/background/concussion cadence); Voorhees NJ Youth Sports Policy (2025); Evanston Parks & Rec coach handbook
- Hospitality: Docyt "Hotel OTA Reconciliation: Why Monthly Is Too Late"; Evention OTA reconciliation; Innrly OTA guide; Revenuenaire 2026 commission rates; FL DBPR Hotel-Motel licensing guide

**Prior-art baseline:** `PLAN.md` §25/§29; `USE-CASES-EXPANSION-V2.md` §§0–6 (grep-checked in full, including the `docs/` copies).
