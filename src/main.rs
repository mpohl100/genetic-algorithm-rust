mod evol;
mod math2d;
mod canvas;

use crate::evol::ordinary_evol::EvolutionOptions;
use crate::canvas::Canvas;

fn main() {
    println!("Hello, world!");
    let options = EvolutionOptions::new();
    let mut canvas = Canvas::new(10, 10);
}
