pub struct WordleBoard {
    pub word: [char;5],
    pub guesses: [[char;5];5],
    pub guess_number: i8,
}

impl WordleBoard {
    pub fn new() -> Self {
        Self {
            word: ['h', 'e', 'l', 'l', 'o'],
            guesses: [['-'; 5]; 5],
            guess_number: 0
        }
    }

    pub fn is_over(&self) -> bool {
        if self.guess_number >= 5 {
            return true
        }

        false
    }

    pub fn guess(&mut self, guess: [char;5]) {
        if self.guess_number < 5 {
            self.guesses[self.guess_number as usize] = guess;
            self.guess_number = self.guess_number + 1;
        }
    }
}