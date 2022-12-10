use std::collections::HashSet;

use super::{direction::Direction, point::Point};

#[derive(Clone)]
pub struct Knot {
    pub index: i32,
    pub position: Point,
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
            visited_points,
        };
    }

    pub fn move_in_direction(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.position.y += 1,
            Direction::Down => self.position.y -= 1,
            Direction::Right => self.position.x += 1,
            Direction::Left => self.position.x -= 1,
        }

        self.visited_points.insert(self.position);
    }

    pub fn follow_knot(&mut self, leading_knot: &Knot) {
        if let Some(adjacent_point) = self.get_adjacent_point(leading_knot) {
            self.position = adjacent_point;
        } else {
            // nothing to do, already adjacent
            return;
        }

        self.visited_points.insert(self.position);
    }

    fn get_adjacent_point(&self, leading_knot: &Knot) -> Option<Point> {
        let distance = self.position.difference(&leading_knot.position);

        // leading knot is too far away diagonally
        if distance.abs().x == 2 && distance.abs().y == 2 {
            return Some(Point {
                x: leading_knot.position.x + distance.x.clamp(-1, 1),
                y: leading_knot.position.y + distance.y.clamp(-1, 1),
            });
        // leading knot is too far away horizontally
        } else if distance.abs().x == 2 {
            return Some(Point {
                x: leading_knot.position.x + distance.x.clamp(-1, 1),
                y: leading_knot.position.y,
            });
        // leading knot is too far away vertically
        } else if distance.abs().y == 2 {
            return Some(Point {
                x: leading_knot.position.x,
                y: leading_knot.position.y + distance.y.clamp(-1, 1),
            });
        } else {
            return None;
        }
    }

    pub fn get_visited_points(&self) -> &HashSet<Point> {
        return &self.visited_points;
    }
}
