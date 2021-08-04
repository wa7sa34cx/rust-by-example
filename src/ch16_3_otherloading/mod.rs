use std::ops;

#[derive(Debug)]
struct Foo {
    foo: i32,
}

impl ops::Add for Foo {
    type Output = Self;

    fn add(self, _rhs: Self) -> Self {
        Foo {
            foo: self.foo + _rhs.foo,
        }
    }
}

pub fn run() {
    let a = Foo { foo: 2 };
    let b = Foo { foo: 3 };
    println!("a + b = {:?}", a + b);
}
