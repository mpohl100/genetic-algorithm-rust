use super::traits::EvolutionOptionsTrait;

#[derive(Clone, Copy)]
pub struct EvolutionCoordinator {
    pub current_generation: usize,
    pub num_generations: usize,
}

impl EvolutionCoordinator {
    pub fn new<EvolOptions>(options: &EvolOptions) -> EvolutionCoordinator
    where
        EvolOptions: EvolutionOptionsTrait,
    {
        EvolutionCoordinator {
            current_generation: 0,
            num_generations: options.get_num_generations(),
        }
    }

    pub fn run(&mut self) {
        while self.current_generation < self.num_generations {
            self.current_generation += 1;
        }
    }

    pub fn get_progress(&self) -> f64 {
        self.current_generation as f64 / self.num_generations as f64
    }

    pub fn get_current_generation(&self) -> usize {
        self.current_generation
    }
}
