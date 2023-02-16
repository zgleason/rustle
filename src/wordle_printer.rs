use super::wordle::WordleBoard;

pub fn print_board(board: &WordleBoard) {

    // Send clear character to clear the terminal
    print!("{}[2J", 27 as char);

    println!("\n\n\n RUST-LE\n");

    for (i, guess) in board.guesses.iter().enumerate() {
        if board.guess_number as usize > i {
            print_guess(guess);
        }
        else {
            print_hidden();
        }
    }
}

fn print_guess(
    guess: &[char;5]) {

    println!("{} {} {} {} {}", 
            guess[0],
            guess[1],
            guess[2],
            guess[3],
            guess[4]
        );
}

fn print_hidden() {
    println!("▢ ▢ ▢ ▢ ▢")
}