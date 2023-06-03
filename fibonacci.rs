use std::io;
fn main() {
    loop {
        println!("Fibonacci n-th number");
        println!("0-th number = 0");
        println!("1-st number = 1");
        println!("Every next number is sum of the previos two numbers");
        println!("Which number of the Fibonacci sequence would you like to calculate");

        let mut choice: String = String::new();
        io::stdin().read_line(&mut choice).expect("Read-line error");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Parsing number error (not an integer?)");
                continue;
            }
        };
        println!("{choice}. number of Fibonacci sequence is: {}", fibonacci(choice));
        break;
    }
}

fn fibonacci(n: u32) -> u64 {
    let mut sum: u64 = 0;
    let mut first: u64 = 0;
    let mut second: u64 = 1;
    match n {
        0 => sum = first,
        1 => {
            sum = second;
        }
        _ => {
            for _i in 1..n {
                sum = first + second;
                first = second;
                second = sum;
            }
        }
    }

    sum
}
