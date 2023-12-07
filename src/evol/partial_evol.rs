pub use super::{
    ordinary_evol::{
        EvolutionCoordinator, EvolutionOptionsTrait, Phenotype, RandomNumberGenerator,
    },
    traits::{PartialChallenge, PartialEvolutionOptionsTrait, PartialPhenotype},
};

#[derive(Clone, Copy)]
pub struct EvolutionResult<Pheno: PartialPhenotype> {
    pub winner: Pheno,
    pub score: f64,
}

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

#[cfg(test)]
mod partial_evol_test {
    use crate::evol::{
        evol_options::{EvolutionOptions, PartialEvolutionOptions},
        ordinary_evol::RandomNumberGenerator,
        partial_evol::partial_evolution,
        test_evol::{XCoordinate, XCoordinateChallenge},
    };

    #[test]
    fn test_evol() {
        let mut rng = RandomNumberGenerator::new();
        let challenge = XCoordinateChallenge::new(2.0, true);
        let starting_value = XCoordinate::new(7.0);
        let evol_options = PartialEvolutionOptions::new(EvolutionOptions::new(), 3.0, 10.0);
        let winner = partial_evolution(starting_value, challenge, evol_options, &mut rng);
        assert!((winner.winner.x() - 3.0).abs() < 1e-2);
    }
}
