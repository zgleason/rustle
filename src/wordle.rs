pub struct WordleBoard {
    pub word: (char, char, char, char, char),
    pub guess1: (char, char, char, char, char),
    pub guess2: (char, char, char, char, char),
    pub guess3: (char, char, char, char, char),
    pub guess4: (char, char, char, char, char),
    pub guess5: (char, char, char, char, char),
    pub guess_number: i8,
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
}