pub fn run() {
    // A counter variable
    let mut n = 0;

    // Loop while `n` is less than 101
    while n < 101 {
        // Increment counter
        n += 1;

        if n % 15 == 0 {
            println!("{} fizzbuzz", n);
            continue;
        }
        if n % 3 == 0 {
            println!("{} fizz", n);
            continue;
        }
        if n % 5 == 0 {
            println!("{} buzz", n);
            continue;
        }
        println!("{}", n);
    }
}
