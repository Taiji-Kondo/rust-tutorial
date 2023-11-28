use std::io;

fn main() {
    let mut x = String::new();

    println!("Please input a number");

    io::stdin()
        .read_line(&mut x)
        .expect("Failed to read line");

    let x: u32 = x.trim()
        .parse()
        .expect("Please type a number");

    if x > 5 {
        println!("x is greater than 5")
    } else {
        println!("x is less than 5")
    }

    let y = if x > 5 { "greater" } else { "less" };
    println!("x is {} than 5", y);
}
