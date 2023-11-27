use rand::prelude::*;
use rand::distributions::Uniform;
use std::vec::Vec;

pub struct EvolutionOptions{
    num_generations: u64, // the number of generations to cross
    log_level: u64, // logging level to see how far the algorithm progressed
    num_parents: u64, // the number of parents to grow a new generation
    num_children: u64, // the number of phenotypes to breed per generation
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

pub struct EvolutionCoordinator{
    current_generation: u64,
    num_generations: u64,
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

pub trait Phenotype{
    fn crossover(&mut self, other: &Self) where Self: Sized;
    fn mutate(&mut self, rng: &RandomNumberGenerator, evolCoordinator: EvolutionCoordinator);
    fn to_string_internal(&self) -> String;   
}

pub trait Challenge{
    fn score(&self, phenotype: Box<dyn Phenotype>, rng: &RandomNumberGenerator) -> f64;
    fn breed(&self, parents: Vec<Box<dyn Phenotype>>, rng: &RandomNumberGenerator, evolCoordinator: EvolutionCoordinator, evolOptions: EvolutionOptions) -> Vec<Box<dyn Phenotype>>;
}
