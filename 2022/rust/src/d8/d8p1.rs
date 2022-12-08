use crate::util::{self, log_result};

use super::forest::Forest;

pub fn run() {
    let lines = util::get_puzzle_input_by_delimiter("data/d8.txt", "\n");
    let forest = Forest::from_lines(lines);
    log_result(forest.get_all_visible_trees(), "d8p1");
}
