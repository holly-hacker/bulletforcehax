extern crate wasm_bindgen;

use log::{Level, debug, info};
use wasm_bindgen::prelude::*;

mod packets;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
    pub fn startGame();
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    // make panics better
    console_error_panic_hook::set_once();

    // allow logging to console
    // could turn off logging in release builds for perf+size improvement, see https://crates.io/crates/console_log or https://docs.rs/log/0.4.10/log/#compile-time-filters
    console_log::init_with_level(Level::Debug).unwrap();

    info!("BulletForceHax initialized, starting game.");
    startGame();
    Ok(())
}

// TODO: support returning multiple sent packets
#[wasm_bindgen]
pub fn sock_send(data: &[u8]) -> Vec<u8> {
    match packets::Packet::read(data) {
        Ok(packet) => { debug!("SEND: length of {}, {:?}", data.len(), packet); Vec::from(data) }, // TODO: return new packet
        Err(error) => { debug!("SEND ERR: {:?}, data: {:?}", error, data); Vec::from(data) }
    }
}

#[wasm_bindgen]
pub fn sock_recv(data: &[u8]) -> Vec<u8> {
    match packets::Packet::read(data) {
        Ok(packet) => { debug!("RECV: length of {}, {:?}", data.len(), packet); Vec::from(data) }, // TODO: return new packet
        Err(error) => { debug!("RECV ERR: {:?}, data: {:?}", error, data); Vec::from(data) }
    }
}
