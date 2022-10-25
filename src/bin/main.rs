extern crate utils;

//use std::env;
use utils::emu;

fn main() {
    //env::set_var("RUST_BACKTRACE", "full");
    emu::run().unwrap();
}
