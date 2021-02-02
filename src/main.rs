use std::io;

fn main() {
    println!("Enter nth Fibonacci number to generate");

    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read number");

    let number: i64 = match number.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("{} is not a number", number.trim()),
    };

    if 2 >= number {
        println!("Fibonacci number #{} is 1", number);
    } else {
        let mut a: i64 = 1;
        let mut b: i64 = 1;
        let mut a_temp: i64;
        let mut counter: i64 = 2;
        while number != counter {
            a_temp = a;
            a = b;
            b = fibonacci(a_temp, b);
            counter += 1;
        }
        println!("Fibonacci number #{} is {}", number, b);
    }
}

fn fibonacci(a: i64, b: i64) -> i64 {
    a + b
}
