# Optimal — Dependency Pins & Build Feasibility Brief

**Status:** engineer's brief for writing `optimal-src/Cargo.toml` · **Data checked:** 2026-11-18
**Method:** crates.io API v1 (`curl -s -H "User-Agent: optimal-build/0.1" https://crates.io/api/v1/crates/<name>`), docs.rs for `gpui 0.2.2`, and `microsoft.github.io/windows-docs-rs` for `windows-rs`.
**No `cargo build` and no git commands were run for this brief.** Every claim below is tagged as *CONFIRMED*, *LIKELY*, or *VERIFY AT COMPILE TIME*. Where documentation is sparse or points at a different patch version than the one we pin, it is called out explicitly — nothing here is presented as more certain than it is.

---

## 1. Exact pins for `Cargo.toml`

### 1.1 Copy-paste fragment

```toml
[workspace.dependencies]
# ADR-001 / ADR-003: pre-1.0 and ADR-mandated — exact (=) pins, commit-pinned in CI.
gpui                  = { version = "=0.2.2" }
windows               = { version = "=0.61.3", features = ["Win32_UI_WindowsAndMessaging", "Win32_System_Threading", "Win32_Security", "Win32_UI_Shell", "Win32_Foundation"] }

# Async / storage / policy
tokio                 = { version = "=1.54.1", features = ["rt-multi-thread", "macros", "sync", "time", "net", "fs", "io-util", "signal"] }
rusqlite              = { version = "=0.43.0", features = ["bundled"] }
z3                    = { version = "=0.23.0", optional = true }
tree-sitter           = { version = "=0.26.13" }   # NOT 0.27.x — see §1.3
tree-sitter-powershell= { version = "=0.26.6" }

# Data / errors / observability / testing
serde                 = { version = "=1.0.229", features = ["derive"] }
serde_json            = { version = "=1.0.154" }
trybuild              = { version = "=1.0.125" }   # [dev-dependencies]
uuid                  = { version = "=1.29.3", features = ["v4", "serde"] }
sha2                  = { version = "=0.10.9" }    # conservative, see §1.4
hex                   = { version = "=0.4.3" }
tracing               = { version = "=0.1.44" }
tracing-subscriber    = { version = "=0.3.23", features = ["env-filter", "fmt", "json"] }
thiserror             = { version = "=2.0.21" }
chrono                = { version = "=0.4.45", features = ["serde"] }
# time = { version = "=0.3.55" }   # add ONLY if a dep needs it; prefer chrono alone
```

> Exact (`=`) pins are used throughout for reproducibility. For the mature 1.x/0.4.x crates a caret requirement would also be defensible, but this project commits to a lockfile + exact pins (ADR-001), so we are consistent.

### 1.2 Per-crate rationale and verification date

| Crate | Pin | crates.io check (2026-11-18) | Why |
|---|---|---|---|
| `gpui` | **=0.2.2** | max_version `0.2.2`, created **2025-10-22**, last updated **2026-10-30** (docs.rs build dated 16 Oct 2026) | **ADR-001** mandates canonical Zed gpui 0.2.2, pin in lockfile **and** to a commit in CI. Pre-1.0 API churn is real; no bumps without a UI regression run. Apache-2.0. |
| `gpui_platform` | **do NOT add** | **404 Not Found** on crates.io. `gpui 0.2.2` does **not** declare it as a dependency (verified via `/api/v1/crates/gpui/0.2.2/dependencies`). | **CONFIRMED:** there is no published `gpui_platform` crate. ADR-001's phrase "gpui + gpui_platform" refers to Zed's *repo layout* / internal platform layer, not two crates.io packages. Use `gpui = "=0.2.2"` only. If a distinct platform crate is genuinely needed, it must be a git/path dependency into Zed's repo — that is a **new ADR**, not a Cargo.toml line. **Do not invent `gpui_platform = "0.2.2"`.** |
| `windows` | **=0.61.3** | latest overall `0.62.2` (2025-10-06). Non-yanked 0.61.x: `0.61.3` (2025-06-12); `0.61.1` (2025-03-18). `0.61.0` and `0.61.2` are **yanked**. | **ADR-003** pins windows-rs 0.61. Use the newest non-yanked patch `0.61.3`. Bonus: **gpui 0.2.2 itself depends on `windows ^0.61` and `windows-core ^0.61`** (CONFIRMED via docs.rs dependency list), so 0.61.3 avoids duplicate major versions. Do not "upgrade" to 0.62.2 without an ADR. |
| `tokio` | **=1.54.1** | max `1.54.1`, 2026-10-21 | Mature 1.x; explicit features, no `full` (smaller binary, clearer capability surface). |
| `rusqlite` | **=0.43.0** | max `0.43.0`, 2026-11-15 | ADR-005 event store. `bundled` compiles SQLite from the amalgamation — required because the MinGW target has no system `libsqlite3`. C compiler needed: see §4. |
| `z3` | **=0.23.0** | max `0.23.0`, 2026-11-11 | ADR-006 policy validation at config load. **MUST be optional** — see §4 blocker. |
| `tree-sitter` | **=0.26.13** | max `0.27.2` (2026-11-07); newest 0.26.x `0.26.13` (2026-08-23) | Deliberately **not** latest: `tree-sitter-powershell 0.26.6` declares `tree-sitter ^0.26.5` as its dev-dep and depends on `tree-sitter-language ^0.1`; its generated parser targets the 0.26 ABI. Pinning 0.27.2 risks an ABI/language-version mismatch. **VERIFY AT COMPILE TIME** (parse a sample command in a unit test). |
| `tree-sitter-powershell` | **=0.26.6** | max `0.26.6`, 2026-10-28 (older: `0.26.4`, `0.25.10`, …) | **It IS available** — the fallback clause in the task is not triggered. It is a community-maintained grammar (maintenance risk); see §1.3 fallback plan. |
| `serde` | **=1.0.229** | max `1.0.229`, 2026-07-18 | gpui 0.2.2 requires `serde ^1.0.221`; this satisfies it. |
| `serde_json` | **=1.0.154** | max `1.0.154`, 2026-10-11 | gpui 0.2.2 requires `^1.0.144`; this satisfies it. |
| `trybuild` | **=1.0.125** | max `1.0.125`, 2026-11-16 | `[dev-dependencies]` only — compile-fail tests for branded risk declarations (ADR/tools §4). Snapshot stderr is compiler-specific: see §4. |
| `uuid` | **=1.29.3** | max `1.29.3`, 2026-11-17 | Event/correlation IDs. Compatible with gpui's `uuid ^1.1.2`. |
| `sha2` | **=0.10.9** | latest `0.11.1` (2026-10-12); newest `0.10.x` = **`0.10.9`** (2025-04-30) | **Conservative pick:** 0.10.x is the battle-tested API used across the ecosystem and in most audit/hash-chain examples (ADR-005). 0.11.1 is newer but is a fresh major with churn risk — no security requirement here demands it. If the team prefers latest, change to `=0.11.1` in one place and note the divergence. |
| `hex` | **=0.4.3** | max `0.4.3`, **2021-03-03** | Frozen/stable; perfect for hash-chain byte encoding. |
| `tracing` | **=0.1.44** | max `0.1.44`, 2025-12-18 | Long-stable 0.1 API. |
| `tracing-subscriber` | **=0.3.23** | max `0.3.23`, 2026-03-13 | `env-filter` + `json` for local/OTel pipeline (ADR-010). |
| `thiserror` | **=2.0.21** | max `2.0.21`, 2026-09-23 | Matches gpui 0.2.2's `thiserror ^2.0.12`. |
| `chrono` | **=0.4.45** | max `0.4.45`, 2026-06-04 | **Recommended single time crate** for event timestamps. |
| `time` | **=0.3.55** | max `0.3.55`, 2026-08-01 | Optional. Historical `chrono 0.4.40` ↔ `time 0.3.34/0.3.35` type-inference break is avoided at these versions, but using both directly is still unnecessary complexity. Prefer `chrono` only unless a dependency forces `time`. |

### 1.3 tree-sitter fallback (if the grammar fails on this toolchain)

`tree-sitter-powershell 0.26.6` uses a `cc` build script (dep: `cc ^1.2`) to compile generated C. If MinGW gcc 6.3 rejects the generated C (C11 features) or the grammar proves unusable:

1. **Safe fallback:** implement a small hand-written PowerShell tokenizer that splits on the *conservative* operator set (`|`, `;`, `&&`, `||`, newlines, with quote/escaping tracking), and **fail closed** — if it cannot prove the split, return a `Deny`/`never` decision, never an auto-approval. This is exactly the ADR-006/ADR-007 posture and is testable without any C compiler.
2. Keep the `tree-sitter` split behind a `tree-sitter-split` cargo feature; the fallback validator is the default in tests.
3. Do **not** substitute `tree-sitter-bash`: it does not parse PowerShell and would silently mis-split.

### 1.4 Why `sha2 0.10.9` instead of latest `0.11.1`

Both are correct; 0.10.9 is chosen because the audit hash chain (ADR-005) is long-lived code, 0.10.x has years of field use, and no requirement depends on 0.11 features. This is the one intentional deviation from "latest" among the non-ADR crates. Recorded here so the next engineer doesn't "helpfully" bump it.

---

## 2. windows-rs 0.61 API calls (exact modules + snippets)

**Documentation caveat (important):** the `windows` facade crate publishes **no versioned docs on docs.rs** (the crate page is a stub pointing to `microsoft.github.io/windows-docs-rs`, which currently documents **0.62.2**). The signatures below were checked against those 0.62.2 pages and reconciled with the known 0.61 API. **0.61 vs 0.62 difference that matters:** in 0.61 `BOOL` is a newtype struct (`windows::core::BOOL(pub i32)` with `.as_bool()` and `TRUE`/`FALSE` consts, also re-exported as `windows::Win32::Foundation::BOOL`); 0.62 generated signatures show parameters such as `binitialowner: bool`. **The snippets use the 0.61 `BOOL` form. VERIFY AT COMPILE TIME** — if your 0.61.3 build wants `bool`, replace `BOOL(1)` with `true` and `member.as_bool()` with `member`.

### 2.1 Per-monitor DPI awareness at startup

Call **once, before any HWND is created** (i.e. before `Application::new()` / before `app.run(...)`), from `main`.

```rust
use windows::Win32::UI::WindowsAndMessaging::{
    SetProcessDpiAwarenessContext,
    DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2,
};

/// CONFIRMED-module (Win32::UI::WindowsAndMessaging) in windows-rs 0.61.
/// The constant is type `DPI_AWARENESS_CONTEXT` (value -4, PER_MONITOR_AWARE_V2).
/// VERIFY AT COMPILE TIME: exact constant name/type name in 0.61.3.
pub fn set_per_monitor_dpi_awareness() {
    // SAFETY: process-wide setting; must precede window creation.
    let result = unsafe {
        SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2)
    };
    // Failure usually means a compatibility layer already fixed the context.
    // Never abort startup over this — just log.
    if let Err(e) = result {
        tracing::warn!(?e, "SetProcessDpiAwarenessContext failed; continuing");
    }
}
```

*Fallback if the context API is unavailable:* `windows::Win32::UI::WindowsAndMessaging::SetProcessDPIAware()` (system-DPI-aware only, **not** per-monitor) — acceptable only with a logged warning, since tools §O5 requires per-monitor-aware input coordinates.

### 2.2 Single-instance lock

```rust
use windows::core::w;
use windows::Win32::Foundation::{
    GetLastError, HANDLE, ERROR_ALREADY_EXISTS, SetLastError,
};
use windows::Win32::System::Threading::CreateMutexW;

/// Keep this value alive for the whole process; dropping the HANDLE releases
/// the kernel object reference (the mutex name is session-local via "Local\\").
pub struct SingleInstance(pub HANDLE);

pub fn acquire_single_instance() -> windows::core::Result<Option<SingleInstance>> {
    // IMPORTANT: clear last-error first; other Win32 calls may have set it.
    unsafe { SetLastError(0) };

    // NOTE (0.61): bInitialOwner is BOOL -> use BOOL(1) / TRUE.
    // 0.62 docs show `binitialowner: bool`; if compiling against 0.61.3
    // complains, use `true` instead. VERIFY AT COMPILE TIME.
    #[cfg(not(test))]
    let name = w!("Local\\Optimal-Single-InstanceMutex-7c2f9a1e");
    #[cfg(test)]
    let name = w!("Local\\Optimal-Single-InstanceMutex-TEST-7c2f9a1e");

    let handle = unsafe {
        CreateMutexW(
            None,                 // lpMutexAttributes: default security
            windows::core::BOOL(1), // bInitialOwner: TRUE — we own it immediately
            name,                 // PCWSTR via windows::core::w! macro
        )
    }?;

    // Must be called IMMEDIATELY after CreateMutexW, before any other Win32 call.
    if unsafe { GetLastError() } == ERROR_ALREADY_EXISTS {
        return Ok(None); // a previous instance already owns the mutex: exit
    }
    Ok(Some(SingleInstance(handle)))
}
```

*Shape verified against 0.62.2 docs:* `CreateMutexW(lpmutexattributes: Option<*const SECURITY_ATTRIBUTES>, binitialowner: bool, lpname: impl Param<PCWSTR>) -> Result<HANDLE>`; `GetLastError() -> WIN32_ERROR`; `ERROR_ALREADY_EXISTS` is a `WIN32_ERROR` constant in `Win32::Foundation`. The `w!` macro and `PCWSTR` come from `windows::core`.

### 2.3 Elevation detection

Two options. `IsUserAnAdmin()` is the one-liner; the SID/`CheckTokenMembership` path is the explicit, non-deprecated one — use it in production (ADR-004: daemon must refuse to run elevated).

```rust
use windows::core::BOOL;
use windows::Win32::Foundation::HANDLE;
use windows::Win32::Security::{
    AllocateAndInitializeSid, CheckTokenMembership, FreeSid,
    DOMAIN_ALIAS_RID_ADMINS, SECURITY_BUILTIN_DOMAIN_RID, SECURITY_NT_AUTHORITY,
    PSID,
};

/// Explicit method: test membership in BUILTIN\Administrators on the
/// thread's effective token (NULL handle => effective token).
/// VERIFY AT COMPILE TIME: module path for PSID/FreeSid and the RID constants
/// (expected in Win32::Security for windows-rs 0.61).
pub fn is_elevated() -> windows::core::Result<bool> {
    let mut sid: PSID = std::ptr::null_mut();
    unsafe {
        AllocateAndInitializeSid(
            &SECURITY_NT_AUTHORITY,
            2,                          // two sub-authorities
            SECURITY_BUILTIN_DOMAIN_RID, // S-1-5-32
            DOMAIN_ALIAS_RID_ADMINS,     // -544
            0, 0, 0, 0, 0, 0,
            &mut sid,
        )?;

        let mut member = BOOL(0); // 0.61 BOOL newtype; see caveat in §2
        let check = CheckTokenMembership(
            HANDLE(0), // NULL => current thread's effective token
            sid,
            &mut member,
        );
        FreeSid(sid);
        check?;
        Ok(member.as_bool())
    }
}
```

Quick check (VERIFY AT COMPILE TIME: module path `windows::Win32::UI::Shell` — confirmed in the 0.62.2 docs; returns `BOOL`):

```rust
use windows::Win32::UI::Shell::IsUserAnAdmin;
pub fn quick_is_admin() -> bool { unsafe { IsUserAnAdmin() }.as_bool() }
```

---

## 3. gpui 0.2.2 API shape (verified against docs.rs, with uncertainty marked)

**CONFIRMED from docs.rs/gpui/0.2.2** (not guesswork):

- `Application::new() -> Self`; also `Application::headless() -> Self` (prevents opening windows — useful for tests); `Application::with_assets`, `with_http_client`.
- `Application::run<F>(self, on_finish_launching: F) where F: 'static + FnOnce(&mut App)`.
- **There is no `AppContext::run()` in 0.2.2.** `AppContext` is now a **trait** implemented by `App`, `Context<T>` and `AsyncApp`; it provides `AppContext::new<T>(&mut self, build_entity: FnOnce(&mut Context<T>) -> T) -> Entity<T>`. The old "pass `&mut AppContext` around" mental model is obsolete.
- `App::open_window<V: 'static + Render>(&mut self, options: WindowOptions, build_root_view: impl FnOnce(&mut Window, &mut App) -> Entity<V>) -> Result<WindowHandle<V>>`.
- `Render` trait: `fn render(&mut self, window: &mut Window, cx: &mut Context<'_, Self>) -> impl IntoElement` (not `dyn`-compatible).
- `WindowOptions` is a public struct with `Default`; fields (all 14 CONFIRMED): `window_bounds: Option<WindowBounds>`, `titlebar: Option<TitlebarOptions>`, `focus: bool`, `show: bool`, `kind: WindowKind`, `is_movable: bool`, `is_resizable: bool`, `is_minimizable: bool`, `display_id: Option<DisplayId>`, `window_background: WindowBackgroundAppearance`, `app_id: Option<String>`, `window_min_size: Option<Size<Pixels>>`, `window_decorations: Option<WindowDecorations>` (Wayland only), `tabbing_identifier: Option<String>` (macOS only).

**LIKELY minimal `main.rs`** (assembled from the confirmed signatures; compile at P0 spike before building on it):

```rust
use gpui::{App, AppContext, Application, Context, IntoElement, Render, Window, WindowOptions, div};

struct Root;

impl Render for Root {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<'_, Self>) -> impl IntoElement {
        div().child("Optimal")
    }
}

fn main() {
    let app = Application::new();
    app.run(|cx: &mut App| {
        let options = WindowOptions {
            // VERIFY AT COMPILE TIME: TitlebarOptions fields were not inspected;
            // likely `title: Option<SharedString>`. Safest first spike:
            // `WindowOptions::default()` and add the titlebar once it compiles.
            // titlebar: Some(gpui::TitlebarOptions {
            //     title: Some("Optimal".into()), ..Default::default()
            // }),
            app_id: Some("com.optimal.desktop".into()),
            ..Default::default()
        };
        cx.open_window(options, |_window, cx| {
            cx.new(|_ctx| Root) // `App` implements the `AppContext` trait
        })
        .expect("open main window");
    });
}
```

**Uncertainty register for gpui:**
- CONFIRMED: the names and shapes quoted above (docs.rs 0.2.2, 100% documented).
- VERIFY AT COMPILE TIME: `TitlebarOptions` fields; whether `div()` is exported at crate root in 0.2.2 (it is in Zed-era gpui, but not individually checked); whether Windows needs any non-default `WindowOptions` for DirectWrite/DPI (ADR-001 says **no** platform features are required on Windows, and docs.rs built the crate on `x86_64-unknown-linux-gnu`, so cross-platform compilation of the crate itself is proven — but a Windows **GUI** build is unverified here, see §4).
- Do **not** assume Zed's internal `cx` idioms from newer/older gpui versions carry over; `Application::run`'s `&mut App` callback and `AppContext`-as-trait are the current 0.2.2 reality.

---

## 4. Build blockers in this environment

**Environment:** `rustc 1.100.0`, target `x86_64-pc-windows-gnu`, MinGW **gcc 6.3**, **no cmake**, **no MSVC**. rustc is new enough that MSRV is a non-issue for every pin above; the blockers are all **native build tooling**.

| Dependency | Risk | Why | Mitigation |
|---|---|---|---|
| **`z3` / `z3-sys`** | **HARD BLOCKER** | `z3-sys` builds Z3 from source with **cmake** (no prebuilt binary is shipped for this target). cmake is absent → build fails. | Make it a **cargo feature**: `z3-validation = ["dep:z3"]`, **default off**. Define a `PolicyValidator` trait; ship a **pure-Rust fallback validator** (static conflict/overbreadth checks over the rule table) used by all tests and dev builds. Enable `z3-validation` only in a production build environment that has cmake (record the toolchain requirement in CI). Keep ADR-006 semantics: any validator failure ⇒ deny. |
| **`gpui` 0.2.2** | **HIGH RISK / likely blocker** | `windows-rs` itself is pure Rust (no MSVC needed), but gpui's **build-dependencies** include `bindgen ^0.71` (**needs libclang**), `cbindgen ^0.28`, `naga ^25.0`, and `embed-resource ^3.0` (**needs a Windows resource compiler, `windres`**), plus `calloop`, `oo7`, `zed-font-kit` (optional), `taffy =0.9.0`. With gcc 6.3, no MSVC and no libclang, a Windows GUI build is unlikely to succeed as-is. | Feature-gate the `ui` crate: `ui = ["dep:gpui"]`, default off for logic-crate tests. Keep **all** core/policy/registry/store crates headless and testable without gpui (optionally use `Application::headless()` for gpui-adjacent unit tests). If the P0 spike fails, options are: install libclang + verify `windres` from MinGW binutils, or evaluate the `gpui-ce` fork per ADR-001 via a **new ADR**. Do not silently drop GPUI from the plan. |
| **`rusqlite` (`bundled`)** | **LOW–MEDIUM** | Needs a C compiler (`cc` crate). MinGW gcc 6.3 **is present**, so the SQLite amalgamation (C99) should compile. gcc 6.3 is old, so test early. | Keep `bundled` (there is no system `libsqlite3` on MinGW). If it fails, try prebuilding SQLite with MinGW and pointing `libsqlite3-sys` at it, or revisit the toolchain. Do **not** switch to a non-bundled build and assume a system library exists. |
| **`tree-sitter` + `tree-sitter-powershell`** | **MEDIUM** | `cc` build script compiles generated C; gcc 6.3 covers C11 in practice but verify. ABI must match `tree-sitter 0.26.x` (see §1.2). | Feature-gate `tree-sitter-split`; default to the fail-closed hand-written splitter in §1.3 for tests. Add a unit test that parses a compound PowerShell command to catch ABI mismatch at compile/test time. |
| **`trybuild`** | **LOW (dev-only)** | Compile-fail `.stderr` snapshots are exact-text and compiler-specific; rustc 1.100 will produce output that differs from upstream snapshots. | Regenerate locally with `TRYBUILD=overwrite cargo test` once when the toolchain is settled; commit the regenerated `.stderr` files. Otherwise CI compile-fail tests fail spuriously. |
| **`windows` 0.61.3** | **LOW** | Pure Rust bindings; declarations only, links to Win32 DLLs via MinGW import libs. | Fine. Keep the feature list minimal (§1.1) to reduce build time. |
| **`chrono` / `time` / `tracing` / `serde` / `uuid` / `hex` / `sha2`** | **LOW** | Pure Rust. | None. |

### Recommended feature layout

```toml
# workspace root
[workspace.package]
edition = "2024"

[workspace.dependencies]
# ... pins from §1.1, with `optional = true` added to gpui, z3, tree-sitter*

# in the binary / facade crate
[features]
default = ["core"]                                   # headless, testable core
core = []
ui = ["dep:gpui", "dep:tree-sitter", "dep:tree-sitter-powershell"]
z3-validation = ["dep:z3"]
native-gui = ["ui"]                                  # the only target that needs the GUI toolchain
```

**Rule of thumb:** `cargo test --workspace` with default features must succeed on this box. Anything requiring cmake, libclang, `windres`, or MSVC is an opt-in feature, never a default.

---

## 5. Confidence summary

- **CONFIRMED:** all version numbers/dates (crates.io API), `gpui_platform` absence, gpui 0.2.2 `Application`/`App`/`Render`/`WindowOptions`/`open_window` shapes (docs.rs), windows 0.61.3 being the newest non-yanked 0.61 patch, and the 0.62.2-drawn windows function signatures for `CreateMutexW`, `GetLastError`, `AllocateAndInitializeSid`, `CheckTokenMembership`, `IsUserAnAdmin`.
- **LIKELY / verify at compile time:** the exact DPI constant/function module path in 0.61, `PSID`/`FreeSid`/RID constant paths, `BOOL` vs `bool` parameter types in 0.61.3, `TitlebarOptions` fields, and whether `gpui 0.2.2`'s Windows GUI build completes on this MinGW-only toolchain.
- **Explicitly not done:** no builds, no lockfile generation, no git operations. All of the above is source/API evidence only.
