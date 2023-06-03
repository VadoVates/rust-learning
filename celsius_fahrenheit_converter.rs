use std::io;
fn main() {
    loop {
        println!("MENU");
        println!("1 - Fahrenheit => Celsjusz");
        println!("2 - Celsjusz => Fahrenheit");
        println!("0 - wyjście");
        let mut wybor: String = String::new();
        io::stdin().read_line(&mut wybor).expect("Błąd odczytu");

        let wybor: u32 = match wybor.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Błąd parsowania do liczby! Wprowadź liczbę jeszcze raz!");
                continue;
            }
        };

        match wybor {
            1 => fahrenheit_to_celcius(),
            2 => celcius_to_fahrenheit(),
            0 => break,
            _ => println!("Powinieneś wprowadzić inną liczbę!"),
        }
    }
}

fn fahrenheit_to_celcius() {
    println!("wewnątrz funkcji Fahrenheit -> Celsjusz!");

    loop {
        let mut fahrenheit = String::new();
        println!("Wprowadź temperaturę fahrenheita do kowersji: ");
        io::stdin()
            .read_line(&mut fahrenheit)
            .expect("Błąd odczytu");
        let fahrenheit: f32 = match fahrenheit.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Miałeś wprowadzić dowolną liczbę zmiennoprzecinkową!");
                continue;
            }
        };

        let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
        println!("Temperatura Celsjusza wynosi: {celsius}");
        break;
    }
}

fn celcius_to_fahrenheit() {
    println!("Wewnątrz funkcji Celsjusz -> Fahrenheit!");
    loop {
        let mut celsius = String::new();
        println!("Wprowadź temperaturę fahrenheita do kowersji: ");
        io::stdin().read_line(&mut celsius).expect("Błąd odczytu");
        let celsius: f32 = match celsius.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Miałeś wprowadzić dowolną liczbę zmiennoprzecinkową!");
                continue;
            }
        };

        let fahrenheit = celsius * 9.0 / 5.0 + 32.0;
        println!("Temperatura Fahrenheita wynosi: {fahrenheit}");
        break;
    }
}
