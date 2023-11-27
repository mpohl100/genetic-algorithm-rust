use std::ops::Add;

use super::vector::Vector;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Add<&Vector> for Point {
    type Output = Self;
    fn add(mut self, vec: &Vector) -> Self {
        self.x += vec.xx;
        self.y += vec.yy;
        self
    }
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Point { x, y }
    }
}
