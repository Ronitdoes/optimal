# Optimal — O0 test log

> Rules from `Optimal/tests.md` §2/§3. **Automated checks were executed by the build agent and are recorded with real results. Human-only checks (visual DPI inspection, physical key presses, screen readers, second-LAN-machine WS rebinding) are recorded as BLOCKED and were NOT marked PASS.** No FAIL was observed; no FAIL was later flipped to PASS.
>
> Environment (observed 2026-11-18): Windows 11 Home Single Language x64, build **10.0.26300**, **not elevated**, rustc/cargo **1.100.0 (x86_64-pc-windows-gnu)**, GPUI **0.2.2** pinned. Local build needs a portable MinGW-w64 + CMake toolchain under `.tools/` (gitignored); release GPUI build additionally needs `fxc.exe` from the Windows SDK (see evidence doc).

---

```
STEP:            O0.1
DATE / WIN BUILD: 2026-11-18 / 10.0.26300
RESULT:          PASS (automated) + BLOCKED (visual DPI)
COMMANDS / UI:   cargo build -p optimal --features ui (debug) -> exit 0;
                 cargo build -p optimal (headless) -> exit 0;
                 short smoke run of target/debug/optimal.exe with OPTIMAL_RPC_TOKEN set
EXPECTED:        clean clone builds; window opens on Win11 at 100/125/175% DPI; one exe
OBSERVED:        workspace + GPUI window compile; startup smoke shows
                 "optimal v0.0.1 gateway=127.0.0.1:<port>" (loopback bound, single-instance
                 mutex acquired, elevated start refused, event store opened).
EVIDENCE:        optimal-src/target/debug/optimal.exe ; shell transcript in build log
FAIL SIGNAL:     none (automated). Visual DPI crispness/offset at 100/125/175% not
                 observable by an agent -> BLOCKED (needs human at the desktop).
ROLLBACK:        none
OWNER:           build agent (automated) / human (visual)
NOTES:           `cargo build --release ... --features ui` is BLOCKED locally: GPUI 0.2.2
                 build.rs requires `fxc.exe` (Windows SDK) in non-debug profiles. CI on
                 windows-latest has the SDK. GPUI version pinned in Cargo.toml + Cargo.lock.
```

```
STEP:            O0.2
DATE / WIN BUILD: 2026-11-18 / 10.0.26300
RESULT:          PASS
COMMANDS / UI:   cargo test -p optimal-registry (5 unit tests + trybuild compile-fail)
EXPECTED:        tool without branded risk declaration fails to compile; unprobed tool unreachable
OBSERVED:        trybuild test `unbranded_tool_does_not_compile` passes with committed
                 .stderr snapshot (E0308 mismatched types: Unbranded vs RiskDeclaration).
                 Unknown-tool and unprobed-tool both return Denied; probe dedupe/last-good
                 grace tested. tools.search returns capability metadata.
EVIDENCE:        optimal-src/crates/registry/tests/ui/missing_risk.rs + .stderr
FAIL SIGNAL:     none
ROLLBACK:        none
OWNER:           build agent
NOTES:           Visual "greyed out in UI" not applicable in O0 (no tool cards yet, O3).
```

```
STEP:            O0.3
DATE / WIN BUILD: 2026-11-18 / 10.0.26300
RESULT:          PASS (logic) + BLOCKED (physical key capture)
COMMANDS / UI:   cargo test -p optimal-runtime (5 tests)
EXPECTED:        5-step read-only task completes; kill stops in-flight; completed effects logged
OBSERVED:        five_step_task_completes_and_persists_plan OK (plan.json + todos.md);
                 kill_stops_inflight_and_logs_completed_effects OK (3 effects then Skipped);
                 Esc soft-interrupt injected between tools; Ctrl+Alt+Space kill stops before
                 first tool; idempotency keys stable across retries.
EVIDENCE:        crates/runtime/src/lib.rs tests
FAIL SIGNAL:     none. Physical Esc / Ctrl+Alt+Space key capture is the GPUI/Win32 layer
                 (O3.4) and cannot be pressed by an agent -> BLOCKED.
ROLLBACK:        none
OWNER:           build agent
NOTES:           PERCEIVE->PLAN->ACT->REFLECT native state machine (ADR-023, no graph framework).
```

```
STEP:            O0.4
DATE / WIN BUILD: 2026-11-18 / 10.0.26300
RESULT:          PASS
COMMANDS / UI:   cargo test -p optimal-policy ; cargo test -p optimal-policy --features tree-sitter-split
EXPECTED:        60+ rule table tests green incl. npm hijack, IWR|iex, compound commands
OBSERVED:        70/70 CSV tier assertions pass; canonicalization regression tests pass;
                 unconditional floors (metadata, path escape, download-exec, encoded shell,
                 elevation, injection, self-delete, destructive root) verified; default tier
                 is `never`; conflict/overbroad/floor-lowering configs refused at load.
                 Tree-sitter PowerShell split passes all tests with the feature enabled.
EVIDENCE:        evals/rules/o0_rule_table.csv (70 rows), crates/policy/tests/table.rs
FAIL SIGNAL:     none
ROLLBACK:        none
OWNER:           build agent
NOTES:           Z3 feature (`z3-validate`) code updated to z3 0.19 API; local native build
                 BLOCKED by CMake/MinGW fxc/libclang toolchain quirk. Feature is default-off
                 with a conservative pure-Rust fail-closed validator; CI runs the Z3 job.
```

```
STEP:            O0.5
DATE / WIN BUILD: 2026-11-18 / 10.0.26300
RESULT:          PASS
COMMANDS / UI:   cargo test -p optimal-store ; cargo check -p optimal-store --features dpapi
EXPECTED:        fake secret never in events.jsonl; tampered audit row fails chain verification
OBSERVED:        12 tests pass. SQLite WAL enabled; event+projection+outbox+receipt+audit in
                 one transaction; no-op still gets receipt + audit row; forced sink redaction
                 keeps vault refs but removes `canary-O0-SECRET-9c2b` from JSONL; tampering an
                 event breaks `verify_chain()`. DPAPI vault-ref module compiles with feature on.
EVIDENCE:        crates/store/src/lib.rs, redaction.rs, vault.rs tests
FAIL SIGNAL:     none
ROLLBACK:        none
OWNER:           build agent
NOTES:           DPAPI seal/unseal was compile-checked, not executed against the live OS store.
```

```
STEP:            O0.6
DATE / WIN BUILD: 2026-11-18 / 10.0.26300
RESULT:          PASS (protocol) + BLOCKED (second-LAN-machine rebinding test)
COMMANDS / UI:   cargo test -p optimal-gateway (6 tests)
EXPECTED:        "hello" task streams end-to-end from core to window; WS Host/Origin fails closed
OBSERVED:        loopback bind, capabilities handshake, constant-time auth before event surface,
                 `harness.v1.*` namespace enforcement, and token/tool-badge/done streaming all
                 pass in an end-to-end socket test. Host/Origin validator rejects foreign origin,
                 non-loopback host, and missing host; RFC 6455 accept key verified.
EVIDENCE:        crates/gateway/src/lib.rs, ws.rs tests
FAIL SIGNAL:     none. Forged-Host upgrade from a second LAN machine cannot be sent by an
                 agent -> BLOCKED (human/network test).
ROLLBACK:        none
OWNER:           build agent
NOTES:           GPUI view consumes the stream via `UiState::apply` (feature-independent test).
```

```
STEP:            O0.7
DATE / WIN BUILD: 2026-11-18 / 10.0.26300
RESULT:          PASS
COMMANDS / UI:   cargo run -p optimal-evals ; cargo clippy --workspace --all-targets -- -D warnings
EXPECTED:        CI fails on leaked secret / canonicalization regression / canary touched
OBSERVED:        eval harness: "checked 70 cases", "scrubber: redacted", "canary: intact",
                 "EVAL O0 PASS"; clippy with -D warnings exits 0; full workspace test suite green.
EVIDENCE:        .github/workflows/ci.yml, crates/evals (optimal-evals), clippy log
FAIL SIGNAL:     none
ROLLBACK:        none
OWNER:           build agent
NOTES:           CI itself has not been executed on GitHub Actions yet (no push/PR run observed
                 at time of writing); local equivalent commands were executed.
```

---

## O0 gate checklist (`Optimal/dod.md` §3)

| Condition | Status | Evidence |
|---|---|---|
| Multi-step task end-to-end | PASS (runtime + gateway stream tests) | O0.3/O0.6 |
| Zero secrets in transcripts | PASS | O0.5, O0.7 |
| Kill switch from UI and hotkey | PARTIAL — logic PASS, physical key + UI BLOCKED | O0.3 |
| Audit chain verifies | PASS | O0.5 |
| 60+ rule tests green | PASS (70) | O0.4 |

**Gate verdict:** all automatable O0 checks pass; four human-only checks remain BLOCKED and must be executed by the user before the O0 gate can be called fully green. No FAIL was observed.
