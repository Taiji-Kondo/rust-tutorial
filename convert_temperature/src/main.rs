use std::io;

fn main() {
    loop {
        println!("Convert celsius to fahrenheit? y/n");

        let mut is_convert_fahrenheit = String::new();
        io::stdin().read_line(&mut is_convert_fahrenheit).expect("Failed to read line");

        match is_convert_fahrenheit.trim() {
            "y" => {
                println!("Please input celsius");
                let mut celsius = String::new();
                io::stdin().read_line(&mut celsius).expect("Failed to read line");

                let fahrenheit: f64 = (celsius.trim().parse().unwrap() * 1.8) + 32;
                println!("{}", fahrenheit);
            },
            "n" => println!("no"),
            _ => {
                println!("Please input y or n");
                continue;
            },
        };
    }
}

// 摂氏（℃）＝【華氏（℉）-32】÷1.8.
// 華氏（℉）＝摂氏（℃）×1.8+32.