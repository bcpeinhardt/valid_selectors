#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/parse_input_as_string.rs");

    // TODO: Add test case for entry witout quotation marks
}