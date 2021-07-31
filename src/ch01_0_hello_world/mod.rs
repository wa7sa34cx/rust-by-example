pub fn run() {
    // Statements here are executed when the compiled binary is called

    // Print text to the console
    println!("Hello World!");
    println!("I'm a Rustacean!");

    // ---
    println!();

    // Comments
    let x = 5 + /* 90 + */ 5;
    println!("Is `x` 10 or 100? x = {}", x);

    // ---
    println!();

    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // ---
    println!();

    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    // ---
    println!();

    // Special formatting can be specified after a `:`.
    println!(
        "{} of {:b} people know binary, the other half doesn't",
        1, 2
    );

    // ---
    println!();

    // You can right-align text with a specified width. This will output
    // "     1". 5 white spaces and a "1".
    println!("{number:>width$}", number = 1, width = 6);

    // You can pad numbers with extra zeroes. This will output "000001".
    println!("{number:>0width$}", number = 1, width = 6);

    // ---
    println!();

    println!("My name is {0}, {1} {0}", "Bond", "James");

    // ---
    println!();

    #[allow(dead_code)]
    #[derive(Debug)]
    struct Structure(i32);

    println!("This struct `{:?}` can be print...", Structure(3));

    // ---
    println!();

    let pi = 3.141592;
    println!("Pi is roughly {:.3}", pi);

    // ---
    println!();

    let peter = Person {
        name: "Peter",
        age: 27,
    };
    println!("{:#?}", peter);
    println!("{:?}", peter);
}

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}
