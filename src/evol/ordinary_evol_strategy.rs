use std::cell::RefCell;

use super::{
    rand::RandomNumberGenerator,
    traits::{EvolutionOptionsTrait, EvolutionStrategy, Phenotype},
};

pub struct OrdinaryEvolutionStrategy;

impl<Pheno, EvolOptions> EvolutionStrategy<Pheno, EvolOptions> for OrdinaryEvolutionStrategy
where
    Pheno: Phenotype,
    EvolOptions: EvolutionOptionsTrait,
{
    fn breed(
        &self,
        parents: &Vec<Pheno>,
        rng: &mut RandomNumberGenerator,
        evol_options: &EvolOptions,
    ) -> Vec<Pheno> {
        let mut children: Vec<Pheno> = Vec::new();
        let mut winner_previous_generation = RefCell::new(parents[0]);
        children.push(*winner_previous_generation.get_mut());

        parents.iter().skip(1).for_each(|parent| {
            let mut child = *winner_previous_generation.get_mut();
            child.crossover(parent);
            child.mutate(rng);
            children.push(child);
        });

        children.extend((parents.len()..evol_options.get_num_children()).map(|_| {
            let mut child = *winner_previous_generation.get_mut();
            child.mutate(rng);
            child
        }));

        children
    }
}
