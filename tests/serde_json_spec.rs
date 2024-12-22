#![cfg(feature = "serde_json")]

use fun_htmx::{hx_vals, hx_vals_serde};
use serde::Serialize;

#[test]
fn test_hx_vals_serde() {
    assert_eq!(
        hx_vals_serde(&Example { foo: "bar" }).to_string(),
        hx_vals("{\"foo\":\"bar\"}").to_string(),
    )
}

#[derive(Debug, Serialize)]
struct Example {
    foo: &'static str,
}
