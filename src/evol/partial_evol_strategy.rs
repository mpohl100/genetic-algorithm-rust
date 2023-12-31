use super::{
    evol_coordinator::EvolutionCoordinator,
    rand::RandomNumberGenerator,
    traits::{EvolutionStrategy, PartialEvolutionOptionsTrait, PartialPhenotype},
};

pub struct PartialEvolutionStrategy;

impl<Pheno, EvolOptions> EvolutionStrategy<Pheno, EvolOptions> for PartialEvolutionStrategy
where
    Pheno: PartialPhenotype,
    EvolOptions: PartialEvolutionOptionsTrait,
{
    fn breed(
        &self,
        parents: Vec<Pheno>,
        rng: &mut RandomNumberGenerator,
        evol_coordinator: EvolutionCoordinator,
        evol_options: &EvolOptions,
    ) -> Vec<Pheno> {
        let mut develop = |pheno: Pheno, initial_mutate: bool| -> Option<Pheno> {
            let mut phenotype = pheno;
            if initial_mutate {
                phenotype.mutate(rng, evol_coordinator);
            }
            let pheno_type_in_range = |phenotype: &Pheno| -> bool {
                let magnitude = phenotype.magnitude();
                let min_magnitude = evol_options.get_min_magnitude();
                let max_magnitude = evol_options.get_max_magnitude();
                magnitude >= min_magnitude && magnitude <= max_magnitude
            };
            let mut advance_phenotype_n_times = |phenotype: Pheno, n: usize| -> Pheno {
                let mut phenotype = phenotype;
                for _ in 0..n {
                    phenotype.mutate(rng, evol_coordinator);
                    if pheno_type_in_range(&phenotype) {
                        break;
                    }
                }
                phenotype
            };
            let mut new_phenotype = phenotype.clone();
            for _ in 0..1000 {
                if pheno_type_in_range(&new_phenotype) {
                    return Some(new_phenotype);
                }
                new_phenotype = advance_phenotype_n_times(phenotype.clone(), 1000);
            }
            None
        };

        let mut children: Vec<Pheno> = Vec::new();
        let winner_previous_generation = parents[0];
        children.push(develop(winner_previous_generation, false).unwrap());
        for i in 1..parents.len() {
            let mut child = winner_previous_generation;
            child.crossover(&parents[i]);
            let mutated_child = develop(child, true).unwrap();
            children.push(mutated_child);
        }
        for _ in parents.len()..evol_options.get_num_children() {
            let child = winner_previous_generation;
            let mutated_child = develop(child, true).unwrap();
            children.push(mutated_child);
        }
        children.clone()
    }
}
