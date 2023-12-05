use crate::math2d::circle::Circle;

use super::segment::Segment;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct SourceCircle {
    circle: Circle,
    segment: Segment,
}

impl SourceCircle {
    pub fn default(circle: Circle) -> Self {
        Self {
            circle,
            segment: Segment::new(0.0, 6),
        }
    }

    pub fn new(circle: Circle, segment: Segment) -> Self {
        Self { circle, segment }
    }

    pub fn get_circle(&self) -> Circle {
        self.circle
    }

    pub fn get_segment(&self) -> Segment {
        self.segment
    }
}
