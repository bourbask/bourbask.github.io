mod app;
mod components;
mod data;
mod services;

use app::App;
use leptos::*;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(start)] // ← Ajouter "start" ici !
pub fn hydrate() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).expect("Failed to initialize logger");

    log::info!("🦀 Starting Leptos Portfolio with modular components...");

    leptos::mount_to_body(App);
}
