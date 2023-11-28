use rand::prelude::*;
use std::vec::Vec;

#[derive(Clone, Copy)]
pub struct EvolutionOptions {
    num_generations: usize, // the number of generations to cross
    log_level: u64,         // logging level to see how far the algorithm progressed
    num_parents: usize,     // the number of parents to grow a new generation
    num_children: usize,    // the number of phenotypes to breed per generation
}

impl EvolutionOptions {
    pub fn new() -> EvolutionOptions {
        EvolutionOptions {
            num_generations: 100,
            log_level: 0,
            num_parents: 2,
            num_children: 20,
        }
    }
}

#[derive(Clone, Copy)]
pub struct EvolutionCoordinator {
    current_generation: usize,
    num_generations: usize,
}

impl EvolutionCoordinator {
    pub fn new(options: EvolutionOptions) -> EvolutionCoordinator {
        EvolutionCoordinator {
            current_generation: 0,
            num_generations: options.num_generations,
        }
    }

    pub fn run(&mut self) {
        while self.current_generation < self.num_generations {
            self.current_generation += 1;
        }
    }

    pub fn get_progress(&self) -> f64 {
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

    pub fn from_seed(seed: u64) -> RandomNumberGenerator {
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

#[derive(Clone, Copy)]
pub struct EvolutionResult<Pheno>
where
    Pheno: Phenotype,
{
    winner: Pheno,
    score: f64,
}

pub trait Phenotype
where
    Self: Clone + Sized,
{
    fn crossover(&mut self, other: &Self);
    fn mutate(&mut self, rng: &mut RandomNumberGenerator, evol_coordinator: EvolutionCoordinator);
    fn to_string_internal(&self) -> String;
}

pub trait Challenge<Pheno>
where
    Pheno: Phenotype,
{
    fn score(&self, phenotype: Pheno, rng: &mut RandomNumberGenerator) -> f64;
    fn breed(
        &self,
        parents: Vec<Pheno>,
        rng: &mut RandomNumberGenerator,
        evol_coordinator: EvolutionCoordinator,
        evol_options: EvolutionOptions,
    ) -> Vec<Pheno>;
}

mod detail {
    use crate::evol::Challenge;
    use crate::evol::EvolutionCoordinator;
    use crate::evol::EvolutionOptions;
    use crate::evol::EvolutionResult;
    use crate::evol::Phenotype;
    use crate::evol::RandomNumberGenerator;

    pub fn breed<Pheno>(
        parents: Vec<Pheno>,
        rng: &mut RandomNumberGenerator,
        evol_coordinator: EvolutionCoordinator,
        evol_options: EvolutionOptions,
    ) -> Vec<Pheno>
    where
        Pheno: Phenotype,
    {
        let mut children = Vec::new();
        let winner_previous_generation = &parents[0];
        children.push(winner_previous_generation.clone());
        for i in 1..parents.len() {
            let mut child = winner_previous_generation.clone();
            child.crossover(&parents[i]);
            child.mutate(rng, evol_coordinator.clone());
            children.push(child);
        }
        for _ in parents.len()..evol_options.num_children {
            let mut child = winner_previous_generation.clone();
            child.mutate(rng, evol_coordinator.clone());
            children.push(child);
        }
        children
    }

    pub fn evolution<Pheno, Chall>(
        starting_value: Pheno,
        challenge: Chall,
        evol_options: EvolutionOptions,
        rng: &mut RandomNumberGenerator,
    ) -> EvolutionResult<Pheno>
    where
        Pheno: Phenotype,
        Chall: Challenge<Pheno>,
    {
        let mut evol_coordinator = EvolutionCoordinator::new(evol_options);
        let mut candidates: Vec<Pheno> = Vec::new();
        let mut fitness: Vec<EvolutionResult<Pheno>> = Vec::new();
        let mut parents: Vec<Pheno> = vec![starting_value];

        for _ in 0..evol_options.num_generations {
            evol_coordinator.run();
            candidates = challenge.breed(
                parents.clone(),
                rng,
                evol_coordinator.clone(),
                evol_options.clone(),
            );
            fitness.clear();
            for candidate in candidates.iter() {
                let score = challenge.score(candidate.clone(), rng);
                fitness.push(EvolutionResult::<Pheno> {
                    winner: candidate.clone(),
                    score: score,
                });
            }
            fitness.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
            if evol_options.log_level > 0 {
                println!("Generation: {}", evol_coordinator.current_generation);
                if evol_options.log_level > 1 {
                    for fit in fitness.iter() {
                        println!(
                            "Score {}: Phenotype: {}",
                            fit.score,
                            fit.winner.to_string_internal()
                        );
                    }
                }
            }
            parents.clear();
            let mut i = 0;
            for fit in fitness.iter() {
                parents.push(fit.winner.clone());
                if i >= evol_options.num_parents {
                    break;
                }
                i += 1;
            }
        }
        fitness[0].clone()
    }
}

#[cfg(test)]
#[path = "./test_evol.rs"]
mod test_evol;
