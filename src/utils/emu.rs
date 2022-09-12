#![allow(dead_code, unused)]

use crate::cart::CartContext;

#[derive(Debug, Default)]
struct EmuContext {
    paused : bool,
    running : bool,
    ticks : u64
}

impl EmuContext {
    pub fn get_context() {
        todo!();
    }
}

pub fn run() -> Result<(), ()> {
    
    let mut ctx = CartContext::new();

    ctx.load("src/utils/poke.gb");

    Ok(())
}
