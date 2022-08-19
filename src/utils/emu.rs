#![allow(dead_code, unused)]

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
    todo!();
}