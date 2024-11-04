# fun-htmx

[![Crates.io Version](https://img.shields.io/crates/v/fun-htmx)](https://crates.io/crates/fun-htmx)
[![License](https://img.shields.io/github/license/jcornaz/fun-htmx)](./LICENSE)
![MSRV](https://img.shields.io/crates/msrv/fun-htmx)
[![Build Status](https://img.shields.io/github/actions/workflow/status/jcornaz/fun-htmx/.github%2Fworkflows%2Fcheck.yml?branch=main)](https://github.com/jcornaz/fun-htmx/actions/workflows/check.yml?query=branch%3Amain)
[![docs.rs](https://img.shields.io/docsrs/fun-htmx)](https://docs.rs/fun-htmx)


This crate provides a collection of [HTMX](https://htmx.org) attributes for [`fun-html`](https://github.com/jcornaz/fun-html/)

# Example

```
use fun_html::{elt::{script_empty, button, text}, attr::src};
use fun_htmx::{hx_get, hx_swap_outer_html};

let quick_start = [
  script_empty([src("https://unpkg.com/htmx.org@2.0.3")]),
  button(
    [hx_get("/clicked"), hx_swap_outer_html()],
    text("Click Me")
  ),
];
```


## Feature flags

* `std`: enabled by default. must be disabled to compile to `no_std`


## MSRV

The minimum supported rust version is currently `1.60`.

It will be updated when required, and that will not be considered a breaking change (it can happen in a minor version).


## MIT License

Copyright (c) 2024 Jonathan Cornaz

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
