use super::{
    evol_coordinator::EvolutionCoordinator,
    rand::RandomNumberGenerator,
    traits::{PartialPhenotype, Phenotype},
};

#[derive(Debug, Default, Copy, Clone)]
pub struct XCoordinate {
    x: f64,
}

impl XCoordinate {
    pub fn new(x: f64) -> XCoordinate {
        XCoordinate { x }
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

    fn mutate(&mut self, rng: &mut RandomNumberGenerator) {
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
}

impl XCoordinateChallenge {
    pub fn new(target: f64) -> Self {
        Self { target }
    }

    pub fn score(&self, phenotype: XCoordinate) -> f64 {
        let x_coordinate = phenotype.x();
        let delta = x_coordinate - self.target;
        1.0 / (delta * delta)
    }
}
