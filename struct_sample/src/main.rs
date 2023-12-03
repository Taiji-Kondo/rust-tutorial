#[derive(Debug)]
// Normal
// fn main() {
//     let width: u32 = 5;
//     let height: u32 = 6;
//
//     let area = area(&width, &height);
//     println!("{}", area);
// }
//
// fn area(width: &u32, height: &u32) -> u32 {
//     width * height
// }

// Tuple
// fn main() {
//     let rect: (u32, u32) = (5, 6);
//
//     let area = area(&rect);
//     println!("{}", area);
// }
//
// fn area((width, height): &(u32, u32)) -> u32 {
//     width * height
// }

// Struct
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect: Rectangle = Rectangle { width: 5, height: 6 };
    println!("{:?}", rect);

    let area = area(&rect);
    println!("{}", area);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}