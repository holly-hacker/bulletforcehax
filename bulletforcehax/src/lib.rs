extern crate wasm_bindgen;

use log::{Level, info};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    // make panics better
    console_error_panic_hook::set_once();

    // allow logging to console
    // could turn off logging in release builds for perf+size improvement, see https://crates.io/crates/console_log or https://docs.rs/log/0.4.10/log/#compile-time-filters
    console_log::init_with_level(Level::Debug).unwrap();

    info!("BulletForceHax wasm initialized!");
    Ok(())
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}
