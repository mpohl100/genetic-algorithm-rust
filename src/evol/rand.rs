use std::collections::VecDeque;

use rand::{rngs::ThreadRng, Rng};

pub struct RandomNumberGenerator {
    pub rd: ThreadRng,
}

impl RandomNumberGenerator {
    pub fn new() -> RandomNumberGenerator {
        RandomNumberGenerator {
            rd: rand::thread_rng(),
        }
    }

    pub fn from_seed(seed: u64) -> RandomNumberGenerator {
        RandomNumberGenerator {
            rd: rand::thread_rng(),
        }
    }

    pub fn fetch_uniform(&mut self, from: f32, to: f32, num: usize) -> VecDeque<f32> {
        let mut uniform_numbers = VecDeque::new();
        for _ in 0..num {
            uniform_numbers.push_back(self.rd.gen_range(from..to));
        }
        uniform_numbers
    }
}
