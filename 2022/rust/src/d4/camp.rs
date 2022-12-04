use range_set::range_compare::RangeIntersect;
use std::ops::RangeInclusive;

use crate::util::range_from_string;

pub struct Camp {
    pub first_section: RangeInclusive<i32>,
    pub second_section: RangeInclusive<i32>,
}

impl Camp {
    pub fn from_input(input: &str) -> Camp {
        let assignments: Vec<&str> = input.split(",").collect();

        let first_section = range_from_string(assignments[0]);
        let second_section = range_from_string(assignments[1]);

        return Camp {
            first_section,
            second_section,
        };
    }

    pub fn sections_fully_overlap(&self) -> bool {
        let range_intersect_opt =
            RangeIntersect::compare(&self.first_section, &self.second_section);

        if let Some(intersect) = range_intersect_opt {
            return match intersect {
                RangeIntersect::OverlapsLeft | RangeIntersect::OverlapsRight => false,
                _ => true,
            };
        } else {
            return false;
        }
    }

    pub fn sections_partially_or_fully_overlap(&self) -> bool {
        let range_intersect_opt =
            RangeIntersect::compare(&self.first_section, &self.second_section);

        if let Some(_intersect) = range_intersect_opt {
            return true;
        } else {
            return false;
        }
    }
}
