use crate::math2d::point::Point;
use crate::math2d::vector::Vector;
pub struct NormVector {
    x: f32,
    y: f32,
}

impl NormVector {
    pub fn new(x: f32, y: f32) -> Self {
        let magnitude = (x.powi(2) + y.powi(2)).sqrt();
        NormVector {
            x: x / magnitude,
            y: y / magnitude,
        }
    }

    pub fn get_vector(&self) -> Vector {
        Vector::new(self.x, self.y)
    }
}

impl From<(Point, Point)> for NormVector {
    fn from((from, to): (Point, Point)) -> Self {
        let x = to.get_x() - from.get_x();
        let y = to.get_y() - from.get_y();
        NormVector::new(x, y)
    }
}

impl From<Vector> for NormVector {
    fn from(vector: Vector) -> Self {
        NormVector::new(vector.get_x(), vector.get_y())
    }
}
