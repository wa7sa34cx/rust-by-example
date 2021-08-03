use std::fmt;
use std::fmt::{Debug, Display};

fn print<T: Debug + Display>(t: T) {
    println!("Debug:\n{:?}", t);
    println!("Display:\n{}", t);
}

#[derive(Debug)]
struct Foo {
    f: i32,
}

impl fmt::Display for Foo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "f = {}", self.f)
    }
}

fn compare_types<T: Debug, U: Debug>(t: &T, u: &U) {
    println!("t: `{:?}`", t);
    println!("u: `{:?}`", u);
}

pub fn run() {
    let a = "42";
    print(a);

    println!();

    let foo = Foo { f: 42 };
    print(foo);

    println!();

    let array = [1, 2, 3];
    let vec = vec![1, 2, 3];
    compare_types(&array, &vec);
}
