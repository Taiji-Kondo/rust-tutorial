fn main() {
    // for_loop();
    while_loop();
}

fn for_loop() {
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

fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");
}