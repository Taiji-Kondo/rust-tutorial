// Normal
// fn main() {
//     let width: u32 = 5;
//     let high: u32 = 6;
//
//     let area = area(&width, &high);
//     println!("{}", area);
// }
//
// fn area(width: &u32, high: &u32) -> u32 {
//     width * high
// }

// Tuple
fn main() {
    let rect: (u32, u32) = (5, 6);

    let area = area(&rect);
    println!("{}", area);
}

fn area((width, high): &(u32, u32)) -> u32 {
    width * high
}