use wasm_bindgen::prelude::*;

pub mod asm;
mod web;

#[wasm_bindgen]
pub fn run_craw(source: &str) -> String {
    web::run_craw(source)
}
