use std::ops::Range;
use std::ops::RangeInclusive;

fn get_positive_range_inclusive(range: Range<i32>) -> RangeInclusive<i32> {
    if range.start < range.end {
        return range.start..=range.end;
    } else {
        return range.end..=range.start;
    };
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone)]
pub struct VentLine {
    pub start_point: Point,
    pub end_point: Point,
}

impl VentLine {
    pub fn is_horizontal(&self) -> bool {
        return self.start_point.y == self.end_point.y;
    }

    pub fn is_vertical(&self) -> bool {
        return self.start_point.x == self.end_point.x;
    }

    pub fn get_all_points(&self, with_diagonals: bool) -> Vec<Point> {
        let horizontal_range = get_positive_range_inclusive(self.start_point.x..self.end_point.x);
        let vertical_range = get_positive_range_inclusive(self.start_point.y..self.end_point.y);

        if self.is_horizontal() {
            return horizontal_range
                .map(|x| Point {
                    x,
                    y: self.start_point.y,
                })
                .collect::<Vec<Point>>();
        } else if self.is_vertical() {
            return vertical_range
                .map(|y| Point {
                    x: self.start_point.x,
                    y,
                })
                .collect::<Vec<Point>>();
        } else if with_diagonals {
            let mut horizontal_vec: Vec<i32> = horizontal_range.collect();
            let mut vertical_vec: Vec<i32> = vertical_range.collect();

            if self.start_point.x > self.end_point.x {
                horizontal_vec = horizontal_vec.into_iter().rev().collect();
            }

            if self.start_point.y > self.end_point.y {
                vertical_vec = vertical_vec.into_iter().rev().collect();
            }

            return horizontal_vec
                .into_iter()
                .zip(vertical_vec)
                .map(|zip| {
                    let (x, y) = zip;
                    return Point { x, y };
                })
                .collect::<Vec<Point>>();
        } else {
            return Vec::new();
        }
    }
}

/// Constructs a VentLine from an input line such as "123,456 -> 789,123"
pub fn from(input_line: &str) -> Result<VentLine, String> {
    if let [start_point_str, end_point_str] = input_line.split(" -> ").collect::<Vec<&str>>()[..] {
        let start_point: Point;
        let end_point: Point;

        if let [start_point_x, start_point_y] =
            start_point_str.split(",").collect::<Vec<&str>>()[..]
        {
            start_point = Point {
                x: start_point_x.parse::<i32>().unwrap(),
                y: start_point_y.parse::<i32>().unwrap(),
            };
        } else {
            return Err("Could not parse start point".to_string());
        }

        if let [end_point_x, end_point_y] = end_point_str.split(",").collect::<Vec<&str>>()[..] {
            end_point = Point {
                x: end_point_x.parse::<i32>().unwrap(),
                y: end_point_y.parse::<i32>().unwrap(),
            };
        } else {
            return Err("Could not parse end point".to_string());
        }

        return Ok(VentLine {
            start_point,
            end_point,
        });
    } else {
        return Err("Could not split input by ' -> '".to_string());
    }
}
