fn main() {
    let mut count = 0;

    'count_up: loop {
        println!("count: {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining: {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'count_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("end count: {}", count);
}
