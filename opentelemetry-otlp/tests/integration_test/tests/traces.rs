use integration_test_runner::asserter::{assert_span_eq, read_spans_from_json};
use std::fs::File;

#[test]
fn assert_json() {
    let left = read_spans_from_json(File::open("./expected_traces.json").unwrap());
    let right = read_spans_from_json(File::open("./traces.json").unwrap());
    assert_span_eq(&left, &right);
}
