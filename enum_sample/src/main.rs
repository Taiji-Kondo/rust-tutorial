#[derive(Debug)]


enum IpAddressKind {
    V4(u32, u32, u32, u32),
    V6(String),
}

fn main() {
    println!("{:?}", IpAddressKind::V4(100, 255, 255, 255));
    println!("{:?}", IpAddressKind::V6(String::from("::1")));
}
