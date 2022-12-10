#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy, PartialOrd, Ord)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn difference(&self, comparison: &Point) -> Point {
        return Point {
            x: self.x - comparison.x,
            y: self.y - comparison.y,
        };
    }

    pub fn abs(&self) -> Point {
        return Point {
            x: self.x.abs(),
            y: self.y.abs(),
        };
    }
}
