use crate::util::{self, log_result};

use super::{direction::parse_direction, rope::Rope};

pub fn run() {
    let lines = util::get_puzzle_input_by_delimiter("data/d9.txt", "\n");

    // 1 part equals 1 head and 1 tail
    let mut rope = Rope::new(1);

    lines.iter().for_each(|line| {
        let (direction_str, distance_str) = line.split_once(" ").unwrap();

        rope.move_head_in_direction(
            parse_direction(direction_str),
            distance_str.parse::<i32>().unwrap(),
        );
    });

    log_result(
        rope.rope_parts.last().unwrap().get_visited_points().len(),
        "d9p1",
    );
}
