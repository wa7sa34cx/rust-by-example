#[derive(Debug)]
struct Foo {
    s: String,
}

impl From<&str> for Foo {
    fn from(s: &str) -> Foo {
        Foo { s: s.to_owned() }
    }
}

pub fn run() {
    let foo = Foo::from("hello");
    println!("{:#?}", foo);
}
