mod classes;
use classes::point::*;

fn main() {
    
    let mut p = Point{ x: 10.0, y: 10.0, z: 10.0 };
    let mut p2 = Point{ x:20.0, y: 10.0, z: 10.0 };

    println!("{}", Point::dist(&p, &p2));

}
