use std::io;

fn main() {
    println!("Zgadnij liczbę");

    println!("Wprowadź liczbę o której myślisz");
    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Błąd odczytu");

    println!("Zgadłeś: {guess}");
}
