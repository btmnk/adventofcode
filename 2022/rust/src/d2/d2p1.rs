use crate::util::{get_puzzle_input, log_result};

use super::rock_paper_scissors::{get_game_result, get_shape_by_code, get_shape_score};

pub fn run() {
    let lines = get_puzzle_input("data/d2.txt", "\n");
    let total_score = lines.iter().fold(0, |total, current| {
        let codes: Vec<&str> = current.split(" ").collect();
        let opponent_shape = get_shape_by_code(codes.first().unwrap());
        let my_shape = get_shape_by_code(codes.last().unwrap());

        // get_game_result expects first value to be our shape
        let game_score = get_game_result(my_shape, opponent_shape);

        total + game_score + get_shape_score(my_shape)
    });

    log_result(total_score, "d2p1");
}
