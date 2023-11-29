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

    pub fn get_start_coordinates(&self) -> (f32, f32) {
        self.start.get_coordinates()
    }

    pub fn get_end_coordinates(&self) -> (f32, f32) {
        self.end.get_coordinates()
    }

    pub fn get_magnitude(&self) -> f32 {
        let (start, end) = (self.get_start_coordinates(), self.get_end_coordinates());
        let xx = end.0 - start.0;
        let yy = end.1 - start.1;
        (xx.powi(2) + yy.powi(2)).sqrt()
    }
}
