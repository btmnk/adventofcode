use std::collections::HashSet;

use super::{direction::Direction, point::Point};

#[derive(Clone)]
pub struct RopePart {
    pub current_head_position: Point,
    pub tail_position: Point,
    previous_head_position: Point,
    visited_tail_points: HashSet<Point>,
}

impl RopePart {
    pub fn new() -> RopePart {
        return RopePart {
            current_head_position: Point { x: 0, y: 0 },
            previous_head_position: Point { x: 0, y: 0 },
            tail_position: Point { x: 0, y: 0 },
            visited_tail_points: HashSet::new(),
        };
    }

    pub fn move_head_in_direction(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.move_head_to_point(
                self.current_head_position.x,
                self.current_head_position.y + 1,
            ),
            Direction::Down => self.move_head_to_point(
                self.current_head_position.x,
                self.current_head_position.y - 1,
            ),
            Direction::Left => self.move_head_to_point(
                self.current_head_position.x - 1,
                self.current_head_position.y,
            ),
            Direction::Right => self.move_head_to_point(
                self.current_head_position.x + 1,
                self.current_head_position.y,
            ),
        }
    }

    pub fn move_head_to_point(&mut self, x: i32, y: i32) {
        self.current_head_position = Point { x, y };

        let distance = Self::get_distance(&Point { x, y }, &self.tail_position);
        let should_move_tail = distance.x.abs() > 0 || distance.y.abs() > 0;

        // println!(
        //     "Move Head {}:{} -> {}:{} | Tail: {}/{} ({}/{}: {})",
        //     self.previous_head_position.x,
        //     self.previous_head_position.y,
        //     x,
        //     y,
        //     self.tail_position.x,
        //     self.tail_position.y,
        //     distance.x,
        //     distance.y,
        //     should_move_tail
        // );

        if should_move_tail {
            self.tail_position = self.previous_head_position;
        }

        self.visited_tail_points.insert(self.tail_position);

        self.previous_head_position = self.current_head_position;
    }

    pub fn get_distance(from: &Point, to: &Point) -> Point {
        let mut x_distance = from.x - to.x;
        let mut y_distance = from.y - to.y;

        if x_distance < 0 {
            x_distance += 1
        } else if x_distance > 0 {
            x_distance -= 1
        };

        if y_distance < 0 {
            y_distance += 1
        } else if y_distance > 0 {
            y_distance -= 1
        };

        return Point {
            x: x_distance,
            y: y_distance,
        };
    }

    pub fn get_visited_points(&self) -> &HashSet<Point> {
        return &self.visited_tail_points;
    }
}
