use crate::util::{self, log_result};

use super::file_tree::FileTree;

pub fn run() {
    let lines = util::get_puzzle_input_by_delimiter("data/d7.txt", "\n");
    let file_tree = FileTree::from_lines(lines);

    let directory_sizes_sum = file_tree
        .get_all_directory_sizes()
        .into_iter()
        .filter(|it| it < &100000)
        .fold(0, |sum, current| sum + current);

    log_result(directory_sizes_sum, "d7p1");
}
