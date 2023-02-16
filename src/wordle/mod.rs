mod wordle_printer;

pub struct WordleBoard {
    word: (char, char, char, char, char),
    guess1: (char, char, char, char, char),
    guess2: (char, char, char, char, char),
    guess3: (char, char, char, char, char),
    guess4: (char, char, char, char, char),
    guess5: (char, char, char, char, char),
    guess_number: i8,
}

impl WordleBoard {
    pub fn new() -> Self {
        Self {
            word: ('h', 'e', 'l', 'l', 'o'),
            guess1: ('-', '-', '-', '-', '-'),
            guess2: ('-', '-', '-', '-', '-'),
            guess3: ('-', '-', '-', '-', '-'),
            guess4: ('-', '-', '-', '-', '-'),
            guess5: ('-', '-', '-', '-', '-'),
            guess_number: 0
        }
    }

    pub fn is_over(&self) -> bool {
        if self.guess_number >= 5 {
            return true
        }

        false
    }

    pub fn guess(&mut self, guess: (char, char, char, char, char)) {
        match self.guess_number {
            0 => self.guess1 = guess,
            1 => self.guess2 = guess,
            2 => self.guess3 = guess,
            3 => self.guess4 = guess,
            4 => self.guess5 = guess,
            _ => {}
        }

        self.guess_number = self.guess_number + 1;
    }

    pub fn print_board(&self) {
        wordle_printer::print_board(&self);
    }

}