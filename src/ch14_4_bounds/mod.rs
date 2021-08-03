use std::fmt::Display;

fn print<T: Display>(arg: T) {
    println!("{}", arg);
}

pub fn run() {
    // let a = vec!(1, 2, 3);
    // print(a);
    
    let b = "Hello!";
    print(b);
  }