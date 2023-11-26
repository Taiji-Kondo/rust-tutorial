// Mutable and immutable variables
// fn main() {
//     // let x = 5; // immutable
//     let mut x = 5; // mutable
//     println!("The value of x is: {}", x);
//     x = 6; // if x is immutable, this will cause an error
//     println!("The value of x is: {}", x);
// }

// Shadowing
fn main() {
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);
}