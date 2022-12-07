use crate::util::{self, log_result};

use super::file_tree::FileTree;

const TOTAL_DISK_SPACE: i32 = 70000000;
const REQUIRED_FREE_DISK_SPACE: i32 = 30000000;

pub fn run() {
    let lines = util::get_puzzle_input_by_delimiter("data/d7.txt", "\n");
    let file_tree = FileTree::from_lines(lines);

    let currently_free_disk_space = TOTAL_DISK_SPACE - file_tree.get_directory_size();
    let additional_required_disk_space = REQUIRED_FREE_DISK_SPACE - currently_free_disk_space;

    let mut directory_sizes = file_tree.get_all_directory_sizes();
    directory_sizes.sort();

    let smallest_directory_freeing_enough_space = directory_sizes
        .into_iter()
        .find(|size| size >= &additional_required_disk_space);

    log_result(smallest_directory_freeing_enough_space, "d7p2");
}
