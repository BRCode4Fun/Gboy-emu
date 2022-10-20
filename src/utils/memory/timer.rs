#![allow(dead_code, unused)]

trait Timer {
    fn new() -> Self;
    fn tick(&mut self); 
}