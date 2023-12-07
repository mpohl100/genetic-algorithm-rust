use super::{evol_coordinator::EvolutionCoordinator, rand::RandomNumberGenerator};

pub trait EvolutionOptionsTrait
where
    Self: Clone + Sized,
{
    fn get_num_generations(&self) -> usize;
    fn get_log_level(&self) -> usize;
    fn get_num_parents(&self) -> usize;
    fn get_num_children(&self) -> usize;
}

pub trait PartialEvolutionOptionsTrait
where
    Self: EvolutionOptionsTrait,
{
    fn get_min_magnitude(&self) -> f64;
    fn get_max_magnitude(&self) -> f64;
}

pub trait Phenotype
where
    Self: Clone + Sized,
{
    fn crossover(&mut self, other: &Self);
    fn mutate(&mut self, rng: &mut RandomNumberGenerator, evol_coordinator: EvolutionCoordinator);
    fn to_string_internal(&self) -> String;
}

pub trait PartialPhenotype
where
    Self: Phenotype,
{
    fn magnitude(&self) -> f64;
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

pub trait PartialChallenge<Pheno, EvolOptions>
where
    Pheno: PartialPhenotype,
    EvolOptions: PartialEvolutionOptionsTrait,
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
