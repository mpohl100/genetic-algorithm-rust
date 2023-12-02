use crate::math2d::point::Point;
use crate::math2d::line::Line;
use crate::math2d::rectangle::Rectangle;
use crate::math2d::circle::Circle;
use std::collections::BTreeSet;

#[derive(Debug, Clone)]
pub struct Canvas{
    pixels: Vec<Vec<i32>>,
    points: Vec<Point>,
}

impl Canvas{
    pub fn new(width: usize, height: usize) -> Canvas{
        let mut pixels = Vec::with_capacity(height);
        for _ in 0..height{
            let mut row = Vec::with_capacity(width);
            for _ in 0..width{
                row.push(0);
            }
            pixels.push(row);
        }
        Canvas{
            pixels,
            points: Vec::new(),
        }
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> i32{
        self.pixels[y][x]
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, value: i32){
        self.pixels[y][x] = value;
        if value == 0{
            let index = self.points.iter().position(|point| *point == Point::new(x as f32, y as f32));
            self.points.remove(index.unwrap());
        }else{
            self.points.push(Point::new(x as f32, y as f32));
        }
    }

    pub fn get_points(&self) -> &Vec<Point>{
        &self.points
    }

    pub fn get_width(&self) -> usize{
        self.pixels[0].len()
    }

    pub fn get_height(&self) -> usize{
        self.pixels.len()
    }

    pub fn clear(&mut self){
        for row in self.pixels.iter_mut(){
            for pixel in row.iter_mut(){
                *pixel = 0;
            }
        }
        self.points.clear();
    }
    pub fn get_pixels(&self) -> String{
        let mut ret = String::new();
        for row in self.pixels.iter(){
            for pixel in row.iter(){
                if *pixel == 0{
                    ret.push_str(".");
                }else{
                    ret.push_str("X");
                }
            }
            ret.push_str("\n");
        }
        ret
    }

    pub fn draw_line(&mut self, line: &Line){
    
    }

    pub fn draw_rectangle(&mut self, rectangle: Rectangle){
        for line in rectangle.get_lines(){
            self.draw_line(line);
        }
    }

    pub fn draw_circle(&mut self, circle: Circle){
    
    }
}

#[cfg(test)]
mod canvas_tests {
    use crate::math2d::line::Line;
    use crate::math2d::point::Point;
    use crate::math2d::rectangle::Rectangle;
    use crate::math2d::circle::Circle;
    use super::Canvas;

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
                canvas.draw_line(&Line::new(Point::new(5.0, 5.0), Point::new(i as f32, j as f32)));
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
                      ....XXX...\n\
                      ...X...X..\n\
                      ..X.....X.\n\
                      ..X.....X.\n\
                      ..X.....X.\n\
                      ...X...X..\n\
                      ....XXX...\n\
                      ..........\n";
        assert_eq!(canvas_pixels, result);
    }
}

