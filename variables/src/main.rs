fn main() {
    // let x = 5; // immutable
    let mut x = 5; // mutable
    println!("The value of x is: {}", x);
    x = 6; // if x is immutable, this will cause an error
    println!("The value of x is: {}", x);
}
