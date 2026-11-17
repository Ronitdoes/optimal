# Optimal — Definition of Done
### Windows desktop / GPUI · applies to every step in `phases.md`

> A step is **done** only when every applicable box below is checked and recorded. "Works on my machine", a passing unit test alone, or an unverified performance claim does not count.

---

## 1. Universal DoD — every `Ox.y` step

### Code & build
- [ ] Compiles on stable Rust with `cargo build --release`; no new warnings under `clippy -D warnings`.
- [ ] Windows 11 x64 verified at **100%, 125%, 175% DPI** (GPUI DPI-awareness ADR-001).
- [ ] Single binary produced; no new runtime dependency without an ADR.
- [ ] Dependencies pinned in `Cargo.lock`; SPDX license checked in CI (BEST §4 license surprises).

### Tests
- [ ] Unit tests for the new logic; rule-engine changes add **table tests** to `evals/`.
- [ ] Property test for any new effect path: "no effect without receipt", idempotency, fail-closed.
- [ ] Existing eval corpus (rule tables, injection corpus, canary files, chaos replay) still green.
- [ ] `tests.md` manual checklist for **this exact step ID** executed and result recorded (pass/fail + timestamp + Windows build).

### Safety invariants (non-negotiable)
- [ ] No secret, token, CVV, or canary value appears in any transcript, log, or telemetry span.
- [ ] No new path from untrusted content (file/page/RAG/MCP) to a mutating tool without a receipt.
- [ ] Default for the new capability is `never` or `confirm`; no `yolo`, no fail-open, no full-access default.
- [ ] Every confirm card shows exact scope, matched rule + tier, and what will be reverted.
- [ ] Policy rechecked after awaits under the commit lock (T3 TOCTOU pattern).
- [ ] Egress deny-by-default preserved; metadata endpoint still hard-denied.
- [ ] Kill switch and undo ledger still work after the change.
- [ ] Unattended contexts resolve to **deny**, never auto-approve; no global "approvals off" or session `yolo` path exists (ADR-019/020 refusals).
- [ ] No model verdict can grant an approval; the guardian is advisory, and a denial result tells the model not to retry or route around it.
- [ ] Unconditional floors are evaluated before every bypass, including deletion of the runtime's own binary/venv.
- [ ] Local WS upgrade validates Host/Origin and fails closed before any PTY/event surface (CVE-2026-53869 class).
- [ ] Autonomous memory/skill writes are staged and review-required by default; per-skill locks and `--force` cannot override a dangerous verdict.
- [ ] Egress and inference-routing are one policy object; half-configuration is a hard refusal, and proxy-control variables are protected.

### Observability & state
- [ ] New effect emits an OTel span with stable internal names; `correlationId` propagates end-to-end.
- [ ] New effect writes an event + projection + outbox row in **one transaction** and is receipt-guarded.
- [ ] Audit hash chain still verifies.

### Docs & honesty
- [ ] `plan.md` / `decisions.md` / `phases.md` updated if behavior or an ADR changed.
- [ ] No unverified savings/accuracy number in code comments or docs; claims carry benchmark version + Pass^k/CI where relevant.
- [ ] Manual test log entry written; any known limitation stated explicitly (no shallow coverage presented as deep).

---

## 2. Definition of *ready* (before a step starts)
- Prior phase gate passed.
- Step's manual tests exist in `tests.md` (no test = cannot be done).
- Any new dependency has an ADR or an explicit note in `decisions.md`.
- Eval fixtures for the new capability are staged in `evals/`.

---

## 3. Phase exit gates (condensed; authoritative list in `phases.md`)

| Phase | Exit condition |
|---|---|
| **O0** | Multi-step task end-to-end; zero secrets; kill switch from UI + hotkey; audit chain verifies; 60+ rule tests green |
| **O1** | 100+ rule tests; rollback drill; three-axis rewind; OS boundary fail-closed; adaptive injection corpus v0 green |
| **O2** | Kill mid-command → restart → no duplicate effect; compaction lossless; telemetry correlates; consent expiry verified |
| **O3** | All six GPUI surfaces render from one event stream; reviewable confirms; own-UI keyboard + screen-reader audit green |
| **O4** | Third-party MCP works; hostile plugin suite green; Code Mode budgeted; OSV fail-closed; 40-case injection corpus green |
| **O5** | 5/10 desktop tasks pass at default approvals; per-app rules enforced; post-action verification + abort; single desktop lock |
| **O6** | 2-week unattended simulation, zero destructive actions; guardian advisory + two-key; jobs survive restart |
| **O7** | Signed update + rollback + key rotation + restore + offline + a11y all green; latency budgets met |
| **O8** | winget/scoop clean-machine install; license/governance published; audit-chain export reconstructs a task |

---

## 4. Release gate (every tagged build)

- [ ] All applicable phase gates green.
- [ ] `cargo build --release` from a clean clone; artifact signed; SLSA provenance + SBOM emitted.
- [ ] Update applied through the blue-green path on a clean machine; rollback drill passed.
- [ ] Full `tests.md` manual suite for the touched steps re-run.
- [ ] Telemetry opt-out verified **on the wire** (no outbound span with content).
- [ ] Changelog names model/provider changes and any deprecation; no silent swap.
- [ ] Known limitations section written (deferred scopes: browser, comms, mobile, i18n, etc.).

---

## 5. Quality bar (measured, not asserted)

| Metric | Threshold |
|---|---|
| Rule-engine table tests | ≥100, all green |
| Injection corpus (static + adaptive/chained) | 40+ cases, zero successful attacks |
| Pass^k on the local task suite | ≥0.9 with confidence intervals |
| Rubber-stamp approvals (<1s) | measured, alert threshold configured |
| Permission card decision latency | ≤200 ms p95 |
| GUI act to early decode | ≤500 ms p95 |
| Secrets in transcripts/logs | **0** |
| Unreviewed irreversible actions | **0** |
| Zero-egress profile outbound calls | **0** |

---

## 6. Explicitly *not* done

- A feature merged without its eval corpus and strictest-sane confirm defaults.
- A confirm that cannot be independently judged from its card (fails `reviewability`).
- A "fix" that changes a tier to `auto` to reduce prompts without a measured fatigue signal and a human decision.
- A checkpoint/rewind that can change files without being able to roll back the conversation.
- A copied mechanism whose unsafe vendor default (yolo, fail-open, full-access) was kept.
- A number quoted from a competitor's marketing page without reproduction.

---

## 7. Evidence trail

Every DoD check that is automated lives in CI; every manual check lives in the `tests.md` log. A release is auditable by reading those two artifacts alone — which is the whole point of the project.
