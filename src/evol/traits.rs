use super::rand::RandomNumberGenerator;

pub trait EvolutionStrategy<Pheno, EvolOptions>
where
    Pheno: Phenotype,
    EvolOptions: EvolutionOptionsTrait,
{
    fn breed(
        &self,
        parents: &Vec<Pheno>,
        rng: &mut RandomNumberGenerator,
        evol_options: &EvolOptions,
    ) -> Vec<Pheno>;
}

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
    Self: Clone + Copy + Sized,
{
    fn crossover(&mut self, other: &Self);
    fn mutate(&mut self, rng: &mut RandomNumberGenerator);
    fn to_string_internal(&self) -> String;
}

pub trait PartialPhenotype
where
    Self: Phenotype,
{
    fn magnitude(&self) -> f64;
}
