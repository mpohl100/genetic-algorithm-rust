use super::point::Point;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vector {
    xx: f32,
    yy: f32,
}

// Constructors
impl Vector {
    pub fn new(xx: f32, yy: f32) -> Self {
        Vector { xx, yy }
    }

    pub fn new_from_points(start: Point, end: Point) -> Self {
        Vector {
            xx: end.x - start.x,
            yy: end.y - start.y,
        }
    }

    pub fn get_coordinates(&self) -> (f32, f32) {
        (self.xx, self.yy)
    }
}
