use super::{
    evol_options::{EvolutionOptions, PartialEvolutionOptions},
    ordinary_evol::{self, Challenge, EvolutionCoordinator, Phenotype, RandomNumberGenerator},
    partial_evol::{self, PartialChallenge, PartialPhenotype},
};

#[derive(Debug, Default, Copy, Clone)]
pub struct XCoordinate {
    x: f64,
}

impl XCoordinate {
    pub fn new(x: f64) -> XCoordinate {
        XCoordinate { x: x }
    }

    pub fn x(&self) -> f64 {
        self.x
    }
}

impl Phenotype for XCoordinate {
    fn crossover(&mut self, other: &Self)
    where
        Self: Sized,
    {
        self.x = (self.x + other.x) / 2.0;
    }

    fn mutate(&mut self, rng: &mut RandomNumberGenerator, evol_coordinator: EvolutionCoordinator) {
        let delta = rng.fetch_uniform(-100.0, 100.0, 1)[0] as f64;
        self.x += delta / 100.0;
    }

    fn to_string_internal(&self) -> String {
        format!("x: {}", self.x)
    }
}

impl PartialPhenotype for XCoordinate {
    fn magnitude(&self) -> f64 {
        self.x.abs()
    }
}

pub struct XCoordinateChallenge {
    target: f64,
    use_partial: bool,
}

impl XCoordinateChallenge {
    pub fn new(target: f64, use_patial: bool) -> XCoordinateChallenge {
        XCoordinateChallenge {
            target: target,
            use_partial: use_patial,
        }
    }
}

impl Challenge<XCoordinate, EvolutionOptions> for XCoordinateChallenge {
    fn score(&self, phenotype: XCoordinate, _rng: &mut RandomNumberGenerator) -> f64 {
        let x_coordinate = phenotype.x();
        let delta = x_coordinate - self.target;
        1.0 / (delta * delta)
    }

    fn breed(
        &self,
        parents: Vec<XCoordinate>,
        rng: &mut RandomNumberGenerator,
        evol_coordinator: EvolutionCoordinator,
        evol_options: &EvolutionOptions,
    ) -> Vec<XCoordinate> {
        if !self.use_partial {
            ordinary_evol::breed(parents, rng, evol_coordinator, evol_options)
        } else {
            Vec::new()
        }
    }
}

impl PartialChallenge<XCoordinate, PartialEvolutionOptions> for XCoordinateChallenge {
    fn score(&self, phenotype: XCoordinate, _rng: &mut RandomNumberGenerator) -> f64 {
        let x_coordinate = phenotype.x();
        let delta = x_coordinate - self.target;
        1.0 / (delta * delta)
    }
    fn breed(
        &self,
        parents: Vec<XCoordinate>,
        rng: &mut RandomNumberGenerator,
        evol_coordinator: EvolutionCoordinator,
        evol_options: &PartialEvolutionOptions,
    ) -> Vec<XCoordinate> {
        if self.use_partial {
            partial_evol::breed_partial(parents, rng, evol_coordinator, evol_options)
        } else {
            Vec::new()
        }
    }
}
