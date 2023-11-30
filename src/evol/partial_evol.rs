use crate::evol::ordinary_evol::EvolutionCoordinator;
use crate::evol::ordinary_evol::EvolutionOptions;
use crate::evol::ordinary_evol::EvolutionOptionsTrait;
use crate::evol::ordinary_evol::Phenotype;
use crate::evol::ordinary_evol::RandomNumberGenerator;

pub trait PartialEvolutionOptionsTrait
where
    Self: EvolutionOptionsTrait,
{
    fn get_min_magnitude(&self) -> f64;
    fn get_max_magnitude(&self) -> f64;
}

#[derive(Clone, Copy)]
pub struct PartialEvolutionOptions {
    options: EvolutionOptions,
    min_magnitude: f64,
    max_magnitude: f64,
}

impl PartialEvolutionOptions {
    pub fn new(
        options: EvolutionOptions,
        min_magnitude: f64,
        max_magnitude: f64,
    ) -> PartialEvolutionOptions {
        PartialEvolutionOptions {
            options: options,
            min_magnitude: min_magnitude,
            max_magnitude: max_magnitude,
        }
    }
}

impl EvolutionOptionsTrait for PartialEvolutionOptions {
    fn get_num_children(&self) -> usize {
        self.options.get_num_children()
    }

    fn get_num_parents(&self) -> usize {
        self.options.get_num_parents()
    }

    fn get_log_level(&self) -> u64 {
        self.options.get_log_level()
    }

    fn get_num_generations(&self) -> usize {
        self.options.get_num_generations()
    }
}

impl PartialEvolutionOptionsTrait for PartialEvolutionOptions {
    fn get_min_magnitude(&self) -> f64 {
        self.min_magnitude
    }

    fn get_max_magnitude(&self) -> f64 {
        self.max_magnitude
    }
}

pub trait PartialPhenotype
where
    Self: Phenotype,
{
    fn magnitude(&self) -> f64;
}

mod detail {
    use crate::evol::ordinary_evol::EvolutionCoordinator;
    use crate::evol::ordinary_evol::RandomNumberGenerator;
    use crate::evol::partial_evol::PartialEvolutionOptionsTrait;
    use crate::evol::partial_evol::PartialPhenotype;

    pub fn breed_partial<Pheno, EvolOptions>(
        parents: Vec<Pheno>,
        rng: &mut RandomNumberGenerator,
        evol_coordinator: EvolutionCoordinator,
        evol_options: &EvolOptions,
    ) -> Vec<Pheno>
    where
        Pheno: PartialPhenotype,
        EvolOptions: PartialEvolutionOptionsTrait,
    {
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

        // actual breed implementation
        let mut children = Vec::new();
        let winner_previous_generation = &parents[0];
        children.push(develop(winner_previous_generation.clone(), false).unwrap());
        for i in 1..parents.len() {
            let mut child = winner_previous_generation.clone();
            child.crossover(&parents[i]);
            let mutated_child = develop(child, true).unwrap();
            children.push(mutated_child);
        }
        for _ in parents.len()..evol_options.get_num_children() {
            let child = winner_previous_generation.clone();
            let mutated_child = develop(child, true).unwrap();
            children.push(mutated_child);
        }
        children
    }
}
