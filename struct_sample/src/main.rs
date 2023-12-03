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

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        other.width < self.width && other.height < self.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let rect1: Rectangle = Rectangle { width: 50, height: 60 };
    let rect2: Rectangle = Rectangle { width: 40, height: 30 };
    let rect3: Rectangle = Rectangle { width: 60, height: 60 };
    let square: Rectangle = Rectangle::square(100);
    println!("rect1: {:?}", rect1);
    println!("rect1: {:?}", rect2);
    println!("rect1: {:?}", rect3);
    println!("square: {:?}", square);

    // let area = area(&rect);
    let area = rect1.area();
    println!("rect1 area: {}", area);
    println!("rect1 can hold rect2: {:?}", rect1.can_hold(&rect2));
    println!("rect1 can hold rect3: {:?}", rect1.can_hold(&rect3));
}