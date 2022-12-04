use crate::util::{self, log_result};

use super::camp::Camp;

pub fn run() {
    let assignments = util::get_puzzle_input("data/d4.txt", "\n");
    let partially_or_fully_overlapping_ranges: i32 =
        assignments.iter().fold(0, |sum, assignment| {
            let camp = Camp::from_input(assignment);

            if camp.sections_partially_or_fully_overlap() {
                return sum + 1;
            }

            return sum;
        });

    log_result(partially_or_fully_overlapping_ranges, "d4p1");
}
