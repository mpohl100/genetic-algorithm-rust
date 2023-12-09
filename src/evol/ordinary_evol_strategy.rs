use super::{
    evol_coordinator::EvolutionCoordinator,
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
        parents: Vec<Pheno>,
        rng: &mut RandomNumberGenerator,
        evol_coordinator: EvolutionCoordinator,
        evol_options: &EvolOptions,
    ) -> Vec<Pheno> {
        let mut children: Vec<Pheno> = Vec::new();
        let winner_previous_generation = parents[0];
        children.push(winner_previous_generation);
        for i in 1..parents.len() {
            let mut child = winner_previous_generation;
            child.crossover(&parents[i]);
            child.mutate(rng, evol_coordinator.clone());
            children.push(child);
        }
        for _ in parents.len()..evol_options.get_num_children() {
            let mut child = winner_previous_generation;
            child.mutate(rng, evol_coordinator.clone());
            children.push(child);
        }
        children.clone()
    }
}
