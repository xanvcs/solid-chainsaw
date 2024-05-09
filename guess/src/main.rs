use rand::Rng;          // rng module
use std::io::BufRead;   // enables reading from stdin

fn main() {
    // makes rng mutable since variables are immutable by default
    // thread_rng makes it a thread safe rng object
    let mut rng = rand::thread_rng();
    // range from inclusive to not inclusive
    let random = rng.gen_range(1..101);

    println!("Guess a number between 1 and 100");
    for line in std::io::stdin().lock().lines() {
        // ensures that a valid input is read or it will skip the if statement
        let parsed = line.ok().as_deref().map(str::parse::<i64>);
        // matches the input
        if let Some(Ok(guess)) = parsed {
            match guess {
                _ if guess < random => println!("Too low"),
                _ if guess > random => println!("Too high"),
                _ => {
                    println!("That's right");
                    break;
                }
            }
        }
    }
}
