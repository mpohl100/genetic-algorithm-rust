use super::point::Point;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Line {
    start: Point,
    end: Point,
}

impl Line {
    pub fn new(start: Point, end: Point) -> Self {
        Line { start, end }
    }

    pub fn get_start(&self) -> &Point {
        &self.start
    }

    pub fn get_end(&self) -> &Point {
        &self.end
    }

    pub fn get_magnitude(&self) -> f32 {
        let xx = self.end.x - self.start.x;
        let yy = self.end.y - self.start.y;
        (xx.powi(2) + yy.powi(2)).sqrt()
    }
}
