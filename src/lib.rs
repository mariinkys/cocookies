// SPDX-License-Identifier: GPL-3.0-only

pub mod api;
pub mod app;
pub mod components;
pub mod models;
pub mod pages;
pub mod utils;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
