use super::traits::{EvolutionOptionsTrait, PartialEvolutionOptionsTrait};

#[derive(Clone, Copy)]
pub struct EvolutionOptions {
    num_generations: usize, // the number of generations to cross
    log_level: usize,       // logging level to see how far the algorithm progressed
    num_parents: usize,     // the number of parents to grow a new generation
    num_children: usize,    // the number of phenotypes to breed per generation
}

impl EvolutionOptions {
    pub fn new() -> EvolutionOptions {
        EvolutionOptions {
            num_generations: 100,
            log_level: 0,
            num_parents: 2,
            num_children: 20,
        }
    }
}

impl EvolutionOptionsTrait for EvolutionOptions {
    fn get_num_generations(&self) -> usize {
        self.num_generations
    }

    fn get_log_level(&self) -> usize {
        self.log_level
    }

    fn get_num_parents(&self) -> usize {
        self.num_parents
    }

    fn get_num_children(&self) -> usize {
        self.num_children
    }
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
            options,
            min_magnitude,
            max_magnitude,
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

    fn get_log_level(&self) -> usize {
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
