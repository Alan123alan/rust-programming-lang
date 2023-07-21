use std::io;

fn main() {
    let mut input = String::new();
    println!("Please enter a fahrenheit temperature:");
    let _ = io::stdin().read_line(&mut input);
    let fahrenheit: f64 = input.trim().parse().expect("Please input a number:");
    let celsius: f64 = (fahrenheit - 32.0)*(5.0/9.0);
    println!("Your input was: {fahrenheit} Fahrenheit degrees");
    println!("Your conversion is: {celsius} Celsius degrees");
}
