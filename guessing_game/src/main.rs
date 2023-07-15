use std::io;
use rand::Rng;
use std::cmp::Ordering;
use std::any::type_name;

fn type_of<T>(_: &T) ->&'static str{
    type_name::<T>()
}

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    //println!("The secret number is: {secret_number}");
    println!("Guess the number!");
    loop{
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");
        //.trim() method necesary since guess will be whatever the user types + \n
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        //println!("You guessed: {guess}");
        //println!("Guess is of type {}",type_of(&guess));
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
