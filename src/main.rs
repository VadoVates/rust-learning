use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Zgadnij liczbę");

    let secret_number = rand::thread_rng().gen_range(1..100);

    println!("Secret number is: {secret_number}");

    println!("Wprowadź liczbę o której myślisz");
    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Błąd odczytu");
    let guess: i32 = guess.trim().parse().expect("Wpisz liczbę naturalną cymbale");

    println!("Wybrałeś: {guess}");

    match guess.cmp(&secret_number)
    {
        Ordering::Less => println!("Za mało"),
        Ordering::Equal => println!("Git, zgadnięte"),
        Ordering::Greater => println!("Za dużo"),
    }
}
