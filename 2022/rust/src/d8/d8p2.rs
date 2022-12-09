use crate::util::{self, log_result};

use super::forest::Forest;

pub fn run() {
    let lines = util::get_puzzle_input_by_delimiter("data/d8.txt", "\n");
    let forest = Forest::from_lines(lines);
    let mut tree_scores: Vec<i32> = forest.get_tree_scores().into_values().collect();
    tree_scores.sort();

    log_result(tree_scores.last(), "d8p2");
}
