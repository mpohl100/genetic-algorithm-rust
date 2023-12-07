use std::vec::Vec;

pub use super::{
    evol_coordinator::EvolutionCoordinator,
    rand::RandomNumberGenerator,
    traits::{Challenge, EvolutionOptionsTrait, Phenotype},
};

#[derive(Clone, Copy)]
pub struct EvolutionResult<Pheno: Phenotype> {
    pub winner: Pheno,
    pub score: f64,
}
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
                score,
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
mod ordinary_evol_test {
    use crate::evol::evol_options::EvolutionOptions;
    use crate::evol::ordinary_evol::evolution;
    use crate::evol::ordinary_evol::RandomNumberGenerator;
    use crate::evol::test_evol::XCoordinate;
    use crate::evol::test_evol::XCoordinateChallenge;

    #[test]
    fn test_evol() {
        let mut rng = RandomNumberGenerator::new();
        let challenge = XCoordinateChallenge::new(2.0, false);
        let starting_value = XCoordinate::new(0.0);
        let evol_options = EvolutionOptions::new();
        let winner = evolution(starting_value, challenge, evol_options, &mut rng);
        assert!((winner.winner.x() - 2.0).abs() < 1e-2);
    }
}
