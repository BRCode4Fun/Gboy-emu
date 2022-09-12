#![allow(unused_imports)]

extern crate utils;

//use std::env;
use utils::emu;

#[allow(unused_must_use)]
fn main() {

    //env::set_var("RUST_BACKTRACE", "full");
    emu::run();
}
