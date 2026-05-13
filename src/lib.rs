#![allow(unexpected_cfgs, clippy::clone_on_copy)]

mod app;
mod components;
mod data;
mod services;

use app::App;
use leptos::*;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(start)]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    #[cfg(debug_assertions)]
    console_log::init_with_level(log::Level::Debug).expect("Failed to initialize logger");
    #[cfg(not(debug_assertions))]
    console_log::init_with_level(log::Level::Error).expect("Failed to initialize logger");

    leptos::mount_to_body(App);
}
