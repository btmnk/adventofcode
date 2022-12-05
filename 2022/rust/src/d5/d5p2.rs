use crate::util::{self, log_result};

use super::{parse_instruction::parse_instruction, storage::Storage};

pub fn run() {
    let instructions = util::get_puzzle_input("data/d5.txt", "\n\n");

    let storage_content = &instructions[0];
    let move_instructions = instructions[1].split("\n").collect::<Vec<&str>>();

    let mut storage = Storage::from_input(storage_content);

    move_instructions
        .into_iter()
        .for_each(|instruction_string| {
            let instruction = parse_instruction(instruction_string);
            storage.move_crates_at_once(instruction.source, instruction.target, instruction.amount);
        });

    log_result(storage.get_top_crates(), "d5p2");
    storage.print_storage();
}
