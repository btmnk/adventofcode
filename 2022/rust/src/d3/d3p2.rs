use crate::util::{self, log_result};

use super::{get_item_type_priority::get_item_type_priority, rucksack::Rucksack};

pub fn run() {
    let rucksack_contents = util::get_puzzle_input_by_delimiter("data/d3.txt", "\n");

    // chunk every 3 lines together to form the elf groups
    let rucksack_groups = rucksack_contents.chunks(3);
    let group_badge_priority_sum = rucksack_groups.fold(0, |sum, group| {
        let rucksacks: Vec<Rucksack> = group
            .into_iter()
            .map(|it| Rucksack::from_content(it.to_string()))
            .collect();

        let first_rucksack = rucksacks.first().unwrap();
        let second_rucksack = &rucksacks.clone()[1];
        let third_rucksack = rucksacks.last().unwrap();

        let common_badge_item_type = first_rucksack
            .get_all_items()
            .into_iter()
            .find(|item_type| {
                second_rucksack.has_item_type(item_type) && third_rucksack.has_item_type(item_type)
            });

        if let Some(badge) = common_badge_item_type {
            return sum + get_item_type_priority(badge.clone());
        } else {
            panic!("Could not find group badge!");
        }
    });

    log_result(group_badge_priority_sum, "d3p2");
}
