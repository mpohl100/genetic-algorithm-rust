mod bubble_swarm;
mod canvas;
mod evol;
mod math2d;

use crate::canvas::Canvas;
use crate::evol::evol_options::EvolutionOptions;
use crate::math2d::circle::Circle;
use crate::math2d::point::Point;

fn main() {
    println!("Hello, world!");
    let options = EvolutionOptions::new();
    let mut canvas = Canvas::new(40, 40);
    canvas.draw_circle(Circle::new(Point::new(20.0, 20.0), 10.0));
    let canvas_pixels = canvas.get_pixels();
    print!("{}", canvas_pixels);
}
