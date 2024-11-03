#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
// TODO #![warn(missing_docs)]

extern crate alloc;

use alloc::borrow::Cow;

use fun_html::Attribute;

/// [`hx-boost`](https://htmx.org/attributes/hx-boost/) attribute
pub fn hx_boost(value: bool) -> Attribute {
    Attribute::new("hx-boost", boolean(value))
}

/// [`hx-get`](https://htmx.org/attributes/hx-get/) attribute
pub fn hx_get(path: impl Into<Cow<'static, str>>) -> Attribute {
    Attribute::new("hx-get", path)
}

/// [`hx-post`](https://htmx.org/attributes/hx-post/) attribute
pub fn hx_post(path: impl Into<Cow<'static, str>>) -> Attribute {
    Attribute::new("hx-post", path)
}

/// [`hx-put`](https://htmx.org/attributes/hx-put/) attribute
pub fn hx_put(path: impl Into<Cow<'static, str>>) -> Attribute {
    Attribute::new("hx-put", path)
}

/// [`hx-patch`](https://htmx.org/attributes/hx-patch/) attribute
pub fn hx_patch(path: impl Into<Cow<'static, str>>) -> Attribute {
    Attribute::new("hx-patch", path)
}

/// [`hx-delete`](https://htmx.org/attributes/hx-delete/) attribute
pub fn hx_delete(path: impl Into<Cow<'static, str>>) -> Attribute {
    Attribute::new("hx-delete", path)
}

/// [`hx-trigger`](https://htmx.org/attributes/hx-trigger/) attribute
pub fn hx_trigger(path: impl Into<Cow<'static, str>>) -> Attribute {
    Attribute::new("hx-trigger", path)
}

/// [`hx-select`](https://htmx.org/attributes/hx-select/) attribute
pub fn hx_select(path: impl Into<Cow<'static, str>>) -> Attribute {
    Attribute::new("hx-select", path)
}

/// [`hx-target`](https://htmx.org/attributes/hx-target/) attribute
pub fn hx_target(path: impl Into<Cow<'static, str>>) -> Attribute {
    Attribute::new("hx-target", path)
}

/// [`hx-swap`](https://htmx.org/attributes/hx-swap/) attribute
pub fn hx_swap(path: impl Into<Cow<'static, str>>) -> Attribute {
    Attribute::new("hx-swap", path)
}

/// `hx-swap="innerHTML"`
///
/// See [`hx-swap`](https://htmx.org/attributes/hx-swap/) attribute
pub fn hx_swap_inner_html() -> Attribute {
    hx_swap("innerHTML")
}

/// `hx-swap="outerHTML"`
///
/// See [`hx-swap`](https://htmx.org/attributes/hx-swap/) attribute
pub fn hx_swap_outer_html() -> Attribute {
    hx_swap("outerHTML")
}

/// `hx-swap="textContent"`
///
/// See [`hx-swap`](https://htmx.org/attributes/hx-swap/) attribute
pub fn hx_swap_text_content() -> Attribute {
    hx_swap("textContent")
}

/// `hx-swap="beforebegin"`
///
/// See [`hx-swap`](https://htmx.org/attributes/hx-swap/) attribute
pub fn hx_swap_before_begin() -> Attribute {
    hx_swap("beforebegin")
}

/// `hx-swap="afterbegin"`
///
/// See [`hx-swap`](https://htmx.org/attributes/hx-swap/) attribute
pub fn hx_swap_after_begin() -> Attribute {
    hx_swap("afterbegin")
}

/// `hx-swap="beforeend"`
///
/// See [`hx-swap`](https://htmx.org/attributes/hx-swap/) attribute
pub fn hx_swap_before_end() -> Attribute {
    hx_swap("beforeend")
}

/// `hx-swap="afterend"`
///
/// See [`hx-swap`](https://htmx.org/attributes/hx-swap/) attribute
pub fn hx_swap_after_end() -> Attribute {
    hx_swap("afterend")
}

/// `hx-swap="delete"`
///
/// See [`hx-swap`](https://htmx.org/attributes/hx-swap/) attribute
pub fn hx_swap_delete() -> Attribute {
    hx_swap("delete")
}

/// `hx-swap="none"`
///
/// See [`hx-swap`](https://htmx.org/attributes/hx-swap/) attribute
pub fn hx_swap_none() -> Attribute {
    hx_swap("none")
}

/// [`hx-push-url`](https://htmx.org/attributes/hx-push-url/) attribute using a boolean
pub fn hx_push_url(value: bool) -> Attribute {
    hx_push_url_str(boolean(value))
}

/// [`hx-push-url`](https://htmx.org/attributes/hx-push-url/) attribute using an URL
pub fn hx_push_url_str(url: impl Into<Cow<'static, str>>) -> Attribute {
    Attribute::new("hx-push-url", url)
}

/// `hx-swap-oob="true"
///
/// See [`hx-swap`](https://htmx.org/attributes/hx-swap/) attribute
pub fn hx_swap_oob() -> Attribute {
    hx_swap_oob_swap("true")
}

/// [`hx-swap-oob`](https://htmx.org/attributes/hx-swap-oob/) attribute
pub fn hx_swap_oob_swap(value: impl Into<Cow<'static, str>>) -> Attribute {
    Attribute::new("hx-swap-oob", value)
}

/// [`hx-on*`](https://htmx.org/attributes/hx-on/) attributes
pub fn hx_on(event: &'static str, action: impl Into<Cow<'static, str>>) -> Attribute {
    Attribute::new_unsafe_name(alloc::format!("hx-on:{event}"), action)
}

/// [`hx-on:htmx:before-request`](https://htmx.org/attributes/hx-on/) attribute
pub fn hx_on_htmx_before_request(action: impl Into<Cow<'static, str>>) -> Attribute {
    Attribute::new("hx-on::before-request", action)
}

/// [`hx-on:htmx:after-request`](https://htmx.org/attributes/hx-on/) attribute
pub fn hx_on_htmx_after_request(action: impl Into<Cow<'static, str>>) -> Attribute {
    Attribute::new("hx-on::after-request", action)
}

fn boolean(value: bool) -> &'static str {
    if value {
        "true"
    } else {
        "false"
    }
}
