pub mod asm;
pub mod c_backend;
pub mod vm;

#[cfg(feature = "wasm")]
mod web;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn run_craw(source: &str) -> String {
    web::run_craw(source)
}

pub use vm::{
    DiskConfig, Instr, RunOptions, Vm, VmConfig, load_program, predecode, run_program_with_options,
    run_vm_with_options,
};
