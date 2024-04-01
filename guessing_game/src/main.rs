use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");     // read_line returns Result: Err or Ok

        // shadow the old guess var, trim newline and parse to u32
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // Note that this would not work without u32 specified, as we need to use
        // the right "parse" function that returns the right type
        println!("You guessed: {guess}");


        // match expression is made up of "arms"
        match guess.cmp(&secret_number) {
            // each arm has a "pattern"
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
