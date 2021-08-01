#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

// 1
fn rect_area(rectangle: Rectangle) -> f32 {
    let Point {x: x1, y: y1} = rectangle.top_left;
    let Point {x: x2, y: y2} = rectangle.bottom_right;

    (x2 - x1) * (y1 - y2)
}

// 2
fn square(point: Point, side: f32) -> Rectangle {
    let Point {x, y} = point;
    let top_left = Point { x: x, y: y + side };
    let bottom_right = Point { x: x + side, y: y };

    Rectangle {
        top_left,
        bottom_right,
    }
}

pub fn run() {
    let top_left = Point { x: 0.0, y: 10.0 };
    let bottom_right = Point { x: 20.0, y: 0.0 };

    let rectangle = Rectangle {
        top_left,
        bottom_right,
    };

    println!("Area of rectangle is: {}", rect_area(rectangle));

    let square = square(Point { x: 0.0, y: 0.0 }, 20.0);
    println!("This is a square: {:#?}", square);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);
}