use rand::prelude::*;
use std::{collections::VecDeque, vec::Vec};

pub trait EvolutionOptionsTrait
where
    Self: Clone + Sized,
{
    fn get_num_generations(&self) -> usize;
    fn get_log_level(&self) -> u64;
    fn get_num_parents(&self) -> usize;
    fn get_num_children(&self) -> usize;
}

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

impl EvolutionOptionsTrait for EvolutionOptions {
    fn get_num_generations(&self) -> usize {
        self.num_generations
    }

    fn get_log_level(&self) -> u64 {
        self.log_level
    }

    fn get_num_parents(&self) -> usize {
        self.num_parents
    }

    fn get_num_children(&self) -> usize {
        self.num_children
    }
}

#[derive(Clone, Copy)]
pub struct EvolutionCoordinator {
    current_generation: usize,
    num_generations: usize,
}

impl EvolutionCoordinator {
    pub fn new<EvolOptions>(options: &EvolOptions) -> EvolutionCoordinator
    where
        EvolOptions: EvolutionOptionsTrait,
    {
        EvolutionCoordinator {
            current_generation: 0,
            num_generations: options.get_num_generations(),
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

    pub fn get_current_generation(&self) -> usize {
        self.current_generation
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

    pub fn fetch_uniform(&mut self, from: f32, to: f32, num: usize) -> VecDeque<f32> {
        let mut uniform_numbers = VecDeque::new();
        for _ in 0..num {
            uniform_numbers.push_back(self.rd.gen_range(from..to));
        }
        uniform_numbers
    }
}

#[derive(Clone, Copy)]
pub struct EvolutionResult<Pheno>
where
    Pheno: Phenotype,
{
    pub winner: Pheno,
    pub score: f64,
}

pub trait Phenotype
where
    Self: Clone + Sized,
{
    fn crossover(&mut self, other: &Self);
    fn mutate(&mut self, rng: &mut RandomNumberGenerator, evol_coordinator: EvolutionCoordinator);
    fn to_string_internal(&self) -> String;
}

pub trait Challenge<Pheno, EvolOptions>
where
    Pheno: Phenotype,
    EvolOptions: EvolutionOptionsTrait,
{
    fn score(&self, phenotype: Pheno, rng: &mut RandomNumberGenerator) -> f64;
    fn breed(
        &self,
        parents: Vec<Pheno>,
        rng: &mut RandomNumberGenerator,
        evol_coordinator: EvolutionCoordinator,
        evol_options: &EvolOptions,
    ) -> Vec<Pheno>;
}

pub mod detail {
    use crate::evol::ordinary_evol::Challenge;
    use crate::evol::ordinary_evol::EvolutionCoordinator;
    use crate::evol::ordinary_evol::EvolutionOptionsTrait;
    use crate::evol::ordinary_evol::EvolutionResult;
    use crate::evol::ordinary_evol::Phenotype;
    use crate::evol::ordinary_evol::RandomNumberGenerator;

    pub fn breed<Pheno, EvolOptions>(
        parents: Vec<Pheno>,
        rng: &mut RandomNumberGenerator,
        evol_coordinator: EvolutionCoordinator,
        evol_options: &EvolOptions,
    ) -> Vec<Pheno>
    where
        Pheno: Phenotype,
        EvolOptions: EvolutionOptionsTrait,
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
        for _ in parents.len()..evol_options.get_num_children() {
            let mut child = winner_previous_generation.clone();
            child.mutate(rng, evol_coordinator.clone());
            children.push(child);
        }
        children
    }

    pub fn evolution<Pheno, Chall, EvolOptions>(
        starting_value: Pheno,
        challenge: Chall,
        evol_options: EvolOptions,
        rng: &mut RandomNumberGenerator,
    ) -> EvolutionResult<Pheno>
    where
        Pheno: Phenotype,
        Chall: Challenge<Pheno, EvolOptions>,
        EvolOptions: EvolutionOptionsTrait,
    {
        let mut evol_coordinator = EvolutionCoordinator::new(&evol_options);
        let mut candidates: Vec<Pheno> = Vec::new();
        let mut fitness: Vec<EvolutionResult<Pheno>> = Vec::new();
        let mut parents: Vec<Pheno> = vec![starting_value];

        for _ in 0..evol_options.get_num_generations() {
            evol_coordinator.run();
            candidates = challenge.breed(
                parents.clone(),
                rng,
                evol_coordinator.clone(),
                &evol_options,
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
            if evol_options.get_log_level() > 0 {
                println!("Generation: {}", evol_coordinator.get_current_generation());
                if evol_options.get_log_level() > 1 {
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
                if i >= evol_options.get_num_parents() {
                    break;
                }
                i += 1;
            }
        }
        fitness[0].clone()
    }
}

#[cfg(test)]
mod ordinary_evol_test {
    use crate::evol::ordinary_evol::detail;
    use crate::evol::ordinary_evol::EvolutionOptions;
    use crate::evol::ordinary_evol::RandomNumberGenerator;
    use crate::evol::test_evol::XCoordinate;
    use crate::evol::test_evol::XCoordinateChallenge;
    #[test]
    fn test_evol() {
        let mut rng = RandomNumberGenerator::new();
        let challenge = XCoordinateChallenge::new(2.0, false);
        let starting_value = XCoordinate::new(0.0);
        let evol_options = EvolutionOptions::new();
        let winner = detail::evolution(starting_value, challenge, evol_options, &mut rng);
        assert!((winner.winner.x() - 2.0).abs() < 1e-2);
    }
}
