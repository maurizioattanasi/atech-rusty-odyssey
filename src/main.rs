use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input your guess between 1 and 100:");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // rust allows shadowing variables, so we can reuse guess for the parsed value
        let guess: u32 = guess.trim().parse().expect("Not a valid number");

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small! Try again."),
            Ordering::Greater => println!("Too big! Try again."),
            Ordering::Equal => {
                println!("You won! The secret number was {}", secret_number);
                break;
            }
        }
    }

    // if guessed_number == secret_number  {
    //     println!("you won! The secret number was {}", secret_number);
    // }
    // else {
    //     println!("you failed! The secret number was {}", secret_number);
    // }
}
