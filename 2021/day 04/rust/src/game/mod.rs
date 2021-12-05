use std::fs;

pub mod board;

pub struct Game {
    pub boards: Vec<board::Board>,
}

impl Game {
    pub fn play_round(&mut self, field_value: &str) {
        for board in &mut self.boards {
            if !board.is_bingo() {
                board.mark_fields(field_value);
            }
        }
    }

    pub fn get_winners(&self) -> Vec<board::Board> {
        let mut winners: Vec<board::Board> = self
            .boards
            .clone()
            .into_iter()
            .filter(|board| board.is_bingo())
            .collect();
        winners.sort_by(|left, right| left.rounds_until_bingo.cmp(&right.rounds_until_bingo));
        return winners;
    }
}

fn get_boards_from_input(file: &str) -> Vec<String> {
    let contents = fs::read_to_string(file).expect("could not read file boards.txt");
    return contents.split("\n\n").map(str::to_string).collect();
}

pub fn from(file: &str) -> Game {
    let board_strings = get_boards_from_input(file);
    let boards: Vec<board::Board> = board_strings
        .into_iter()
        .map(board::create_board_from_input)
        .collect();
    return Game { boards };
}
