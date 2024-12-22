use fun_html::Attribute;
use fun_htmx::*;
use rstest::rstest;

#[rstest]
#[case(hx_get("/path"), "hx-get=\"/path\"")]
#[case(hx_post("/path"), "hx-post=\"/path\"")]
#[case(hx_put("/path"), "hx-put=\"/path\"")]
#[case(hx_patch("/path"), "hx-patch=\"/path\"")]
#[case(hx_delete("/path"), "hx-delete=\"/path\"")]
#[case(hx_trigger("foo"), "hx-trigger=\"foo\"")]
#[case(hx_select("something"), "hx-select=\"something\"")]
#[case(hx_target("something"), "hx-target=\"something\"")]
#[case(hx_swap("innerHTML swap:3s"), "hx-swap=\"innerHTML swap:3s\"")]
#[case(hx_swap_inner_html(), "hx-swap=\"innerHTML\"")]
#[case(hx_swap_outer_html(), "hx-swap=\"outerHTML\"")]
#[case(hx_swap_text_content(), "hx-swap=\"textContent\"")]
#[case(hx_swap_before_begin(), "hx-swap=\"beforebegin\"")]
#[case(hx_swap_after_begin(), "hx-swap=\"afterbegin\"")]
#[case(hx_swap_before_end(), "hx-swap=\"beforeend\"")]
#[case(hx_swap_after_end(), "hx-swap=\"afterend\"")]
#[case(hx_swap_delete(), "hx-swap=\"delete\"")]
#[case(hx_swap_none(), "hx-swap=\"none\"")]
#[case(hx_push_url(true), "hx-push-url=\"true\"")]
#[case(hx_push_url(false), "hx-push-url=\"false\"")]
#[case(hx_push_url_str("/foo/bar"), "hx-push-url=\"/foo/bar\"")]
#[case(hx_swap_oob(), "hx-swap-oob=\"true\"")]
#[case(
    hx_swap_oob_swap("innerHTML swap:3s"),
    "hx-swap-oob=\"innerHTML swap:3s\""
)]
#[case(hx_boost(true), "hx-boost=\"true\"")]
#[case(hx_boost(false), "hx-boost=\"false\"")]
#[case(hx_on("click", "alert('hello')"), "hx-on:click=\"alert('hello')\"")]
#[case(
    hx_on_htmx_before_request("alert('hello')"),
    "hx-on::before-request=\"alert('hello')\""
)]
#[case(
    hx_on_htmx_after_request("alert('hello')"),
    "hx-on::after-request=\"alert('hello')\""
)]
#[case(hx_confirm("are you sure?"), "hx-confirm=\"are you sure?\"")]
#[case(
    hx_vals("{ \"foo\": \"bar\" }"),
    "hx-vals=\"{ &quot;foo&quot;: &quot;bar&quot; }\""
)]
fn should_render_htmx_attribute(#[case] attribute: Attribute, #[case] expected: &str) {
    assert_eq!(attribute.to_string(), expected);
}
