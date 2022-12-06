use crate::util;

use super::elf;

pub fn run() {
    let calories_per_elf = util::get_puzzle_input_by_delimiter("data/d1.txt", "\n\n");
    let elves = elf::get_elves_from_input(calories_per_elf);

    let mut elf_calories_sorted: Vec<i32> = elves.iter().map(|it| it.get_calories_sum()).collect();
    elf_calories_sorted.sort();

    if let Some(highest_calories) = elf_calories_sorted.last() {
        util::log_result(highest_calories, "d1p1");
    } else {
        util::log_result("Error", "d1p1");
    }
}
