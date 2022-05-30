use rand::Rng;
use std::io;

fn main() {
    println!("Guess a number between 1 and 10!");

    println!("Please input your guess.");

    let mut guess = String::new();
    let random_num: _ = rand::thread_rng().gen_range(1..10);

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

        let input: u32 = guess
            .trim()
            .parse()
            .expect("aaa");

        if input == random_num {
            println!("You guessed correctly. Number is {}", random_num);
        } else {
            println!("You guessed incorrectly. Number is {}", random_num);
        }
}
