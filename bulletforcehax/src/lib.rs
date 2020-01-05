extern crate wasm_bindgen;

use log::{debug, error, info, Level};
use wasm_bindgen::prelude::*;

mod packets;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
    pub fn startGame();
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    // make panics better
    console_error_panic_hook::set_once();

    // allow logging to console
    // could turn off logging in release builds for perf+size improvement, see https://crates.io/crates/console_log or https://docs.rs/log/0.4.10/log/#compile-time-filters
    console_log::init_with_level(Level::Trace).unwrap();

    info!("BulletForceHax initialized, starting game.");
    startGame();
    Ok(())
}

// TODO: support returning multiple sent packets
// TODO: perhaps pass boolean or enum that tells which socket we're on
#[wasm_bindgen]
pub fn sock_send(data: &[u8]) -> Vec<u8> {
    match packets::Packet::read(&data, packets::Direction::Send) {
        Ok(packet) => {
            debug!("SEND: {:?}", packet);
            match packet.into_vec() {
                Ok(vec) => vec,
                Err(error) => {
                    error!("SEND ERR: {:?}, data: {:?}", error, data);
                    Vec::from(data)
                }
            }
        }
        Err(error) => {
            error!("SEND ERR: {:?}, data: {:?}", error, data);
            Vec::from(data)
        }
    }
}

#[wasm_bindgen]
pub fn sock_recv(data: &[u8]) -> Vec<u8> {
    match packets::Packet::read(&data, packets::Direction::Recv) {
        Ok(packet) => {
            debug!("RECV: {:?}", packet);
            match packet.into_vec() {
                Ok(vec) => vec,
                Err(error) => {
                    error!("RECV ERR: {:?}, data: {:?}", error, data);
                    Vec::from(data)
                }
            }
        }
        Err(error) => {
            error!("RECV ERR: {:?}, data: {:?}", error, data);
            Vec::from(data)
        }
    }
}
