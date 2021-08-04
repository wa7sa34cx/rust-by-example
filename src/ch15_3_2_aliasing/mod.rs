#[derive(Debug)]
struct Point {
    x: i32, 
    y: i32
}

pub fn run() {
    let mut point = Point { x: 0, y: 0 };

    let borrowed_point = &point;
    
    let mutable_borrow = &mut point;
    mutable_borrow.y = 2;

    // println!("{:?}", point.y); // !!!! Error!

    println!("{}", mutable_borrow.x);

}