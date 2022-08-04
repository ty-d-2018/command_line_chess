pub mod chess;

use chess::*;


fn main() {
    let mut point_x: Point = Point::new(10, 10);
    let mut point_x2: Point = Point::new(5, 5);
    point_x = point_x / point_x2.clone();
    println!("point_x is ({},{})", point_x.x, point_x.y);
}
