#[test]
fn test_public_api_semver_guards() {
    let tests = trybuild::TestCases::new();
    tests.compile_fail("tests/ui/struct_literal_params.rs");
    tests.compile_fail("tests/ui/exhaustive_response_match.rs");
    tests.pass("tests/ui/unwrap_response_without_error_import.rs");
    tests.pass("tests/ui/into_request_response.rs");
}
