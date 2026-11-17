//! O0.2 gate: a tool registered without a branded risk declaration must fail
//! to compile. Snapshots are compiler-version-specific (generated with the
//! pinned rustc 1.100 via `TRYBUILD=overwrite`).

#[test]
fn unbranded_tool_does_not_compile() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/*.rs");
}
