use std::collections::HashSet;

use super::{direction::Direction, point::Point, rope_part::RopePart};

pub struct Rope {
    pub rope_parts: Vec<RopePart>,
}

impl Rope {
    pub fn new(parts: i32) -> Rope {
        let mut rope_parts: Vec<RopePart> = vec![];

        for _ in 0..parts {
            rope_parts.push(RopePart::new())
        }

        return Rope { rope_parts };
    }

    pub fn move_head_in_direction(&mut self, direction: Direction, distance: i32) {
        println!("\n===== Move {} steps {:?}\n", distance, direction);

        for _ in 0..distance {
            self.rope_parts
                .clone()
                .iter()
                .enumerate()
                .for_each(|(index, _)| {
                    if index == 0 {
                        self.rope_parts[index].move_head_in_direction(direction);
                    } else {
                        let previous_rope_part = self.rope_parts[index - 1].clone();

                        self.rope_parts[index].move_head_to_point(
                            previous_rope_part.tail_position.x,
                            previous_rope_part.tail_position.y,
                        )
                    }
                })
        }

        self.render_rope();
    }

    pub fn get_visited_tail_positions(&self) -> &HashSet<Point> {
        return self.rope_parts.last().unwrap().get_visited_points();
    }

    pub fn render_rope(&self) {
        let mut head_positions: Vec<Point> = vec![];

        self.rope_parts.iter().for_each(|rope_part| {
            head_positions.push(rope_part.current_head_position);
        });

        let visited_points = self.get_visited_tail_positions();

        let mut sorted_points = head_positions.clone();

        sorted_points.sort_by_key(|point| point.x);
        let smallest_x = sorted_points.first().unwrap().x;
        let largest_x = sorted_points.last().unwrap().x;

        sorted_points.sort_by_key(|point| point.y);
        let smallest_y = sorted_points.first().unwrap().y;
        let largest_y = sorted_points.last().unwrap().y;

        let render_x_start = smallest_x - 30;
        let render_x_end = largest_x + 30;

        let render_y_start = smallest_y - 10;
        let render_y_end = largest_y + 10;

        for y in (render_y_start..=render_y_end).rev() {
            print!("\n");

            for x in render_x_start..=render_x_end {
                let current_point = Point { x, y };
                let position_index_opt =
                    head_positions.iter().position(|pos| pos == &current_point);

                if let Some(position_index) = position_index_opt {
                    print!("{}", position_index);
                } else if visited_points.contains(&current_point) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
        }

        println!("");
    }
}
