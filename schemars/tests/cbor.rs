mod util;
use util::*;

#[test]
fn cbor() -> TestResult {
    test_default_generated_schema::<serde_cbor::Value>("cbor")
}
