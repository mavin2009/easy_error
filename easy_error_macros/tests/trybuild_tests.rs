// easy_error_macros/tests/trybuild_tests.rs

#[test]
fn ui_tests() {
    let t = trybuild::TestCases::new();
    // These paths are relative to the `easy_error_macros` directory
    t.pass("tests/compile-pass/define_error.rs");
    t.pass("tests/compile-pass/try_easy.rs");
    t.compile_fail("tests/compile-fail/invalid_define_error.rs");
    t.compile_fail("tests/compile-fail/invalid_try_easy.rs");
}
