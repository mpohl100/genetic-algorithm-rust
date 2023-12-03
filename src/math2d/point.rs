use std::ops::Add;

use super::vector::Vector;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Point {
    x: f32,
    y: f32,
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

    pub fn get_coordinates(&self) -> (f32, f32) {
        (self.x, self.y)
    }

    pub fn get_x(&self) -> f32 {
        self.x
    }

    pub fn get_y(&self) -> f32 {
        self.y
    }
}
