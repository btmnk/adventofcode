use crate::util::{self, log_result};

use super::{get_item_type_priority::get_item_type_priority, rucksack::Rucksack};

pub fn run() {
    let rucksack_contents = util::get_puzzle_input("data/d3.txt", "\n");
    let priority_sum = rucksack_contents.iter().fold(0, |sum, content| {
        let rucksack = Rucksack::from_content(content.clone());
        let common_type_opt = rucksack.get_common_type();

        if let Some(common_type) = common_type_opt {
            return sum + get_item_type_priority(common_type);
        } else {
            return sum;
        }
    });

    log_result(priority_sum, "d3p1");
}
