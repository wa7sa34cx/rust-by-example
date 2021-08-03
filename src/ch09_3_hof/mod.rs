fn is_odd(n: i32) -> bool {
    n % 2 == 1
}

pub fn run() {
    let upper = 1000;

    // Functional approach
    let sum_of_squared_odd_numbers: i32 = (0..)
        .map(|n| n * n) // All natural numbers squared
        .take_while(|&n_squared| n_squared < upper) // Below upper limit
        .filter(|&n_squared| is_odd(n_squared)) // That are odd
        .map(|n_squared| {
            println!("{}", n_squared);
            n_squared
        })
        .fold(0, |acc, n_squared| acc + n_squared); // Sum them

    println!("sum: {}", sum_of_squared_odd_numbers);
    
    println!();
    println!("{}", 0 % 2);
    println!("{}", 1 % 2);
    println!("{}", 2 % 2);
}
