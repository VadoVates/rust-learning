use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Zgadnij liczbę");

    let secret_number = rand::thread_rng().gen_range(1..100);

    loop {
        println!("Wprowadź liczbę o której myślisz");
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Błąd odczytu");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Miała być liczba, debilu");
                continue;
            }
        };

        println!("Wybrałeś: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Za mało"),
            Ordering::Equal => {
                println!("Git, zgadnięte");
                break;
            }
            Ordering::Greater => println!("Za dużo"),
        }
    }
}
