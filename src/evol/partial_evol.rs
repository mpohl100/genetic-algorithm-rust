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

#[derive(Clone, Copy)]
pub struct EvolutionResult<Pheno>
where
    Pheno: PartialPhenotype,
{
    pub winner: Pheno,
    pub score: f64,
}

pub trait PartialPhenotype
where
    Self: Phenotype,
{
    fn magnitude(&self) -> f64;
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

pub mod detail {
    use crate::evol::ordinary_evol::EvolutionCoordinator;
    use crate::evol::ordinary_evol::RandomNumberGenerator;
    use crate::evol::partial_evol::EvolutionResult;
    use crate::evol::partial_evol::PartialChallenge;
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

    pub fn partial_evolution<Pheno, Chall, EvolOptions>(
        starting_value: Pheno,
        challenge: Chall,
        evol_options: EvolOptions,
        rng: &mut RandomNumberGenerator,
    ) -> EvolutionResult<Pheno>
    where
        Pheno: PartialPhenotype,
        Chall: PartialChallenge<Pheno, EvolOptions>,
        EvolOptions: PartialEvolutionOptionsTrait,
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
mod partial_evol_test {
    use crate::evol::ordinary_evol::EvolutionOptions;
    use crate::evol::ordinary_evol::RandomNumberGenerator;
    use crate::evol::partial_evol::detail;
    use crate::evol::partial_evol::PartialEvolutionOptions;
    use crate::evol::test_evol::XCoordinate;
    use crate::evol::test_evol::XCoordinateChallenge;
    #[test]
    fn test_evol() {
        let mut rng = RandomNumberGenerator::new();
        let challenge = XCoordinateChallenge::new(2.0, true);
        let starting_value = XCoordinate::new(7.0);
        let evol_options = PartialEvolutionOptions::new(EvolutionOptions::new(), 3.0, 10.0);
        let winner = detail::partial_evolution(starting_value, challenge, evol_options, &mut rng);
        assert!((winner.winner.x() - 3.0).abs() < 1e-2);
    }
}
