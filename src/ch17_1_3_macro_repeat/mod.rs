macro_rules! sum {
    ($a:literal) => ($a);
    ($a:literal, $($b:literal),+) => (
        $a + sum!($($b),+)
    );

}

pub fn run() {
    println!("{}", sum!(1));
    println!("{}", sum!(1, 2));
    println!("{}", sum!(1, 2, 3));
}
