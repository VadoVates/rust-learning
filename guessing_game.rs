// [dependencies]
// rand = "0.8.5"

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess a number");

    let secret_number = rand::thread_rng().gen_range(1..100);

    loop {
        println!("Write the integer within <0;100> you are thinking of:");
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Read error");


        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Error - that was not an integer");
                continue;
            }
        };
        

        println!("You took: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too little"),
            Ordering::Equal => {
                println!("You guessed it!");
                break;
            }
            Ordering::Greater => println!("Too much"),
        }


    }
}