mod bubble_swarm;
mod canvas;
mod evol;
mod math2d;

use crate::canvas::Canvas;
use crate::evol::ordinary_evol::EvolutionOptions;

fn main() {
    println!("Hello, world!");
    let options = EvolutionOptions::new();
    let mut canvas = Canvas::new(10, 10);
}
