use crate::util::{get_puzzle_input_by_delimiter, log_result};

use super::rock_paper_scissors::{
    get_game_result, get_shape_by_code, get_shape_by_game_result, get_shape_score,
};

pub fn run() {
    let lines = get_puzzle_input_by_delimiter("data/d2.txt", "\n");
    let total_score = lines.iter().fold(0, |total, current| {
        let codes: Vec<&str> = current.split(" ").collect();

        let attack_shape = get_shape_by_code(codes.first().unwrap());
        let target_result = codes.last().unwrap();

        let target_shape = get_shape_by_game_result(attack_shape, target_result);

        // get_game_result expects first value to be our shape
        let game_score = get_game_result(target_shape, attack_shape);

        total + game_score + get_shape_score(target_shape)
    });

    log_result(total_score, "d2p2");
}
