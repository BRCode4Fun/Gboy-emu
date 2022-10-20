#![allow(dead_code, unused)]

use std::env;

use super::{
    cartridge::CartContext,
    cpu::Cpu,
};

#[derive(Debug, Default)]
pub struct EmuContext {
    paused  : bool,
    running : bool,
    ticks   : u64,
}

impl EmuContext {
    pub fn get_context() {
        todo!();
    }
}

pub fn run() -> Result<(), ()> {

    let file_path : String = env::args().nth(1)
                                .expect("Expected path to the ROM file");

    let mut ctx = CartContext::new();

    ctx.load(&file_path).expect(&format!("Failed to load ROM file: {}", file_path));

    Ok(())
}
