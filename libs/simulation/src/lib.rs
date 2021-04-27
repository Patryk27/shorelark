#![feature(crate_visibility_modifier)]

pub use self::{animal::*, brain::*, config::*, eye::*, food::*, generation_summary::*, world::*};

use self::animal_indvidual::*;
use lib_genetic_algorithm as ga;
use lib_neural_network as nn;
use nalgebra as na;
use rand::{Rng, RngCore};

mod animal;
mod animal_indvidual;
mod brain;
mod config;
mod eye;
mod food;
mod generation_summary;
mod world;

pub struct Simulation {
    crate config: Config,
    crate ga: ga::GeneticAlgorithm<ga::RouletteWheelSelection>,
    crate world: World,
    crate step_idx: usize,
    crate generation_idx: usize,
}

impl Simulation {
    pub fn new(config: Config, rng: &mut dyn RngCore) -> Self {
        let ga = ga::GeneticAlgorithm::new(
            ga::RouletteWheelSelection::default(),
            ga::UniformCrossover::default(),
            ga::GaussianMutation::new(0.01, 0.3),
        );

        let world = World::random(&config, rng);

        Self {
            config,
            ga,
            world,
            step_idx: 0,
            generation_idx: 0,
        }
    }

    pub fn step(&mut self, rng: &mut dyn RngCore) -> Option<GenerationSummary> {
        self.step_idx += 1;
        self.step_process(rng);

        if self.step_idx >= self.config.generation_length {
            Some(self.step_evolve(rng))
        } else {
            None
        }
    }

    pub fn train(&mut self, rng: &mut dyn RngCore) -> GenerationSummary {
        loop {
            if let Some(stats) = self.step(rng) {
                break stats;
            }
        }
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    fn step_process(&mut self, rng: &mut dyn RngCore) {
        for animal in &mut self.world.animals {
            for food in &mut self.world.foods {
                if (food.position - animal.position).norm() < self.config.food_size {
                    animal.satiation += 1;
                    food.reset(rng);
                }
            }
        }

        for animal in &mut self.world.animals {
            animal.step(&self.config, &self.world.foods);
        }
    }

    fn step_evolve(&mut self, rng: &mut dyn RngCore) -> GenerationSummary {
        let individuals: Vec<_> = self
            .world
            .animals
            .iter()
            .map(AnimalIndividual::new)
            .collect();

        let (individuals, statistics) = self.ga.evolve(rng, &individuals);

        let animals = individuals
            .into_iter()
            .map(|individual| Animal::from_chromosome(&self.config, rng, individual.chromosome))
            .collect();

        let summary = GenerationSummary {
            generation_idx: self.generation_idx,
            statistics,
        };

        self.world.reset(rng, animals);
        self.step_idx = 0;
        self.generation_idx += 1;

        summary
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[test]
    #[ignore]
    fn smoke_test() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let mut sim = Simulation::new(Default::default(), &mut rng);

        let mut avg_fitness = 0.0;

        for _ in 0..10 {
            avg_fitness += sim.train(&mut rng).statistics.avg_fitness();
        }

        avg_fitness /= 10.0;

        assert!((25.0..30.0).contains(&avg_fitness));
    }
}
