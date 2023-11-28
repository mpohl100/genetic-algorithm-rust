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
        let (start_x, start_y) = start.get_coordinates();
        let (end_x, end_y) = end.get_coordinates();
        Vector {
            xx: end_x - start_x,
            yy: end_y - start_y,
        }
    }

    pub fn get_coordinates(&self) -> (f32, f32) {
        (self.xx, self.yy)
    }
}
