use crate::ventline;
use std::collections::HashMap;

pub fn count_dangerous_points(vent_lines: Vec<ventline::VentLine>, with_diagonals: bool) -> i32 {
    let mut points: HashMap<ventline::Point, i32> = HashMap::new();

    vent_lines
        .into_iter()
        .map(|line| line.get_all_points(with_diagonals))
        .flatten()
        .for_each(|point| {
            if let Some(target) = points.get_mut(&point) {
                *target += 1;
            } else {
                points.insert(point, 1);
            }
        });

    let sum: i32 = points.iter().fold(0 as i32, |sum, current| {
        let (_, occurences) = current;
        return if occurences > &1 { sum + 1 } else { sum };
    });

    return sum;
}
