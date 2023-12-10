use std::marker::PhantomData;

use super::{
    evol_coordinator::EvolutionCoordinator,
    rand::RandomNumberGenerator,
    traits::{EvolutionOptionsTrait, EvolutionStrategy, Phenotype},
};

#[derive(Clone)]
pub struct EvolutionResult<Pheno: Phenotype> {
    pub winner: Pheno,
    pub score: f64,
}

pub struct EvolutionLauncher<Pheno, EvolOptions, Strategy>
where
    Pheno: Phenotype,
    EvolOptions: EvolutionOptionsTrait,
    Strategy: EvolutionStrategy<Pheno, EvolOptions>,
{
    strategy: Strategy,
    score_fn: Box<dyn Fn(Pheno) -> f64>,
    _marker: PhantomData<(Pheno, EvolOptions)>,
}

impl<Pheno, EvolOptions, Strategy> EvolutionLauncher<Pheno, EvolOptions, Strategy>
where
    Pheno: Phenotype,
    EvolOptions: EvolutionOptionsTrait,
    Strategy: EvolutionStrategy<Pheno, EvolOptions>,
{
    pub fn new(strategy: Strategy, score_fn: Box<dyn Fn(Pheno) -> f64>) -> Self {
        Self {
            strategy,
            score_fn,
            _marker: PhantomData,
        }
    }

    pub fn evolve(
        &self,
        evol_options: EvolOptions,
        starting_value: Pheno,
        rng: &mut RandomNumberGenerator,
    ) -> EvolutionResult<Pheno> {
        let mut evol_coordinator = EvolutionCoordinator::new(&evol_options);
        let mut candidates: Vec<Pheno> = Vec::new();
        let mut fitness: Vec<EvolutionResult<Pheno>> = Vec::new();
        let mut parents: Vec<Pheno> = vec![starting_value];

        for _ in 0..evol_options.get_num_generations() {
            candidates.clear();
            evol_coordinator.run();
            candidates.extend(self.strategy.breed(&parents, rng, &evol_options));

            fitness.clear();
            candidates.iter().for_each(|candidate| {
                let score = (self.score_fn)(*candidate);
                fitness.push(EvolutionResult::<Pheno> {
                    winner: candidate.clone(),
                    score,
                })
            });

            fitness.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
            if evol_options.get_log_level() > 0 {
                println!("Generation: {}", evol_coordinator.get_current_generation());
                if evol_options.get_log_level() > 1 {
                    fitness.iter().for_each(|fit| {
                        println!(
                            "Score {}: Phenotype: {}",
                            fit.score,
                            fit.winner.to_string_internal()
                        )
                    });
                }
            }

            parents.clear();
            fitness
                .iter()
                .take(evol_options.get_num_parents())
                .for_each(|fit| {
                    parents.push(fit.winner.clone());
                });
        }
        fitness[0].clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::evol::{
        evol_options::{EvolutionOptions, PartialEvolutionOptions},
        ordinary_evol_strategy::OrdinaryEvolutionStrategy,
        partial_evol_strategy::PartialEvolutionStrategy,
        rand::RandomNumberGenerator,
        test_evol::{XCoordinate, XCoordinateChallenge},
    };

    use super::EvolutionLauncher;

    #[test]
    fn test_ordinary() {
        let mut rng = RandomNumberGenerator::new();
        let starting_value = XCoordinate::new(0.0);
        let evol_options = EvolutionOptions::new();
        let strategy = OrdinaryEvolutionStrategy;
        let challenge = XCoordinateChallenge::new(2.0);
        let launcher: EvolutionLauncher<XCoordinate, EvolutionOptions, OrdinaryEvolutionStrategy> =
            EvolutionLauncher::new(
                strategy,
                Box::new(move |phenotype: XCoordinate| challenge.score(phenotype)),
            );
        let winner = launcher.evolve(evol_options, starting_value, &mut rng);
        assert!((winner.winner.x() - 2.0).abs() < 1e-2);
    }

    #[test]
    fn test_partial() {
        let mut rng = RandomNumberGenerator::new();
        let starting_value = XCoordinate::new(7.0);
        let evol_options = PartialEvolutionOptions::new(EvolutionOptions::new(), 3.0, 10.0);
        let strategy = PartialEvolutionStrategy;
        let challenge = XCoordinateChallenge::new(2.0);
        let launcher: EvolutionLauncher<
            XCoordinate,
            PartialEvolutionOptions,
            PartialEvolutionStrategy,
        > = EvolutionLauncher::new(
            strategy,
            Box::new(move |phenotype: XCoordinate| challenge.score(phenotype)),
        );
        let winner = launcher.evolve(evol_options, starting_value, &mut rng);
        assert!((winner.winner.x() - 3.0).abs() < 1e-2);
    }
}
