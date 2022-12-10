use std::collections::HashSet;

use super::{direction::Direction, knot::Knot, point::Point};

pub struct Rope {
    pub knots: Vec<Knot>,
    pub debug: bool,
}

impl Rope {
    pub fn new(knots: i32) -> Rope {
        let mut knot_points: Vec<Knot> = vec![];

        for index in 0..knots {
            knot_points.push(Knot::new(index));
        }

        return Rope {
            knots: knot_points,
            debug: false,
        };
    }

    pub fn move_head_in_direction(&mut self, direction: Direction, distance: i32) {
        if self.debug {
            println!("\n===== Move {} steps {:?}\n", distance, direction);
        }

        for _ in 0..distance {
            self.knots
                .clone()
                .iter()
                .enumerate()
                .for_each(|(index, _)| {
                    if index == 0 {
                        self.knots[index].move_in_direction(direction);
                    } else {
                        let leading_knot = self.knots[index - 1].clone();
                        self.knots[index].follow_knot(&leading_knot);
                    }
                });

            if self.debug {
                self.render_rope();
            }
        }
    }

    pub fn get_visited_tail_positions(&self) -> &HashSet<Point> {
        return self.knots.last().unwrap().get_visited_points();
    }

    pub fn render_rope(&self) {
        let mut head_positions: Vec<Point> = vec![];

        self.knots.iter().for_each(|knot| {
            head_positions.push(knot.position);
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
                } else if current_point.x == 0 && current_point.y == 0 {
                    print!("s");
                } else {
                    print!(".");
                }
            }
        }

        println!("");
    }
}
