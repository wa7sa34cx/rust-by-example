macro_rules! say_hello {
    () => {
        println!("Hello!");
    };
}

pub fn run() {
    say_hello!();
}
