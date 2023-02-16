mod wordle;
mod guess_prompt;

use wordle::WordleBoard;
use guess_prompt::GuessReturn;
use colored::Colorize;
use std::io::{stdin, Read};

fn main() {
    let mut board = WordleBoard::new();

    while !board.is_over() {
        game_loop(&mut board);
    }

    end_game(&board)
}

fn game_loop(board: &mut WordleBoard) {
    board.print_board();

    let guess_return = guess_prompt::read_guess();

    match guess_return {
        GuessReturn::GuessResult(guess_result) => {
            board.guess(guess_result);
        },
        GuessReturn::GuessError => {}
    }
}

fn end_game(board: &WordleBoard) {
    board.print_board();
    println!("{}", "\nGAME OVER".red());

    // wait for enter to exit the game
    stdin().read(&mut [0]).unwrap();
}