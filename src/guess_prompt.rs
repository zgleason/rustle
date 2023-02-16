// Use read guess to get a 5 letter word from the user

use std::io;

pub enum GuessReturn {
    GuessError,
    GuessResult([char;5])
}

pub fn read_guess() -> GuessReturn {
    let mut parsed_guess = ['-';5];

    let mut guess = String::new();

    println!("\nGUESS: ");

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess = guess.trim();

    if guess.chars().count() == 5 {
        parsed_guess[0] = guess.chars().nth(0).expect("BAD WORD");
        parsed_guess[1] = guess.chars().nth(1).expect("BAD WORD");
        parsed_guess[2] = guess.chars().nth(2).expect("BAD WORD");
        parsed_guess[3] = guess.chars().nth(3).expect("BAD WORD");
        parsed_guess[4] = guess.chars().nth(4).expect("BAD WORD");

        return GuessReturn::GuessResult(parsed_guess)
    }

    GuessReturn::GuessError
}