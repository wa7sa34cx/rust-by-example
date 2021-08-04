use std::cmp::Ordering;

fn longer<'a, 'b>(a: &'a str, b: &'a str) -> &'a str {
    // if a > b {
    //     return a;
    // }
    // if a < b {
    //     return b;
    // }
    // "equal"

    match a.len().cmp(&b.len()) {
        Ordering::Less => b,
        Ordering::Greater => a,
        Ordering::Equal => "equal",
    }
}

#[derive(Debug)]
struct Borrowed<'a>(&'a i32);

// Similarly, both references here must outlive this structure.
#[derive(Debug)]
struct NamedBorrowed<'a> {
    x: &'a i32,
    y: &'a i32,
}

// An enum which is either an `i32` or a reference to one.
#[derive(Debug)]
enum Either<'a> {
    Num(i32),
    Ref(&'a i32),
}

pub fn run() {
    let a = "Hello";
    let b = "World!";

    println!("{}", longer(a, b));

    //--
    println!();

    let x = 18;
    let y = 15;

    let single = Borrowed(&x);
    let double = NamedBorrowed { x: &x, y: &y };
    let reference = Either::Ref(&x);
    let number = Either::Num(y);

    println!("x is borrowed in {:?}", single);
    println!("x and y are borrowed in {:?}", double);
    println!("x is borrowed in {:?}", reference);
    println!("y is *not* borrowed in {:?}", number);
}
