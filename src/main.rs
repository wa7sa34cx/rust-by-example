#![allow(dead_code, unused_imports)]

mod ch01_0_hello_world;
mod ch01_2_2_1_list;
mod ch01_2_3_color;
mod ch02_2_matrix;
mod ch03_1_structs;
mod ch05_types;
mod ch08_3_fizzbuzz;
mod ch08_5_1_3_pointers;
mod ch09_2_1_capturing;
mod ch09_2_2_as_input;
mod ch09_2_6_2_sti;
mod ch09_3_hof;
mod ch10_02_struct_visibility;

use ch01_0_hello_world as hello_world;
use ch01_2_2_1_list as list;
use ch01_2_3_color as color;
use ch02_2_matrix as matrix;
use ch03_1_structs as structs;
use ch05_types as types;
use ch08_3_fizzbuzz as fizzbuzz;
use ch08_5_1_3_pointers as pointers;
use ch09_2_1_capturing as capturing;
use ch09_2_2_as_input as as_input;
use ch09_2_6_2_sti as sti;
use ch09_3_hof as hof;
use ch10_02_struct_visibility as sv;

fn main() {
    // hello_world::run();
    // list::run();
    // color::run();
    // matrix::run();
    // structs::run();
    // types::run();
    // fizzbuzz::run();
    // pointers::run();
    // capturing::run();
    // as_input::run();
    // sti::run();
    // hof::run();
    sv::run();
}
