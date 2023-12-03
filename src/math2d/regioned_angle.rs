use std::f32::consts::PI;

use super::{line::Line, point::Point, vector::Vector};

pub trait RegionedAngleTrait {
    fn get_radians(&self) -> f32;
    fn degrees(&self) -> f32;
    fn min_degrees(&self) -> i32;
    fn max_degrees(&self) -> i32;
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct RegionedAngle<const MIN_DEGREES: i32, const MAX_DEGREES: i32> {
    radians: f32,
}

impl<const MIN_DEGREES: i32, const MAX_DEGREES: i32> RegionedAngle<MIN_DEGREES, MAX_DEGREES> {
    pub fn new() -> Self {
        Self { radians: 0.0 }
    }

    pub fn from_angle<A>(other: &A) -> Self
    where
        A: RegionedAngleTrait,
    {
        let mut regioned_angle: RegionedAngle<MIN_DEGREES, MAX_DEGREES> = RegionedAngle {
            radians: other.get_radians(),
        };
        regioned_angle.move_to_range();
        regioned_angle
    }

    fn move_to_range(&mut self) -> Self {
        while self.radians < MIN_DEGREES as f32 / 180.0 * PI {
            self.radians += 2.0 * PI;
        }
        while self.radians >= MAX_DEGREES as f32 / 180.0 * PI {
            self.radians -= 2.0 * PI;
        }
        *self
    }

    fn radians_from_vectors(v1: &Vector, v2: &Vector) -> f32 {
        let dot_product = v1.dot(v2);
        let cross_product = v1.cross(v2);
        let magnitude_product = v1.magnitude() * v2.magnitude();
        let cos_angle = dot_product / magnitude_product;
        let angle = cos_angle.acos();
        if cross_product < 0.0 {
            -angle
        } else {
            angle
        }
    }
}

impl<const MIN_DEGREES: i32, const MAX_DEGREES: i32> From<(Point, Point, Point)>
    for RegionedAngle<MIN_DEGREES, MAX_DEGREES>
{
    fn from((start, center, end): (Point, Point, Point)) -> Self {
        let v1 = Vector::from((center, start));
        let v2 = Vector::from((center, end));
        let radians = RegionedAngle::<-180, 180>::radians_from_vectors(&v1, &v2);
        let mut regioned_angle: RegionedAngle<MIN_DEGREES, MAX_DEGREES> = RegionedAngle { radians };
        regioned_angle.move_to_range()
    }
}

impl<const MIN_DEGREES: i32, const MAX_DEGREES: i32> From<(&Line, &Line)>
    for RegionedAngle<MIN_DEGREES, MAX_DEGREES>
{
    fn from((l1, l2): (&Line, &Line)) -> Self {
        let v1 = Vector::from((l1.get_start(), l1.get_end()));
        let v2 = Vector::from((l2.get_start(), l2.get_end()));
        let radians = RegionedAngle::<-180, 180>::radians_from_vectors(&v1, &v2);
        let mut regioned_angle: RegionedAngle<MIN_DEGREES, MAX_DEGREES> = RegionedAngle { radians };
        regioned_angle.move_to_range()
    }
}

impl<const MIN_DEGREES: i32, const MAX_DEGREES: i32> From<f32>
    for RegionedAngle<MIN_DEGREES, MAX_DEGREES>
{
    fn from(degrees: f32) -> Self {
        let radians = degrees / 180.0 * PI;
        let mut regioned_angle: RegionedAngle<MIN_DEGREES, MAX_DEGREES> = RegionedAngle { radians };
        regioned_angle.move_to_range()
    }
}

impl<const MIN_DEGREES: i32, const MAX_DEGREES: i32> RegionedAngleTrait
    for RegionedAngle<MIN_DEGREES, MAX_DEGREES>
{
    fn get_radians(&self) -> f32 {
        self.radians
    }

    fn degrees(&self) -> f32 {
        self.radians.to_degrees()
    }

    fn min_degrees(&self) -> i32 {
        MIN_DEGREES
    }

    fn max_degrees(&self) -> i32 {
        MAX_DEGREES
    }
}
