use super::{line::Line, point::Point};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Rectangle {
    lines: [Line; 4],
}

impl Rectangle {
    pub fn new(tl: Point, br: Point) -> Self {
        let (tl_x, tl_y) = tl.get_coordinates();
        let (br_x, br_y) = br.get_coordinates();

        let tr = Point::new(br_x, tl_y);
        let bl = Point::new(tl_x, br_y);

        let lines = [
            Line::new(tl, tr),
            Line::new(tr, br),
            Line::new(br, bl),
            Line::new(bl, tl),
        ];
        Rectangle { lines }
    }

    pub fn get_lines(&self) -> &[Line; 4] {
        &self.lines
    }

    pub fn area(&self) -> f32 {
        &self.lines[0].get_magnitude() * &self.lines[1].get_magnitude()
    }
}
