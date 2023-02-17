use super::wordle::WordleBoard;
use super::wordle_printer;
use super::guess_prompt;
use super::random_word;

use colored::Colorize;
use std::io::{stdin, Read};

pub fn play() {
    let mut board = WordleBoard::new(random_word::get_random_word());

    while !board.is_over() {
        game_loop(&mut board);
    }

    end_game(&board);
}

pub fn game_loop(board: &mut WordleBoard) {
    wordle_printer::print_board(board);

    let guess_return = guess_prompt::read_guess();

    match guess_return {
        guess_prompt::GuessReturn::GuessResult(guess_result) => {
            board.guess(guess_result);
        },
        guess_prompt::GuessReturn::GuessError => {}
    }
}

pub fn end_game(board: &WordleBoard) {
    wordle_printer::print_board(board);

    match board.has_won() {
        true => println!("{}", "\n YOU WIN".green()),
        false => {
            println!("\n{} {} {} {} {}", 
                board.word[0], 
                board.word[1], 
                board.word[2], 
                board.word[3], 
                board.word[4]);
            println!("{}", "\nGAME OVER".red());
        }
    }
    

    // wait for enter to exit the game
    stdin().read(&mut [0]).unwrap();
}
