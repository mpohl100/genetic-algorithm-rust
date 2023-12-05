use super::point::Point;
use std::f32::consts::PI;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Circle {
    center: Point,
    radius: f32,
}

impl Circle {
    pub fn new(center: Point, radius: f32) -> Self {
        Circle { center, radius }
    }

    pub fn get_center(&self) -> Point {
        self.center
    }

    pub fn get_center_coords(&self) -> (f32, f32) {
        self.center.get_coordinates()
    }

    pub fn get_radius(&self) -> f32 {
        self.radius
    }

    pub fn area(&self) -> f32 {
        PI * self.radius.powi(2)
    }
}
