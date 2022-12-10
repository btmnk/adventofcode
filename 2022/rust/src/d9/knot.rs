use std::collections::HashSet;

use super::{direction::Direction, point::Point};

#[derive(Clone)]
pub struct Knot {
    pub index: i32,
    pub position: Point,
    pub last_move: Option<Point>,
    pub previous_position: Point,
    visited_points: HashSet<Point>,
}

impl Knot {
    pub fn new(index: i32) -> Knot {
        let start = Point { x: 0, y: 0 };
        let mut visited_points = HashSet::new();
        visited_points.insert(start);

        return Knot {
            index,
            position: start,
            last_move: None,
            previous_position: start,
            visited_points: HashSet::new(),
        };
    }

    pub fn move_in_direction(&mut self, direction: Direction) {
        self.previous_position = self.position;

        match direction {
            Direction::Up => self.position.y += 1,
            Direction::Down => self.position.y -= 1,
            Direction::Right => self.position.x += 1,
            Direction::Left => self.position.x -= 1,
        }

        self.last_move = Some(self.position.clone().difference(&self.previous_position));
        self.visited_points.insert(self.position);
    }

    pub fn follow_knot(&mut self, leading_knot: &Knot) {
        if !self.should_move(leading_knot) {
            return;
        }

        self.previous_position = self.position;

        self.position = leading_knot.previous_position;
        self.last_move = Some(Point {
            x: leading_knot.previous_position.x - self.previous_position.x,
            y: leading_knot.previous_position.y - self.previous_position.y,
        });

        self.visited_points.insert(self.position);
    }

    pub fn follow_knot_aligned(&mut self, leading_knot: &Knot) {
        if !self.should_move(leading_knot) {
            return;
        }

        let was_aligned = self.was_aligned_with(leading_knot);

        self.previous_position = self.position;

        if was_aligned {
            if let Some(last_move) = leading_knot.last_move {
                self.position.x += last_move.x;
                self.position.y += last_move.y;
                self.last_move = leading_knot.last_move;

                self.visited_points.insert(self.position);

                return;
            }
        }

        self.position = leading_knot.previous_position;
        self.last_move = Some(Point {
            x: leading_knot.previous_position.x - self.previous_position.x,
            y: leading_knot.previous_position.y - self.previous_position.y,
        });

        self.visited_points.insert(self.position);
    }

    pub fn should_move(&self, leading_knot: &Knot) -> bool {
        let distance = leading_knot.position.difference(&self.position).abs();
        return distance.x > 1 || distance.y > 1;
    }

    pub fn was_aligned_with(&self, leading_knot: &Knot) -> bool {
        let previous_x_distance = leading_knot.previous_position.x - self.position.x;
        let previous_y_distance = leading_knot.previous_position.y - self.position.y;

        return previous_x_distance == 0 && previous_y_distance.abs() == 1
            || previous_x_distance.abs() == 1 && previous_y_distance == 0;
    }

    pub fn get_visited_points(&self) -> &HashSet<Point> {
        return &self.visited_points;
    }
}
