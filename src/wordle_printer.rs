use super::wordle::WordleBoard;
use super::wordle::LetterState;

use colored::Colorize;

pub fn print_board(board: &WordleBoard) {

    // Send clear character to clear the terminal
    print!("{}[2J", 27 as char);

    println!("\n\n\n RUST-LE\n");

    for (i, guess) in board.guesses.iter().enumerate() {
        if board.guess_number as usize > i {
            print_guess(guess, &board);
        }
        else {
            print_hidden();
        }
    }
}

fn print_guess(
    guess: &[char;5],
    board: &WordleBoard) {

    let letter_states = board.get_state(&guess);

    let mut formatted_string = String::new();

    for (i, state) in letter_states.iter().enumerate() {
        formatted_string.push_str(&get_colored_letter(&guess[i], state));
        formatted_string.push_str(" ");
    }

    println!("{formatted_string}");
}

fn get_colored_letter(letter: &char, state: &LetterState) -> String {
    match state {
        LetterState::Right => letter.to_string().green().to_string(),
        LetterState::Wrong => letter.to_string().truecolor(50, 50, 50).to_string(),
        LetterState::WrongLocation => letter.to_string().yellow().to_string()
    }
    
}

fn print_hidden() {
    println!("▢ ▢ ▢ ▢ ▢")
}