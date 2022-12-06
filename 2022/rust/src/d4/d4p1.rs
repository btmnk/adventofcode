use crate::util::{self, log_result};

use super::camp::Camp;

pub fn run() {
    let assignments = util::get_puzzle_input_by_delimiter("data/d4.txt", "\n");
    let fully_overlapping_ranges: i32 = assignments.iter().fold(0, |sum, assignment| {
        let camp = Camp::from_input(assignment);

        if camp.sections_fully_overlap() {
            return sum + 1;
        }

        return sum;
    });

    log_result(fully_overlapping_ranges, "d4p1");
}
