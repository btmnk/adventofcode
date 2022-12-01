use crate::get_puzzle_input::get_puzzle_input_lines;

use super::elf;

pub fn run() {
    let calories_per_elf = get_puzzle_input_lines("data/d1.txt", "\n\n");
    let elves = elf::get_elves_from_input(calories_per_elf);

    let mut elf_calories_sorted: Vec<i32> = elves.iter().map(|it| it.get_calories_sum()).collect();
    elf_calories_sorted.sort();
    elf_calories_sorted.reverse();

    let top_3_calories_sum = elf_calories_sorted[0..3]
        .iter()
        .fold(0, |sum, current| sum + current);

    print!("\n Top 3 calories sum: {} \n", top_3_calories_sum);
}
