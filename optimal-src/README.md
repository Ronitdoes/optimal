# Optimal — O0 skeleton (`optimal-src/`)

> **This is phase O0 only; O1+ is NOT implemented.**
>
> Only the skeleton and safety spine (steps O0.1–O0.7) are in scope here. No capability
> phase is built in this track: no git/rewind (O1), no Windows GUI automation (O5),
> no MCP/skills (O4), no signed updates (O7), no productization (O8). See
> `../DONE-DEFERRED.md` for the explicit deferrals.

## Workspace layout

```
optimal-src/
├─ Cargo.toml (workspace, pinned deps; Cargo.lock committed)
├─ crates/
│  ├─ core/        # agent loop, planner, state, effect dispatch
│  ├─ policy/      # canonicalize, tiers, Z3 validation, tree-sitter split
│  ├─ registry/    # tool registry, branded risk declarations, probes
│  ├─ tools/       # tool implementations (envelope + result)
│  ├─ runtime/     # host runtime, interrupt/kill switch
│  ├─ store/       # SQLite events + outbox + receipts + hash chain
│  ├─ ui/          # GPUI window and root view
│  ├─ gateway/     # local JSON-RPC/WS, capabilities handshake, harness.v1.*
│  ├─ guardian/    # advisory reviewer client (advisory only)
│  └─ telemetry/   # OTel spans, pinned internal attribute adapter, content OFF
├─ runner/          # Windows launcher / service entry point (single exe)
└─ evals/           # rule tables, canary files, injection corpus, scrubber, manual scripts
```

This is the O0 crate set. The full planned set in `../Optimal/plan.md` §3 also includes
`crates/model/` (BYOK provider adapters), which arrives in a later phase and is not
part of O0.

## Prerequisites

- **Windows 11 x64** (track targets Windows 10/11 x64; Windows 11 is the tested target, ADR-002).
- **Rust stable** (latest) via `rustup`, with a committed `Cargo.lock` pinning `gpui`,
  `gpui_platform`, `windows-rs`, `tokio`, `rusqlite`, `z3`, etc. (ADR-001).
- **MSVC C++ Build Tools** ("Desktop development with C++" in Visual Studio Build
  Tools) **and CMake** — the `z3` crate builds its C++ dependency from source, and
  several Windows/SQLite deps need a native toolchain.
- **PowerShell 7** on PATH for shell tooling.
- **Never elevated.** Build, run, and test as a normal (non-administrative) user. The
  core daemon refuses to start elevated (ADR-004); per-verb elevation is a later,
  gated capability, never a default.

## Build & test

```powershell
# From D:\projects\ai-harness\optimal-src
cargo build --release                         # one .exe (O0.1)
cargo test --workspace                        # unit tests + rule-table tests in evals/
cargo clippy --workspace --all-targets -- -D warnings
```

- DPI awareness and a clean window at 100% / 125% / 175% are **manual** checks
  (`../Optimal/tests.md` O0.1); a passing build/test run alone does not satisfy DoD.
- Rule-engine table tests, the injection corpus, and the transcript scrubber live in
  `evals/` and are wired into CI at O0.7.
- Manual checks are keyed to step IDs in `../Optimal/tests.md` §1. Record every result
  in `../TESTLOG.md` using the exact §2 template. Per ADR-018, a step is **not done**
  until `dod.md` is satisfied and its manual test is executed and recorded.

## Safety spine (the part that is not optional)

- **Deny-by-default.** Tiers are `readonly | auto | confirm | never`; the default for
  anything unknown or undeclared is `never`. Every effect follows
  `tool call → canonicalize → policy → OS boundary → execute → hash-chained audit`.
  `policy.toml` is Z3-validated at load; commands are split with tree-sitter;
  the most-restrictive rule wins; authorization is rechecked after awaits under the
  same lock that commits the effect (TOCTOU). Unattended `confirm` resolves to
  skip + notify. There is no `yolo`, no fail-open hook, and no full-access default.
- **Branded risk declarations.** A tool cannot be registered without a
  `RiskDeclaration` (a private-constructor compile-time brand); a missing declaration
  is a compile error. Availability is probe-gated (`probe.cmd`), and every tool uses
  one uniform `ToolEnvelope` / `Result` with a `correlationId`.
- **Receipts & redaction.** Secrets are redacted at the persistence sink, the audit
  log is hash-chained, and the kill switch (Esc / Ctrl+Alt+Space / tray) works
  mid-task (O0.5, O0.3).

## Governing documents

All behavior is governed by `../Optimal/` (authoritative over this README):

- `plan.md` — vision, stack, success criteria (§7), explicit deferrals (§6)
- `phases.md` — discrete O0 steps and their done conditions (O0 table)
- `dod.md` — Definition of Done, phase gates, quality bar
- `tests.md` — manual test matrix (§1), log template (§2), stop conditions (§3)
- `decisions.md` — ADRs, especially ADR-018 (docs are release gates)
- `tools.md` — tool inventory and the tools explicitly not built here (§3)

## Honesty note

At the time of writing this skeleton, the automated build/tests had **not** been run in
the authoring environment (no MSVC/CMake toolchain, no GitHub token configured), and
the visual/DPI/hotkey/second-machine checks cannot be executed by an agent at all.
Every O0 step is therefore **pending human verification** — see `../TESTLOG.md` and
`../DONE-DEFERRED.md`. Nothing here should be read as a claim that tests passed.
