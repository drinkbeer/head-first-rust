#![allow(unused_variables)]
#![allow(dead_code)]

// fn main() {
//     run_guess_game();
// }
pub mod guess_game_mod {
    use std::cmp::Ordering;
    use rand::Rng;

    pub fn run_guess_game() {
        println!("Guess the number between 1 and 100 (inclusive)!");
        let target = rand::thread_rng().gen_range(1..=100);

        loop {
            let mut guess = String::new();
            std::io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");

            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please input a number! {}", guess);
                    continue;
                }
            };
            println!("You guessed: {}", guess);

            match guess.cmp(&target) {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal => {
                    println!("You win!");
                    break;
                },
            }
        }
    }
}

