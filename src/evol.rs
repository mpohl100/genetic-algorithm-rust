use rand::prelude::*;
use std::vec::Vec;

pub struct EvolutionOptions{
    num_generations: usize, // the number of generations to cross
    log_level: u64, // logging level to see how far the algorithm progressed
    num_parents: usize, // the number of parents to grow a new generation
    num_children: usize, // the number of phenotypes to breed per generation
}

impl EvolutionOptions{
    pub fn new() -> EvolutionOptions{
        EvolutionOptions{
            num_generations: 100,
            log_level: 0,
            num_parents: 2,
            num_children: 20,
        }
    }
}

#[derive(Clone, Copy)]
pub struct EvolutionCoordinator{
    current_generation: usize,
    num_generations: usize,
}

impl EvolutionCoordinator{
    pub fn new(options: EvolutionOptions) -> EvolutionCoordinator{
        EvolutionCoordinator{
            current_generation: 0,
            num_generations: options.num_generations,
        }
    }

    pub fn run(&mut self){
        while self.current_generation < self.num_generations{
            self.current_generation += 1;
            println!("Generation: {}", self.current_generation);
        }
    }

    pub fn get_progress(&self) -> f64{
        self.current_generation as f64 / self.num_generations as f64
    }
}

pub struct RandomNumberGenerator {
    rd: ThreadRng,
}

impl RandomNumberGenerator {
    pub fn new() -> RandomNumberGenerator {
        RandomNumberGenerator {
            rd: rand::thread_rng(),
        }
    }

    pub fn fetch_uniform(&mut self, from: i32, to: i32, num: usize) -> Vec<i32> {
        let mut uniform_numbers = Vec::new();
        for _ in 0..num {
            uniform_numbers.push(self.rd.gen_range(from..to));
        }
        uniform_numbers
    }
}

pub struct EvolutionResult<Pheno> where Pheno: Phenotype{
    winner: Pheno,
    score: f64,
}

pub trait Phenotype where Self: Clone + Sized{
    fn crossover(&mut self, other: &Self);
    fn mutate(&mut self, rng: &mut RandomNumberGenerator, evol_coordinator: EvolutionCoordinator);
    fn to_string_internal(&self) -> String;   
}

pub trait Challenge<Pheno> where Pheno: Phenotype{
    fn score(&self, phenotype: Pheno, rng: &mut RandomNumberGenerator) -> f64;
    fn breed(&self, parents: Vec<Pheno>, rng: &mut RandomNumberGenerator, evol_coordinator: EvolutionCoordinator, evol_options: EvolutionOptions) -> Vec<Pheno>;
}

mod detail{
    use crate::evol::EvolutionResult;
    use crate::evol::EvolutionCoordinator;
    use crate::evol::EvolutionOptions;
    use crate::evol::RandomNumberGenerator;
    use crate::evol::Phenotype;
    use crate::evol::Challenge;

    pub fn breed<Pheno>(parents: Vec<Pheno>, rng: &mut RandomNumberGenerator, evol_coordinator: EvolutionCoordinator, evol_options: EvolutionOptions) -> Vec<Pheno> where Pheno: Phenotype{
        let mut children = Vec::new();
        let winner_previous_generation = &parents[0];
        children.push(winner_previous_generation.clone());
        for i in 1..evol_options.num_parents{
            let mut child = winner_previous_generation.clone();
            child.crossover(&parents[i]);
            child.mutate(rng, evol_coordinator.clone());
            children.push(child);
        }
        for _ in evol_options.num_parents..evol_options.num_children{
            let mut child = winner_previous_generation.clone();
            child.mutate(rng, evol_coordinator.clone());
            children.push(child);
        }
        children
    }

    pub fn evolution<Pheno, Chall>(starting_value: Pheno, challenge: Chall, evol_options: EvolutionOptions, rng: &mut RandomNumberGenerator) -> EvolutionResult<Pheno> where Pheno: Phenotype, Chall: Challenge<Pheno>{
        panic!("Not implemented");
    }
}

#[cfg(test)]
mod evol_test{

use crate::evol::detail;
use crate::evol::EvolutionOptions;
use crate::evol::EvolutionCoordinator;
use crate::evol::RandomNumberGenerator;
use crate::evol::Phenotype;
use crate::evol::Challenge;

#[derive(Debug, Default, Copy, Clone)]
struct XCoordinate{
    x: f64,
}

impl XCoordinate{
    fn new(x: f64) -> XCoordinate{
        XCoordinate{
            x: x,
        }
    }

    fn x(&self) -> f64{
        self.x
    }   
}

impl Phenotype for XCoordinate{
    fn crossover(&mut self, other: &Self) where Self: Sized{
        self.x = (self.x + other.x) / 2.0;
    }

    fn mutate(&mut self, rng: &mut RandomNumberGenerator, evol_coordinator: EvolutionCoordinator){
        let delta = rng.fetch_uniform(-100, 100, 1)[0] as f64;
        self.x += delta / 100.0;
    }

    fn to_string_internal(&self) -> String{
        format!("x: {}", self.x)
    }
}

struct XCoordinateChallenge{
    target: f64,
}

impl XCoordinateChallenge{
    fn new(target: f64) -> XCoordinateChallenge{
        XCoordinateChallenge{
            target: target,
        }
    }
}

impl Challenge<XCoordinate> for XCoordinateChallenge{
    fn score(&self, phenotype: XCoordinate, _rng: &mut RandomNumberGenerator) -> f64{
        let x_coordinate = phenotype.x();
        let delta = x_coordinate - self.target;
        1.0 / (delta * delta)
    }

    fn breed(&self, parents: Vec<XCoordinate>, rng: &mut RandomNumberGenerator, evol_coordinator: EvolutionCoordinator, evol_options: EvolutionOptions) -> Vec<XCoordinate>{
        detail::breed(parents, rng, evol_coordinator, evol_options)
    }
}

#[test]
fn test_evol(){
    let mut rng = RandomNumberGenerator::new();
    let challenge = XCoordinateChallenge::new(2.0);
    let starting_value = XCoordinate::new(0.0);
    let evol_options = EvolutionOptions::new();
    let winner = detail::evolution(starting_value, challenge, evol_options, &mut rng);
    assert_eq!(winner.winner.x(), 2.0);
}

}