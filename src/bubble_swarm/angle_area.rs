use crate::math2d::regioned_angle::{RegionedAngle, RegionedAngleTrait};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct AngleArea {
    area: f32,
    nb_angles: usize,
}

impl AngleArea {
    pub fn new(area: f32, nb_angles: usize) -> AngleArea {
        AngleArea { area, nb_angles }
    }

    pub fn is_within<T>(&self, angle: &T) -> bool
    where
        T: RegionedAngleTrait,
    {
        let segment_angle = 360.0 / self.nb_angles as f32;
        let angle_1 = segment_angle * self.area - 1e-10;
        let angle_2 = segment_angle * (self.area + 1.0) + 1e-10;
        let converted_angle = RegionedAngle::<0, 360>::from_angle(angle);
        let degrees = converted_angle.degrees();
        degrees >= angle_1 && degrees <= angle_2
    }

    pub fn get_angle(&self, factor: f32) -> RegionedAngle<-180, 180> {
        let segment_angle = 360.0 / self.nb_angles as f32;
        let degrees = segment_angle * self.area + segment_angle * factor;
        RegionedAngle::<-180, 180>::from(degrees)
    }
}
