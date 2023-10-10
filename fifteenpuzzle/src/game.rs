use rand::seq::SliceRandom;
use std::error;
use std::time::Instant;

pub type GameResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Clone, PartialEq, Debug)]
pub enum GameState {
    STARTED,
    PLAYING,
    PAUSED,
    FINISHED,
    EXIT,
}

#[derive(Clone)]
pub struct GameInfo {
    pub numbers: [u16; 16],
    pub moves: u32,
    pub seconds: u32,
    pub game_state: GameState,
    pub start_time: Instant,
    pub current_time: u32,
}

impl Default for GameInfo {
    fn default() -> Self {
        Self {
            numbers: shuffle(),
            moves: 0,
            seconds: 0,
            game_state: GameState::STARTED,
            start_time: Instant::now(),
            current_time: 0,
        }
    }
}

impl GameInfo {
    /// Constructs a new instance of [`GameInfo`].
    pub fn new() -> Self {
        Self::default()
    }

    // Generates puzzle
    pub fn generate_puzzle(&mut self, event: GameState) {
        if event == GameState::STARTED {
            shuffle();
        } else {
        }
    }

    //swap two index
    fn swap(&mut self, in1: u16, in2: u16) {
        let index1 = in1 as usize;
        let index2 = in2 as usize;
        let temp = self.numbers[index1];
        self.numbers[index1] = self.numbers[index2];
        self.numbers[index2] = temp;
    }

    // Handles key movement
    pub fn move_key(&mut self, ch: char) -> bool {
        let empty = self.find_empty();
        let mut _arr = self.numbers;
        let zero_row = (empty as f64 / 4 as f64).floor() as u16;
        let zero_col = empty % 4;
        match ch {
            'w' => {
                if zero_row > 0 {
                    let curr = (zero_row) * 4 + zero_col;
                    let swapped = (zero_row - 1) * 4 + zero_col;
                    self.swap(curr, swapped);
                    return true;
                }
                return false;
            }
            'a' => {
                if zero_col > 0 {
                    let curr = (zero_row) * 4 + zero_col;
                    let swapped = (zero_row) * 4 + zero_col - 1;
                    self.swap(curr, swapped);
                    return true;
                }
                return false;
            }
            's' => {
                if zero_row < 3 {
                    let curr = (zero_row) * 4 + zero_col;
                    let swapped = (zero_row + 1) * 4 + zero_col;
                    self.swap(curr, swapped);
                    return true;
                }
                return false;
            }
            'd' => {
                if zero_col < 3 {
                    let curr = (zero_row) * 4 + zero_col;
                    let swapped = (zero_row) * 4 + zero_col + 1;
                    self.swap(curr, swapped);
                    return true;
                }
                return false;
            }
            _ => return false,
        }
    }

    // Check if game is won
    fn check_win(&mut self, arr: [u16; 16]) -> bool {
        let result = (0..16).into_iter().all(|x| {
            if x == 15 {
                arr[x as usize] == 0
            } else {
                x + 1 == arr[x as usize]
            }
        });

        result
    }

    // Updates game info
    pub fn handle_game_change(&mut self, key_moved: bool, ch: char) {
        if key_moved && self.game_state != GameState::FINISHED {
            self.moves += 1;
        }

        match self.game_state {
            GameState::STARTED => {
                if ['w', 'a', 's', 'd'].contains(&ch) {
                    self.game_state = GameState::PLAYING
                } else {
                    self.game_state = GameState::STARTED
                }
            }
            GameState::PLAYING => {
                let is_done = self.check_win(self.numbers);

                if ch == 'p' {
                    self.game_state = GameState::PAUSED
                } else if is_done {
                    self.game_state = GameState::FINISHED
                } else {
                    self.game_state = GameState::PLAYING
                }
            }
            GameState::PAUSED => self.game_state = GameState::PLAYING,
            GameState::FINISHED => {
                if ch == 'r' {
                    self.game_state = GameState::STARTED
                } else {
                    self.game_state = GameState::FINISHED
                }
            }
            _ => {}
        }
        // game_data.base_time = update_elapsed_time(&game_data, &next_game_state);
    }

    pub fn exit(&mut self) {
        self.game_state = GameState::EXIT;
    }

    pub fn tick(&self) {}

    pub fn find_empty(&mut self) -> u16 {
        for (i, &value) in self.numbers.iter().enumerate() {
            if value == 0 {
                return i as u16;
            }
        }
        return 0;
    }
}

// Shuffles the array
pub fn shuffle() -> [u16; 16] {
    let mut arr = [0; 16];
    let mut rng = rand::thread_rng();
    (0..16).into_iter().enumerate().for_each(|args| {
        let (index, number) = args;
        arr[index] = number;
    });
    loop {
        arr.shuffle(&mut rng);
        if is_solvable(&arr) {
            break;
        }
    }

    return arr;
}

// Checks if the puzzle is solvable
pub fn is_solvable(puzzle: &[u16]) -> bool {
    let mut parity = 0;
    let grid_width = 4;
    let mut row = 0;
    let mut blank_row = 0;

    for (i, &value) in puzzle.iter().enumerate() {
        if i % grid_width == 0 {
            row += 1;
        }
        if value == 0 {
            blank_row = row;
            continue;
        }
        for j in i + 1..puzzle.len() {
            if value > puzzle[j] && puzzle[j] != 0 {
                parity += 1;
            }
        }
    }
    if grid_width % 2 == 0 {
        if blank_row % 2 == 0 {
            return parity % 2 == 0;
        } else {
            return parity % 2 != 0;
        }
    } else {
        return parity % 2 == 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_game_info() {
        let game_info = GameInfo::default();

        assert_eq!(game_info.moves, 0);
    }

    #[test]
    fn test_shuffle() {
        let arr = shuffle();
        assert_eq!(arr.len(), 16);
    }

    #[test]
    fn is_solvable_should_correct() {
        {
            let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0];
            let is_solvable = is_solvable(&arr);
            assert_eq!(is_solvable, true);
        }
        {
            let arr = [7, 10, 11, 1, 0, 9, 3, 4, 5, 8, 13, 2, 14, 6, 12, 15];
            let is_solvable = is_solvable(&arr);
            assert_eq!(is_solvable, true);
        }
        {
            let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 15, 14];
            let is_solvable = is_solvable(&arr);
            assert_eq!(is_solvable, false);
        }
    }

    #[test]
    fn invalid_input() {
        let mut game = GameInfo::new();

        //invalid char
        assert_eq!(game.move_key('x'), false);
        assert_eq!(game.move_key(' '), false);

        // Starting state
        game.handle_game_change(false, 'x');
        assert_eq!(game.game_state, GameState::STARTED);

        // Playing state
        game.handle_game_change(true, 'w');
        assert_eq!(game.game_state, GameState::PLAYING);

        // Exit state
        game.exit();
        assert_eq!(game.game_state, GameState::EXIT);
    }

    #[test]
    fn win_condition() {
        let mut game = GameInfo::new();
        game.numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0];
        game.move_key('w');
        game.handle_game_change(true, 'w');
        game.move_key('s');
        game.handle_game_change(true, 's');
        assert_eq!(game.check_win(game.numbers), true);
        assert_eq!(game.game_state, GameState::FINISHED);
    }
}
