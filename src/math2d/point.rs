use std::ops::Add;

use super::vector::Vector;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Add<Vector> for Point {
    type Output = ();
    fn add(mut self, vec: Vector) {
        let (xx, yy) = vec.get_coordinates();
        self.x += xx;
        self.y += yy;
    }
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Point { x, y }
    }
}
