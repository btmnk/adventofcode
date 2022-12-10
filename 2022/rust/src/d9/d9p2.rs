use crate::util::{self, log_result};

use super::{direction::parse_direction, rope::Rope};

pub fn run() {
    let lines = util::get_puzzle_input_by_delimiter("data/d9_example_2.txt", "\n");

    // H + 1-9 where 9 is the tail of the last part means there are a total of 9 parts
    let mut rope = Rope::new(9);

    lines.iter().for_each(|line| {
        let (direction_str, distance_str) = line.split_once(" ").unwrap();

        rope.move_head_in_direction(
            parse_direction(direction_str),
            distance_str.parse::<i32>().unwrap(),
        );
    });

    println!("Visited Positions: {:?}", rope.get_visited_tail_positions());

    log_result(rope.get_visited_tail_positions().len(), "d9p2");
}
