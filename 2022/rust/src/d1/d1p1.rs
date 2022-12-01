use crate::get_puzzle_input::get_puzzle_input_lines;

use super::elf;

pub fn run() {
    let calories_per_elf = get_puzzle_input_lines("data/d1.txt", "\n\n");
    let elves = elf::get_elves_from_input(calories_per_elf);

    let mut elf_calories_sorted: Vec<i32> = elves.iter().map(|it| it.get_calories_sum()).collect();
    elf_calories_sorted.sort();

    if let Some(highest_calories) = elf_calories_sorted.last() {
        print!("\n Highest calories: {} \n", highest_calories);
    } else {
        print!("\n Could not get first result from sorted array \n");
    }
}
