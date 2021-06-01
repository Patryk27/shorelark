#![feature(crate_visibility_modifier)]

pub use self::{animal::*, brain::*, config::*, eye::*, food::*, statistics::*, world::*};

mod animal;
mod animal_individual;
mod brain;
mod config;
mod eye;
mod food;
mod statistics;
mod world;

use self::animal_individual::*;
use lib_genetic_algorithm as ga;
use lib_neural_network as nn;
use nalgebra as na;
use rand::{Rng, RngCore};
use serde::{Deserialize, Serialize};
use std::f32::consts::*;

pub struct Simulation {
    config: Config,
    world: World,
    age: usize,
    generation: usize,
}

impl Simulation {
    pub fn random(config: Config, rng: &mut dyn RngCore) -> Self {
        let world = World::random(&config, rng);

        Self {
            config,
            world,
            age: 0,
            generation: 0,
        }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn step(&mut self, rng: &mut dyn RngCore) -> Option<Statistics> {
        self.process_collisions(rng);
        self.process_brains();
        self.process_movements();
        self.try_evolving(rng)
    }

    pub fn train(&mut self, rng: &mut dyn RngCore) -> Statistics {
        loop {
            if let Some(statistics) = self.step(rng) {
                return statistics;
            }
        }
    }
}

impl Simulation {
    fn process_collisions(&mut self, rng: &mut dyn RngCore) {
        for animal in &mut self.world.animals {
            for food in &mut self.world.foods {
                let distance = na::distance(&animal.position, &food.position);

                if distance <= self.config.food_size {
                    animal.satiation += 1;
                    food.position = rng.gen();
                }
            }
        }
    }

    fn process_brains(&mut self) {
        for animal in &mut self.world.animals {
            animal.process_brain(&self.config, &self.world.foods);
        }
    }

    fn process_movements(&mut self) {
        for animal in &mut self.world.animals {
            animal.process_movement();
        }
    }

    fn try_evolving(&mut self, rng: &mut dyn RngCore) -> Option<Statistics> {
        self.age += 1;

        if self.age > self.config.sim_generation_length {
            Some(self.evolve(rng))
        } else {
            None
        }
    }

    fn evolve(&mut self, rng: &mut dyn RngCore) -> Statistics {
        self.age = 0;
        self.generation += 1;

        let mut individuals: Vec<_> = self
            .world
            .animals
            .iter()
            .map(AnimalIndividual::from_animal)
            .collect();

        if self.config.ga_reverse == 1 {
            let max_satiation = self
                .world
                .animals
                .iter()
                .map(|animal| animal.satiation)
                .max()
                .unwrap_or_default();

            for individual in &mut individuals {
                individual.fitness = (max_satiation as f32) - individual.fitness;
            }
        }

        let ga = ga::GeneticAlgorithm::new(
            ga::RouletteWheelSelection::default(),
            ga::UniformCrossover::default(),
            ga::GaussianMutation::new(self.config.ga_mut_chance, self.config.ga_mut_coeff),
        );

        let (individuals, statistics) = ga.evolve(rng, &individuals);

        self.world.animals = individuals
            .into_iter()
            .map(|i| i.into_animal(&self.config, rng))
            .collect();

        for food in &mut self.world.foods {
            food.position = rng.gen();
        }

        Statistics {
            generation: self.generation - 1,
            ga: statistics,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[test]
    #[ignore]
    fn test() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let mut sim = Simulation::random(Default::default(), &mut rng);

        let avg_fitness = (0..10)
            .map(|_| sim.train(&mut rng).ga.avg_fitness())
            .sum::<f32>()
            / 10.0;

        approx::assert_relative_eq!(31.944998, avg_fitness);
    }
}
