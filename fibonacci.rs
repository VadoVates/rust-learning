use std::io;
fn main() {
    loop {
        println!("Fibonacci n-th number");
        println!("Którą liczbę z ciągu Fibonacciego chcesz wyliczyć? ");

        let mut wybor: String = String::new();
        io::stdin().read_line(&mut wybor).expect("Błąd odczytu");

        let wybor: u32 = match wybor.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Błąd parsowania do liczby! Wprowadź liczbę jeszcze raz!");
                continue;
            }
        };
        println!("Wynik to: {}", fibonacci(wybor));
        break;
    }
}

fn fibonacci(n: u32) -> u64 {
    let mut suma: u64 = 0;
    let mut first: u64 = 0;
    let mut second: u64 = 1;
    match n {
        0 => suma = first,
        1 => {
            suma = second;
        }
        _ => {
            for _i in 1..n {
                suma = first + second;
                first = second;
                second = suma;
            }
        }
    }

    suma
}
