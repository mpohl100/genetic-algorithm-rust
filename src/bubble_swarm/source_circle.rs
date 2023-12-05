use crate::math2d::circle::Circle;

use super::angle_area::AngleArea;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct SourceCircle {
    circle: Circle,
    angle_area: AngleArea,
}

impl SourceCircle {
    pub fn default(circle: Circle) -> Self {
        Self {
            circle,
            angle_area: AngleArea::new(0.0, 6),
        }
    }

    pub fn new(circle: Circle, angle_area: AngleArea) -> Self {
        Self { circle, angle_area }
    }

    pub fn get_circle(&self) -> Circle {
        self.circle
    }

    pub fn get_angle_area(&self) -> AngleArea {
        self.angle_area
    }
}
