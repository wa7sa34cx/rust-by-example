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
mod ch10_2_struct_visibility;
mod ch13_1_dead_code;
mod ch13_3_cfg;
mod ch14_4_bounds;
mod ch14_5_multiple_bounds;
mod ch15_2_2_partial_moves;
mod ch15_3_2_aliasing;
mod ch15_3_3_ref_pattern;
mod ch15_4_lifetimes;
mod ch16_2_dyn_traints;
mod ch16_3_otherloading;
mod ch16_5_fibonacci;
mod ch16_6_implementation;
mod ch16_sheep;
mod ch17_1_1_designators;
mod ch17_1_2_macro_overload;
mod ch17_1_3_macro_repeat;
mod ch17_macro_rules;
mod ch18_2_3_and_then;
mod ch18_4_5_wrapping_errors;
mod ch18_5_iterating_over_results;
mod ch19_3_long_string;
mod ch19_7_hashmap;
mod ch20_1_1_map_reduce;
mod ch20_1_threads;
mod ch20_2_channels;
mod ch20_3_path;
mod ch20_5_processes;
mod ch20_6_fs;

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
use ch10_2_struct_visibility as sv;
use ch13_1_dead_code as dead_code;
use ch13_3_cfg as cfg;
use ch14_4_bounds as bounds;
use ch14_5_multiple_bounds as multiple_bounds;
use ch15_2_2_partial_moves as partial_moves;
use ch15_3_2_aliasing as aliasing;
use ch15_3_3_ref_pattern as ref_pattern;
use ch15_4_lifetimes as lifetimes;
use ch16_2_dyn_traints as dyn_traints;
use ch16_3_otherloading as otherloading;
use ch16_5_fibonacci as fibonacci;
use ch16_6_implementation as implementation;
use ch16_sheep as sheep;
use ch17_1_1_designators as designators;
use ch17_1_2_macro_overload as macro_overload;
use ch17_1_3_macro_repeat as macro_repeat;
use ch17_macro_rules as macro_rules;
use ch18_2_3_and_then as and_then;
use ch18_4_5_wrapping_errors as wrapping_errors;
use ch18_5_iterating_over_results as iterating_over_results;
use ch19_3_long_string as long_string;
use ch19_7_hashmap as hashmap;
use ch20_1_1_map_reduce as map_reduce;
use ch20_1_threads as threads;
use ch20_2_channels as channels;
use ch20_3_path as path;
use ch20_5_processes as processes;
use ch20_6_fs as fs;

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
    // sv::run();
    // dead_code::run();
    // bounds::run();
    // multiple_bounds::run();
    // partial_moves::run();
    // aliasing::run();
    // ref_pattern::run();
    // lifetimes::run();
    // sheep::run();
    // dyn_traints::run();
    // otherloading::run();
    // fibonacci::run();
    // implementation::run();
    // macro_rules::run();
    // designators::run();
    // macro_overload::run();
    // macro_repeat::run();
    // and_then::run();
    // wrapping_errors::run();
    // iterating_over_results::run();
    // long_string::run();
    // hashmap::run();
    // threads::run();
    // map_reduce::run();
    // channels::run();
    // path::run();
    // processes::run();
    fs::run().unwrap();
}
