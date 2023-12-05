use crate::math2d::circle::Circle;

pub struct AlreadyOptimizedCircles {
    circles: Vec<Circle>,
}

impl AlreadyOptimizedCircles {
    pub fn new() -> Self {
        Self { circles: vec![] }
    }

    pub fn get_circles(&self) -> &Vec<Circle> {
        &self.circles
    }

    pub fn add_circle(&mut self, circle: Circle) {
        self.circles.push(circle);
    }

    pub fn area(&self) -> f32 {
        self.circles.iter().map(|c| c.area()).sum()
    }
}
