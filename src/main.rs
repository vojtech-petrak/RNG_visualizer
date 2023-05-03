use std::io::{stdout, stdin, Write};

fn main() {
    let shift: u32 = input_u32(&"shift", 30, 31);
    let mut seed: u32 = input_u32(&"seed", 510510, 4294967295);
    let mut output: (u32, u32) = (seed & 1, 0);

    for _i in 0..input_u32(&"range", 4294967295, 2147483648) {
        match output.0 {
            0 => print!(" "),
            1 => print!("█"),
            _ => (),
        }
        seed >>= 1;
        output.1 = seed & 1;
        seed |= (output.0 ^ output.1) << shift;
        output.0 = output.1;
    }
    
    loop {
        let mut input: String = String::new();
        stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();
        if input == "quit" || input == "exit" {
            break;
        }
    }
}

fn input_u32(property: &str, default: u32, max: u32) -> u32 {
    loop {
        print!("{property}: [{default}] = ");
        stdout().flush().unwrap();

        let mut input: String = String::new();
        stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();

        if input == "" { return default; }
        match input.parse::<u32>() {
            Ok(number) => {
                if number <= max {
                    return number;
                } else {
                    println!("{number} is out of bound");
                }
            },
            Err(_) => println!("{input} is en invalid value"),
        }
    }
}
