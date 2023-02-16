use super::wordle::WordleBoard;
use colored::Colorize;

pub fn print_board(board: &WordleBoard) {

    // Send clear character to clear the terminal
    print!("{}[2J", 27 as char);

    println!("\n\n\n RUST-LE\n");

    if board.guess_number > 0 {
        print_guess(board.guess1, board.word);
    }
    else {
        print_hidden();
    }

    if board.guess_number > 1 {
        print_guess(board.guess2, board.word);
    }
    else {
        print_hidden();
    }

    if board.guess_number > 2 {
        print_guess(board.guess3, board.word);
    }
    else {
        print_hidden();
    }

    if board.guess_number > 3 {
        print_guess(board.guess4, board.word);
    }
    else {
        print_hidden();
    }

    if board.guess_number > 4 {
        print_guess(board.guess5, board.word);
    }
    else {
        print_hidden();
    }

}

fn print_guess(
    guess: (char, char, char, char, char),
    actual: (char, char, char, char, char)) {

    println!("{} {} {} {} {}", 
            get_colored_letter(&guess.0, &actual.0, &actual),
            get_colored_letter(&guess.1, &actual.1, &actual),
            get_colored_letter(&guess.2, &actual.2, &actual),
            get_colored_letter(&guess.3, &actual.3, &actual),
            get_colored_letter(&guess.4, &actual.4, &actual),
        );
}

fn get_colored_letter(
    guess: &char, 
    actual: &char, 
    word: &(char, char, char, char, char)) -> colored::ColoredString {

    if guess == actual {
        return guess.to_string().green().bold();
    }

    guess.to_string().truecolor(50, 50, 50)
}

fn print_hidden() {
    println!("▢ ▢ ▢ ▢ ▢")
}