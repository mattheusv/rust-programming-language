use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let scret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", scret_number);

    loop {
        println!("Please input your guess;");

        let mut guess = String::new();

        // references in rust is imutable by default
        // so &mut is required to pass a mutable reference to read_line
        // https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html#storing-values-with-variables
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read lone");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&scret_number) {
            Ordering::Less => println!("To small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
