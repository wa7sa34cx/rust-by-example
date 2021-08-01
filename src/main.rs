#![allow(dead_code, unused_imports)]

mod ch01_0_hello_world;
mod ch01_2_2_1_list;
mod ch01_2_3_color;
mod ch02_2_matrix;
mod ch03_1_structs;
mod ch05_types;

use ch01_0_hello_world as hello_world;
use ch01_2_2_1_list as list;
use ch01_2_3_color as color;
use ch02_2_matrix as matrix;
use ch03_1_structs as structs;
use ch05_types as types;

fn main() {
    // hello_world::run();
    // list::run();
    // color::run();
    // matrix::run();
    // structs::run();
    types::run();
}
