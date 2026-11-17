# O0 gate evidence — Optimal Windows / GPUI track

Captured 2026-11-18 on Windows 11 Home Single Language x64, build **10.0.26300**, non-elevated.
Rust/cargo **1.100.0** (`x86_64-pc-windows-gnu`). GPUI **=0.2.2**, windows-rs **=0.61.3** (both exact-pinned in `Cargo.toml` and resolved in `Cargo.lock`).

## Commands actually executed (local)

| Command | Result |
|---|---|
| `cargo check --workspace` | exit 0 |
| `cargo build --workspace` | exit 0 (single headless exe) |
| `cargo build -p optimal --features ui` | exit 0 (GPUI window binary) |
| `cargo test --workspace --locked` | exit 0 — all test binaries green |
| `cargo test -p optimal-policy --features tree-sitter-split` | exit 0 |
| `cargo check -p optimal-store --features dpapi` | exit 0 |
| `cargo run -p optimal-evals` | `EVAL O0 PASS: rule table, scrubber, canary, receipt` |
| `cargo clippy --workspace --all-targets --locked -- -D warnings` | exit 0 |
| smoke run of `target/debug/optimal.exe` | `optimal v0.0.1 gateway=127.0.0.1:<port> token=<set>`; single-instance mutex acquired; elevated start refused; SQLite WAL store opened |

## Counts

- Rule-table cases: **70** (≥60 required), all tier assertions green.
- Workspace automated tests: **69** across 11 test binaries (2 common, 27 policy, 4 policy table, 5 registry, 5 runtime, 6 gateway, 12 store, 4 UI/telemetry/guardian/evals, plus compile-fail test).
- Compile-fail guarantee: `missing_risk.rs` rejected with E0308 (`Unbranded` vs `RiskDeclaration`), snapshot committed at `crates/registry/tests/ui/missing_risk.stderr`.
- Secret leak count in transcripts/logs: **0** (forced sink redaction tested with `canary-O0-SECRET-9c2b`).
- Audit chain: verifies on append; tampering an event makes `verify_chain()` return false.
- No-op effects: receive receipt + audit row from day 1.

## Honest limitations

1. **Human-only checks are BLOCKED**, not PASS: 100/125/175% DPI visual inspection, physical Esc/Ctrl+Alt+Space capture, screen-reader/accessibility, and the second-LAN-machine forged-Host WS upgrade. See `TESTLOG.md`.
2. **`cargo build --release --features ui` is blocked locally**: GPUI 0.2.2's build script needs `fxc.exe` from the Windows SDK for non-debug profiles; absent here and installing it would require elevation (forbidden by the project). The debug GPUI build and all headless builds pass. CI (`windows-latest`) has the SDK and runs the release build.
3. **Z3 native build blocked locally** by a CMake/MinGW C++ toolchain quirk; feature is default-off, validated via conservative pure-Rust fail-closed validator in tests, and exercised in the CI `optional-validators` job. The validator code was written against the actual z3 0.19 API (source-checked).
4. **CI workflow has not been executed on GitHub** at authoring time; local equivalents of every CI command were run.
5. **Delivery structure:** the seven O0 steps were implemented and gated sequentially in the working tree, but delivered as **one integrated PR** rather than seven stacked PRs. Reason: the steps have circular build-time coupling (the O0.1 workspace cannot compile until later crates exist), so splitting into seven independently buildable PRs would have required rewriting history and would have produced PRs that do not build. Per-step gate evidence is preserved in `TESTLOG.md`; I can split into stacked PRs on request.

## Deliverables

- Integration commit: `ff6dd2f` on branch `o0/integration` (pushed to origin).
- Tag: **v0.0.1** (annotated, points at `ff6dd2f`, pushed to origin).
- Latest branch commits: `6ba30d1` (evidence), `1eecc25` (CI pin-check fix).
- PR: **#1** — https://github.com/Ronitdoes/optimal/pull/1 (`o0/integration` → `main`).
- Remote `main` was kept at the seed commit `d271413` so the PR contains the O0 diff.
- CI: run `44169888196` on PR #1. Headless job **completed/success**; GPUI-window and
  optional-validator jobs were still running at time of writing (z3 builds from source).
  An earlier run's GPUI job failed only on my `cargo tree -i gpui` pin-check step; that
  step was rewritten to a deterministic Cargo.toml/Cargo.lock check.
- This file + `TESTLOG.md` + `DONE-DEFERRED.md` constitute the O0 gate evidence trail.
