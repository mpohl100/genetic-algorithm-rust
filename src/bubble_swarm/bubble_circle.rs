use crate::math2d::{circle::Circle, point::Point, vector::Vector};

use super::{
    source_circle::SourceCircle,
    utils::{calculate_circle_intersection, get_mid_point},
};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct BubbleCircle {
    circle: Circle,
    source_circle: SourceCircle,
}

impl BubbleCircle {
    pub fn new(circle: Circle, source_circle: SourceCircle) -> Self {
        Self {
            circle,
            source_circle,
        }
    }

    fn get_radius(&self, center: Point) -> usize {
        let p = self.source_circle.get_circle().get_center();
        let radius =
            Vector::from((center, p)).magnitude() - self.source_circle.get_circle().get_radius();
        if radius >= 0.0 {
            radius as usize
        } else {
            0
        }
    }

    fn get_circle(&self) -> Circle {
        self.circle
    }

    pub fn get_source_circle(&self) -> SourceCircle {
        self.source_circle
    }

    fn is_within_angle_of_source_circle(&self) -> bool {
        let get_radius = |vec: Vector| -> usize {
            let magnitude = vec.magnitude();
            if magnitude >= 0.0 {
                magnitude as usize
            } else {
                0
            }
        };
        let thales_circle = Circle::new(
            get_mid_point(
                self.get_circle().get_center(),
                self.get_source_circle().get_circle().get_center(),
            ),
            get_radius(
                Vector::from((
                    self.get_circle().get_center(),
                    self.get_source_circle().get_circle().get_center(),
                ))
                .scale(0.5),
            ) as f32,
        );
        let intersection_points = calculate_circle_intersection(&self.get_circle(), &thales_circle);
        todo!()
    }

    pub fn crossover(&mut self, other: &Self) {
        let new_center = Point::new(
            (self.circle.get_center_coords().0 + other.circle.get_center_coords().0) / 2.0,
            (self.circle.get_center_coords().1 + other.circle.get_center_coords().1) / 2.0,
        );
        self.circle = Circle::new(new_center, self.circle.get_radius());
    }
}
