pub struct WordleBoard {
    pub word: [char;5],
    pub guesses: [[char;5];5],
    pub guess_number: i8,
}

#[derive(Copy, Clone)]
pub enum LetterState {
    Right,
    Wrong,
    WrongLocation
}

impl WordleBoard {
    pub fn new(puzzle_word: [char;5]) -> Self {
        Self {
            word: puzzle_word,
            guesses: [['-'; 5]; 5],
            guess_number: 0
        }
    }

    pub fn is_over(&self) -> bool {
        if self.guess_number >= 5 || self.has_won() {
            return true
        }

        false
    }

    pub fn has_won(&self) -> bool {
        // Check bounds of array and get the previous guess
        let guess_index = self.guess_number as usize;

        if self.guess_number > 0 && self.guess_number < 6 {
            if self.guesses[guess_index - 1] == self.word {
                return true
            }
        }

        false
    }

    pub fn guess(&mut self, guess: [char;5]) {
        if self.guess_number < 5 {
            self.guesses[self.guess_number as usize] = guess;
            self.guess_number = self.guess_number + 1;
        }
    }

    pub fn get_state(&self, word: &[char;5]) -> [LetterState;5] {

        let mut state = [LetterState::Wrong; 5];

        for i in 0..state.len() {
            if word[i] == self.word[i] {
                state[i] = LetterState::Right;
            }
            // Don't show more wrong location letters than duplicates in the word
            else if self.get_duplicate_count(&word[i]) > self.get_prior_duplicates(&word, i) {
                state[i] = LetterState::WrongLocation;
            }
        }

        state
    }

    // Gets the number of duplicates before a letter in a word
    fn get_prior_duplicates(&self, word: &[char;5], index: usize) -> u8 {
        let mut count = 0;

        for i in 0..index {
            if word[i] == word[index] {
                count = count + 1;
            }
        }

        count
    }

    // Gets the number of times "letter" appears in "word"
    fn get_duplicate_count(&self, letter: &char) -> u8 {
        let mut count = 0;

        for word_letter in self.word {
            if &word_letter == letter {
                count = count + 1;
            }
        }

        count
    }
}

