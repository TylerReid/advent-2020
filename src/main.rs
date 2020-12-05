#![allow(dead_code)]
#[macro_use]
extern crate lazy_static;

mod four;
mod five;
mod five_clever;

fn main() {
    five_clever::day_five();
}
