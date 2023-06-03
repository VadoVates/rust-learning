use std::io;
fn main() {
    loop {
        println!("MENU");
        println!("1 - Fahrenheit => Celsius");
        println!("2 - Celsius => Fahrenheit");
        println!("0 - Exit");
        let mut choice: String = String::new();
        io::stdin().read_line(&mut choice).expect("Read-line error");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Parsing number error (not an unsigned integer?)");
                continue;
            }
        };

        match choice {
            1 => fahrenheit_to_celcius(),
            2 => celcius_to_fahrenheit(),
            0 => break,
            _ => println!("Thou shall enter another number!"),
        }
    }
}

fn fahrenheit_to_celcius() {

    loop {
        let mut fahrenheit = String::new();
        println!("Write Fahrenheit temperature to convert into Celsius: ");
        io::stdin()
            .read_line(&mut fahrenheit)
            .expect("Błąd odczytu");
        let fahrenheit: f32 = match fahrenheit.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Parsing numbe error (not a floating point number?)");
                continue;
            }
        };

        let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
        println!("Celsius temperature is: {celsius}");
        break;
    }
}

fn celcius_to_fahrenheit() {
    loop {
        let mut celsius = String::new();
        println!("Write Celsius temperature to convert: ");
        io::stdin().read_line(&mut celsius).expect("Read-line error");
        let celsius: f32 = match celsius.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Parsing numbe error (not a floating point number?)");
                continue;
            }
        };

        let fahrenheit = celsius * 9.0 / 5.0 + 32.0;
        println!("Fahrenheit temperature is: {fahrenheit}");
        break;
    }
}
