use super::{
    point::Point,
    regioned_angle::{RegionedAngle, RegionedAngleTrait},
};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vector {
    xx: f32,
    yy: f32,
}

impl From<(Point, Point)> for Vector {
    fn from((start, end): (Point, Point)) -> Self {
        let (start_x, start_y) = start.get_coordinates();
        let (end_x, end_y) = end.get_coordinates();
        Vector {
            xx: end_x - start_x,
            yy: end_y - start_y,
        }
    }
}

impl Vector {
    pub fn new(xx: f32, yy: f32) -> Self {
        Vector { xx, yy }
    }

    pub fn get_coordinates(&self) -> (f32, f32) {
        (self.xx, self.yy)
    }

    pub fn scale(&self, scalar: f32) -> Self {
        Vector::new(self.xx * scalar, self.yy * scalar)
    }

    pub fn magnitude(&self) -> f32 {
        (self.xx.powi(2) + self.yy.powi(2)).sqrt()
    }

    pub fn rotate(&self, angle: RegionedAngle<-180, 180>) -> Self {
        let radians = angle.get_radians();
        let cos_angle = radians.cos();
        let sin_angle = radians.sin();
        let dx: f32 = 0.0;
        let dy: f32 = 0.0;
        let x_rotated = ((self.xx - dx) * cos_angle) - ((dy - self.yy) * sin_angle) + dx;
        let y_rotated = dy - ((dy - self.yy) * cos_angle) + ((self.xx - dx) * sin_angle);
        Vector::new(x_rotated, y_rotated)
    }
}
