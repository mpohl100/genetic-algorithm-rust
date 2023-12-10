use std::cell::RefCell;

use super::{
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
        parents: &Vec<Pheno>,
        rng: &mut RandomNumberGenerator,
        evol_options: &EvolOptions,
    ) -> Vec<Pheno> {
        let mut develop = |pheno: Pheno, initial_mutate: bool| -> Option<Pheno> {
            let mut phenotype = pheno;
            if initial_mutate {
                phenotype.mutate(rng);
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
                    phenotype.mutate(rng);
                    if pheno_type_in_range(&phenotype) {
                        break;
                    }
                }
                phenotype
            };

            for _ in 0..1000 {
                if pheno_type_in_range(&phenotype) {
                    return Some(phenotype);
                }
                phenotype = advance_phenotype_n_times(phenotype, 1000);
            }
            None
        };

        let mut children: Vec<Pheno> = Vec::new();
        let mut winner_previous_generation = RefCell::new(parents[0]);
        children.push(develop(*winner_previous_generation.get_mut(), false).unwrap());

        parents.iter().skip(1).for_each(|parent| {
            let mut child = *winner_previous_generation.get_mut();
            child.crossover(parent);
            let mutated_child = develop(child, true).unwrap();
            children.push(mutated_child);
        });

        children.extend((parents.len()..evol_options.get_num_children()).map(|_| {
            let child = *winner_previous_generation.borrow_mut();
            let mutated_child = develop(child, true).unwrap();
            mutated_child
        }));

        children
    }
}
