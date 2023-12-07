use crate::canvas::detail::Pattern;
use crate::math2d::circle::Circle;
use crate::math2d::line::Line;
use crate::math2d::point::Point;
use crate::math2d::rectangle::Rectangle;
use crate::math2d::vector::Vector;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PositivePoint {
    x: usize,
    y: usize,
}

impl PositivePoint {
    fn new(x: usize, y: usize) -> PositivePoint {
        PositivePoint { x, y }
    }
}

impl From<Point> for PositivePoint {
    fn from(point: Point) -> PositivePoint {
        let (x, y) = point.get_coordinates();
        PositivePoint::new(x as usize, y as usize)
    }
}

#[derive(Clone)]
pub struct Canvas {
    pixels: Vec<Vec<i32>>,
    points: Vec<Point>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        let mut pixels = Vec::with_capacity(height);
        for _ in 0..height {
            let mut row = Vec::with_capacity(width);
            for _ in 0..width {
                row.push(0);
            }
            pixels.push(row);
        }
        Canvas {
            pixels,
            points: Vec::new(),
        }
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> i32 {
        self.pixels[y][x]
    }

    pub fn draw_pixel_maybe(&mut self, point: Point, value: i32) {
        if point.get_x() < 0.0 || point.get_y() < 0.0 {
            return;
        }
        let x = point.get_x() as usize;
        let y = point.get_y() as usize;
        if x < self.get_width() && y < self.get_height() {
            self.draw_pixel(PositivePoint::new(x, y), value);
        }
    }

    pub fn draw_pixel(&mut self, positive_point: PositivePoint, value: i32) {
        self.pixels[positive_point.x][positive_point.y] = value;
        if value == 0 {
            let index = self
                .points
                .iter()
                .position(|point| PositivePoint::from(*point) == positive_point);
            self.points.remove(index.unwrap());
        } else {
            self.points
                .push(Point::new(positive_point.x as f32, positive_point.y as f32));
        }
    }

    pub fn get_points(&self) -> &Vec<Point> {
        &self.points
    }

    pub fn get_width(&self) -> usize {
        self.pixels[0].len()
    }

    pub fn get_height(&self) -> usize {
        self.pixels.len()
    }

    pub fn clear(&mut self) {
        for row in self.pixels.iter_mut() {
            for pixel in row.iter_mut() {
                *pixel = 0;
            }
        }
        self.points.clear();
    }
    pub fn get_pixels(&self) -> String {
        let mut ret = String::new();
        for row in self.pixels.iter() {
            for pixel in row.iter() {
                if *pixel == 0 {
                    ret.push_str(".");
                } else {
                    ret.push_str("X");
                }
            }
            ret.push_str("\n");
        }
        ret
    }

    fn draw_line(&mut self, line: &Line) {
        let mut start = line.get_start();
        let mut end = line.get_end();
        if end.get_coordinates().0 < start.get_coordinates().0 {
            std::mem::swap(&mut start, &mut end);
        }
        const DO_LOG: bool = false;
        let mut d_x = end.get_coordinates().0 - start.get_coordinates().0;
        let mut d_y = end.get_coordinates().1 - start.get_coordinates().1;
        let mut current_point = start;
        // draw start and end point
        self.draw_pixel(PositivePoint::from(start), 1);
        self.draw_pixel(PositivePoint::from(end), 1);
        if d_x == 0.0 {
            if d_y >= 0.0 {
                for _ in 0..=d_y as usize {
                    self.draw_pixel(PositivePoint::from(current_point), 1);
                    current_point = Point::new(current_point.get_x(), current_point.get_y() + 1.0);
                }
            } else {
                for _ in 0..=(-d_y) as usize {
                    self.draw_pixel(PositivePoint::from(current_point), 1);
                    current_point = Point::new(current_point.get_x(), current_point.get_y() - 1.0);
                }
            }
            return;
        }
        if d_y == 0.0 {
            if d_x >= 0.0 {
                if DO_LOG {
                    println!("x positive");
                }
                for _ in 0..=d_x as usize {
                    self.draw_pixel(PositivePoint::from(current_point), 1);
                    current_point = Point::new(current_point.get_x() + 1.0, current_point.get_y());
                }
            } else {
                for _ in 0..=(-d_x) as usize {
                    self.draw_pixel(PositivePoint::from(current_point), 1);
                    current_point = Point::new(current_point.get_x() - 1.0, current_point.get_y());
                }
            }
            return;
        }

        let gradient = f64::from(d_y) / f64::from(d_x);
        if DO_LOG {
            println!("gradient = {}; dX={}; dY={}", gradient, d_x, d_y);
        }
        enum Direction {
            X,
            YUp,
            YDown,
        }
        let move_coord = |d: &mut f32, point: &mut Point, went: &mut i32, direction: Direction| {
            match direction {
                Direction::X => {
                    *d -= 1.0;
                    *point = Point::new(point.get_x() + 1.0, point.get_y());
                    *went += 1;
                }
                Direction::YUp => {
                    *d -= 1.0;
                    *point = Point::new(point.get_x(), point.get_y() + 1.0);
                    *went += 1;
                }
                Direction::YDown => {
                    *d += 1.0;
                    *point = Point::new(point.get_x(), point.get_y() - 1.0);
                    *went -= 1;
                }
            }
        };
        let go_x = |went_x: f64, went_y: f64| -> Direction {
            let deduce_current_gradient = || -> f64 {
                if went_x == 0.0 {
                    if went_y > 0.0 {
                        return f64::INFINITY;
                    } else if went_y == 0.0 {
                        return 0.0;
                    } else {
                        return -f64::INFINITY;
                    }
                }
                went_y / went_x
            };
            if DO_LOG {
                println!("went_x = {}; went_y = {}", went_x, went_y);
            }
            let current_gradient = deduce_current_gradient();
            if DO_LOG {
                println!(
                    "current gradient = {}; gradient = {}",
                    current_gradient, gradient
                );
            }
            if gradient >= 0.0 {
                if current_gradient > gradient {
                    if DO_LOG {
                        println!("go x");
                    }
                    return Direction::X;
                }
                if DO_LOG {
                    println!("go y up");
                }
                return Direction::YUp;
            } else {
                if current_gradient < gradient {
                    if DO_LOG {
                        println!("go x");
                    }
                    return Direction::X;
                }
                if DO_LOG {
                    println!("go y down");
                }
                return Direction::YDown;
            }
        };
        let mut went_x = 0;
        let mut went_y = 0;
        loop {
            let positive_current_point = PositivePoint::from(current_point);
            if d_x == 0.0 && d_y == 0.0 {
                if positive_current_point != PositivePoint::from(end) {
                    panic!("end point not hit in draw_line.");
                }
                self.draw_pixel(positive_current_point, 1);
                break;
            }
            if DO_LOG {
                println!(
                    "setting point to 1: x={}; y={}; dX={}; dY={}",
                    current_point.get_x(),
                    current_point.get_y(),
                    d_x,
                    d_y
                );
            }
            self.draw_pixel(positive_current_point, 1);
            let direction = go_x(f64::from(went_x), f64::from(went_y));
            match direction {
                Direction::X => move_coord(&mut d_x, &mut current_point, &mut went_x, direction),
                Direction::YUp | Direction::YDown => {
                    move_coord(&mut d_y, &mut current_point, &mut went_y, direction)
                }
            }
        }
    }

    pub fn draw_rectangle(&mut self, rectangle: Rectangle) {
        for line in rectangle.get_lines() {
            self.draw_line(line);
        }
    }

    pub fn draw_circle(&mut self, circle: Circle) {
        let start_x =
            (circle.get_center() + Vector::new(-circle.get_radius() - 5.0, 0.0)).get_x() as i32;
        let end_x =
            (circle.get_center() + Vector::new(circle.get_radius() + 5.0, 0.0)).get_x() as i32;
        let start_y =
            (circle.get_center() + Vector::new(0.0, -circle.get_radius() - 5.0)).get_y() as i32;
        let end_y =
            (circle.get_center() + Vector::new(0.0, circle.get_radius() + 5.0)).get_y() as i32;
        let mut all_patterns = Vec::<Vec<Pattern>>::new();
        for y in start_y..end_y {
            let mut line_patterns = Vec::<Pattern>::new();
            for x in start_x..end_x {
                let mut previous_pattern;
                if line_patterns.is_empty() {
                    previous_pattern = Pattern::new(x - 1, y, vec!['.']);
                } else {
                    previous_pattern = line_patterns[line_patterns.len() - 1].clone();
                }
                let point = Point::new(x as f32, y as f32);
                let distance = Vector::from((circle.get_center(), point)).magnitude();
                if distance <= circle.get_radius() {
                    line_patterns.push(Pattern::new(
                        x,
                        y,
                        vec![
                            previous_pattern.get_pattern()
                                [previous_pattern.get_pattern().len() - 1],
                            'X',
                        ],
                    ));
                } else {
                    line_patterns.push(Pattern::new(
                        x,
                        y,
                        vec![
                            previous_pattern.get_pattern()
                                [previous_pattern.get_pattern().len() - 1],
                            '.',
                        ],
                    ));
                }
            }
            if !line_patterns.is_empty() {
                all_patterns.push(line_patterns);
            }
        }

        // use all_patterns to actually draw the circle
        let mut previous_pattern = Vec::<Pattern>::new();
        for (i, line_patterns) in all_patterns.iter().enumerate() {
            let start_pattern = line_patterns
                .iter()
                .find(|pattern| pattern.get_pattern() == vec!['.', 'X']);
            if start_pattern.is_none() {
                continue;
            }
            let end_pattern = line_patterns
                .iter()
                .find(|pattern| pattern.get_pattern() == vec!['X', '.']);
            if end_pattern.is_none() {
                continue;
            }
            let mut previous_pattern_found = true;
            let previous_start_pattern = previous_pattern
                .iter()
                .find(|pattern| pattern.get_pattern() == vec!['.', 'X']);
            if previous_start_pattern.is_none() {
                previous_pattern_found = false;
            }
            let previous_end_pattern = previous_pattern
                .iter()
                .find(|pattern| pattern.get_pattern() == vec!['X', '.']);
            if previous_end_pattern.is_none() {
                previous_pattern_found = false;
            }
            if !previous_pattern_found {
                // first line
                for x in start_pattern.unwrap().get_x()..end_pattern.unwrap().get_x() {
                    self.draw_pixel_maybe(Point::new(x as f32, line_patterns[0].get_y() as f32), 1);
                }
            } else {
                let mut previous_start_x = previous_start_pattern.unwrap().get_x();
                let mut current_start_x = start_pattern.unwrap().get_x();
                let mut previous_end_x = previous_end_pattern.unwrap().get_x();
                let mut current_end_x = end_pattern.unwrap().get_x();
                if previous_start_x > current_start_x {
                    std::mem::swap(&mut previous_start_x, &mut current_start_x);
                }
                if previous_end_x > current_end_x {
                    std::mem::swap(&mut previous_end_x, &mut current_end_x);
                }
                for x in previous_start_x..current_start_x + 1 {
                    self.draw_pixel_maybe(Point::new(x as f32, line_patterns[0].get_y() as f32), 1);
                }
                for x in previous_end_x..current_end_x + 1 {
                    self.draw_pixel_maybe(Point::new(x as f32, line_patterns[0].get_y() as f32), 1);
                }
            }
            previous_pattern = line_patterns.to_vec();
        }
    }
}

mod detail {
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Pattern {
        x: i32,
        y: i32,
        pattern: Vec<char>,
    }

    impl Pattern {
        pub fn new(x: i32, y: i32, pattern: Vec<char>) -> Pattern {
            Pattern { x, y, pattern }
        }

        pub fn get_x(&self) -> i32 {
            self.x
        }

        pub fn get_y(&self) -> i32 {
            self.y
        }

        pub fn get_pattern(&self) -> Vec<char> {
            self.pattern.clone()
        }
    }
}

#[cfg(test)]
mod canvas_tests {
    use super::Canvas;
    use crate::math2d::circle::Circle;
    use crate::math2d::line::Line;
    use crate::math2d::point::Point;
    use crate::math2d::rectangle::Rectangle;

    #[test]
    fn canvas_line() {
        let mut canvas = Canvas::new(10, 10);
        canvas.draw_line(&Line::new(Point::new(1.0, 1.0), Point::new(7.0, 3.0)));
        let canvas_pixels = canvas.get_pixels();
        let result = "..........\n\
                      .XX.......\n\
                      ..X.......\n\
                      ..X.......\n\
                      ..XX......\n\
                      ...X......\n\
                      ...X......\n\
                      ...X......\n\
                      ..........\n\
                      ..........\n";
        assert_eq!(canvas_pixels, result);
    }

    #[test]
    fn canvas_rectangle() {
        let mut canvas = Canvas::new(5, 5);
        canvas.draw_rectangle(Rectangle::new(Point::new(1.0, 1.0), Point::new(3.0, 3.0)));
        let canvas_pixels = canvas.get_pixels();
        assert_eq!(canvas_pixels, ".....\n.XXX.\n.X.X.\n.XXX.\n.....\n");
    }

    #[test]
    fn canvas_line_fuzztest() {
        for i in 1..9 {
            for j in 1..9 {
                let mut canvas = Canvas::new(10, 10);
                canvas.draw_line(&Line::new(
                    Point::new(5.0, 5.0),
                    Point::new(i as f32, j as f32),
                ));
                let canvas_pixels = canvas.get_pixels();
                assert_eq!(canvas_pixels.chars().nth(5 * 11 + 5).unwrap(), 'X');
                assert_eq!(canvas_pixels.chars().nth(i * 11 + j).unwrap(), 'X');
            }
        }
    }

    #[test]
    fn canvas_circle() {
        let mut canvas = Canvas::new(10, 10);
        canvas.draw_circle(Circle::new(Point::new(5.0, 5.0), 3.0));
        let canvas_pixels = canvas.get_pixels();
        let result = "..........\n\
                      ..........\n\
                      .....XX...\n\
                      ...XXXXXX.\n\
                      ...X....X.\n\
                      ..XX....X.\n\
                      ...X....X.\n\
                      ...X....X.\n\
                      ...XXXXXX.\n\
                      .....XX...\n";
        assert_eq!(canvas_pixels, result);
    }
}
