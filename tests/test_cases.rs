#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/parse_input_as_string.rs");
    t.compile_fail("tests/invalid_syntax_returns_error.rs");
}