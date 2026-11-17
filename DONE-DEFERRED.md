# Optimal — Done / Deferred (O0 track)

> **Scope:** phase O0 only (`Optimal/phases.md`). O1+ is NOT implemented.
> **Honesty:** "PASS" below means the automated check was actually executed and
> recorded. Human-only checks are **BLOCKED**, never marked PASS by an agent.
> Full per-step log: `TESTLOG.md`. Evidence: `O0-GATE-EVIDENCE.md`.

## Done (O0) — implemented and gated

| Step | Implemented | Automated gate | Human-only gate |
|---|---|---|---|
| **O0.1** | Rust workspace; GPUI 0.2.2 pinned in `Cargo.toml` + `Cargo.lock`; root view; Win32 per-monitor DPI awareness; single-instance mutex; elevation refusal | **PASS** — debug build with `--features ui`, headless build, startup smoke (loopback gateway bound, mutex acquired, not elevated) | **BLOCKED** — visual DPI at 100/125/175% and resize |
| **O0.2** | `ToolEnvelope` + sealed/branded `RiskDeclaration`; probe cache (TTL + last-good grace + dedupe); `tools.search`; uniform `ToolResult` | **PASS** — 5 unit tests + trybuild compile-fail snapshot for a tool with no branded declaration | n/a in O0 (no tool cards UI until O3) |
| **O0.3** | Native PERCEIVE→PLAN→ACT→REFLECT loop; interrupt queue; kill switch; `plan.json` + `todos.md`; bounded retries + stable idempotency keys | **PASS** — 5 tests incl. 5-step completion and mid-flight kill with completed-effect log | **BLOCKED** — physical Esc / Ctrl+Alt+Space key capture (O3.4) |
| **O0.4** | Canonicalizer (case/slash/8.3/UNC/trailing-dot/escape); four tiers default `never`; unconditional floors; fail-closed load validator; tree-sitter PowerShell split (feature) | **PASS** — 70/70 rule-table tiers + canonicalization tests; tree-sitter feature tested | n/a |
| **O0.5** | SQLite WAL events; event+projection+outbox+receipt+audit in one transaction; no-op receipts; forced sink redaction; hash chain; DPAPI vault refs | **PASS** — 12 tests incl. redaction, no-op receipt, tamper detection; DPAPI feature compiles | **BLOCKED** — live DPAPI seal/unseal on the target machine |
| **O0.6** | Loopback JSON-RPC (`harness.v1.*`), capabilities handshake, constant-time auth, streaming into GPUI `UiState`, WS Host/Origin validation | **PASS** — 6 tests incl. end-to-end stream and DNS-rebinding rejection | **BLOCKED** — forged-Host WS upgrade from a second LAN machine |
| **O0.7** | Eval harness (70-case rule table, canary, scrubber); CI workflow | **PASS** — harness exits 0 with all checks; `clippy -D warnings` clean; all workspace tests green | **BLOCKED** — GitHub Actions run not yet observed |

**O0 exit gate:** multi-step task end-to-end (PASS), zero secrets (PASS), kill switch from UI + hotkey (PARTIAL — logic PASS, UI/hotkey BLOCKED), audit chain verifies (PASS), 60+ rule tests (PASS — 70). **Not fully green until the four BLOCKED human checks are executed.** No FAIL was observed; no FAIL was flipped to PASS.

## Deferred / not in this track

From `Optimal/plan.md` §6 and `Optimal/tools.md` §3, plus cross-cutting exclusions:

- **Browser agent** — `browser.*`, CDP/Playwright/extension bridge; own injection corpus.
- **Comms** — mail/calendar; draft-only + confirm, after the desktop loop is trustworthy.
- **Cloud/DevOps CLI adapters** — `aws.*`, `gh.*`, `az.*`, `kubectl.*`.
- **Payments beyond draft** — two-key gate; no card data in context.
- **i18n / non-Gregorian time / international regimes** — global track.
- **macOS / Linux parity** — Windows-first (ADR-002).
- **VS Code extension, web dashboard, mobile** — one surface first.
- **Multi-tenancy / Social OS tools** — later products on the receipts substrate.
- **Tauri / Electron / Node / TypeScript core** — extra runtime + IPC boundary (ADR-001/003).
- **Python core** — only optional narrow ML sidecars later (Whisper/OCR via `windows-rs` first).
- **LangGraph / any graph agent framework in the core loop** — ADR-023; native Rust state machines only; graph frameworks are interop targets via MCP/A2A, never dependencies.
- **Later phases** O1–O8 (git/rewind/OS boundary, durability/compaction/OTel, UX surfaces, MCP/skills/Code Mode/TTSR, UIA automation, guardian/scheduling, signed updates/offline, distribution/licensing) — not implemented here.

## Environment limitations (observed 2026-11-18)

- **Rust 1.100.0 x86_64-pc-windows-gnu installed locally; system MinGW too old** — a portable MinGW-w64 + CMake was placed in gitignored `.tools/`. GitHub Actions uses its own Windows toolchain.
- **`cargo build --release --features ui` is BLOCKED locally** — GPUI 0.2.2 `build.rs` requires `fxc.exe` (Windows SDK) in non-debug profiles; not present and installing it would require elevation (forbidden). Debug GPUI build and headless build PASS. CI on `windows-latest` has the SDK.
- **Z3 native build is BLOCKED locally** — CMake + MinGW C++ toolchain quirk while compiling bundled Z3. Code targets z3 0.19 API; feature is default-off with a fail-closed pure-Rust validator; CI runs the Z3 job.
- **GitHub credentials are present** (git push authenticated in a dry run); PR creation via API was attempted during delivery — see `O0-GATE-EVIDENCE.md` for the outcome.
- **Human-only UI/DPI/hotkey/screen-reader/second-LAN checks** are not executable by an agent and remain BLOCKED in `TESTLOG.md`.
