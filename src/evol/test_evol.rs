use crate::evol::ordinary_evol::detail;
use crate::evol::ordinary_evol::Challenge;
use crate::evol::ordinary_evol::EvolutionCoordinator;
use crate::evol::ordinary_evol::EvolutionOptions;
use crate::evol::ordinary_evol::Phenotype;
use crate::evol::ordinary_evol::RandomNumberGenerator;

#[derive(Debug, Default, Copy, Clone)]
struct XCoordinate {
    x: f64,
}

impl XCoordinate {
    fn new(x: f64) -> XCoordinate {
        XCoordinate { x: x }
    }

    fn x(&self) -> f64 {
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
        let delta = rng.fetch_uniform(-100, 100, 1)[0] as f64;
        self.x += delta / 100.0;
    }

    fn to_string_internal(&self) -> String {
        format!("x: {}", self.x)
    }
}

struct XCoordinateChallenge {
    target: f64,
}

impl XCoordinateChallenge {
    fn new(target: f64) -> XCoordinateChallenge {
        XCoordinateChallenge { target: target }
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
        detail::breed(parents, rng, evol_coordinator, evol_options)
    }
}

#[test]
fn test_evol() {
    let mut rng = RandomNumberGenerator::new();
    let challenge = XCoordinateChallenge::new(2.0);
    let starting_value = XCoordinate::new(0.0);
    let evol_options = EvolutionOptions::new();
    let winner = detail::evolution(starting_value, challenge, evol_options, &mut rng);
    assert!((winner.winner.x() - 2.0).abs() < 1e-2);
}
