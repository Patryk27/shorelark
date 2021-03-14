#![feature(crate_visibility_modifier)]

pub use self::{animal::*, brain::*, config::*, eye::*, food::*, generation_summary::*, world::*};

use lib_genetic_algorithm as ga;
use lib_neural_network as nn;
use nalgebra as na;
use rand::{Rng, RngCore, SeedableRng};
use rand_chacha::ChaCha8Rng;

mod animal;
mod brain;
mod config;
mod eye;
mod food;
mod generation_summary;
mod world;

#[derive(Clone, Debug)]
pub struct Engine {
    crate rng: ChaCha8Rng,
    crate config: Config,
    crate world: World,
    crate step_idx: usize,
    crate generation_idx: usize,
}

impl Engine {
    pub fn new(config: Config) -> Self {
        let mut rng = ChaCha8Rng::from_entropy();
        let world = World::random(&config, &mut rng);

        Self {
            rng,
            config,
            world,
            step_idx: 0,
            generation_idx: 0,
        }
    }

    pub fn step(&mut self) -> Option<GenerationSummary> {
        struct Individual {
            chromosome: ga::Chromosome,
            fitness: f32,
        }

        impl ga::Individual for Individual {
            fn create(chromosome: ga::Chromosome) -> Self {
                Self {
                    chromosome,
                    fitness: 0.0,
                }
            }

            fn chromosome(&self) -> &ga::Chromosome {
                &self.chromosome
            }

            fn fitness(&self) -> f32 {
                self.fitness
            }
        }

        self.step_idx += 1;

        for animal in &mut self.world.animals {
            for food in &mut self.world.foods {
                if (&food.position - &animal.position).norm() < self.config.food_size {
                    animal.satiation += 1;
                    food.reset(&mut self.rng);
                }
            }
        }

        for animal in &mut self.world.animals {
            animal.step(&self.config, &self.world.foods);
        }

        if self.step_idx >= self.config.generation_length {
            let ga = ga::GeneticAlgorithm::new(
                ga::RouletteWheelSelection::new(),
                ga::UniformCrossover::new(),
                ga::GaussianMutation::new(0.01, 0.3),
            );

            let animals: Vec<_> = self
                .world
                .animals
                .iter()
                .map(|animal| Individual {
                    chromosome: animal.brain.chromosome(),
                    fitness: animal.satiation as f32,
                })
                .collect();

            let (animals, statistics) = ga.evolve(&mut self.rng, &animals);

            let animals = animals
                .into_iter()
                .map(|animal| {
                    Animal::from_chromosome(&self.config, &mut self.rng, animal.chromosome)
                })
                .collect();

            let summary = GenerationSummary {
                generation_idx: self.generation_idx,
                statistics,
            };

            self.world.reset(&mut self.rng, animals);
            self.step_idx = 0;
            self.generation_idx += 1;

            Some(summary)
        } else {
            None
        }
    }

    pub fn train(&mut self) -> GenerationSummary {
        loop {
            if let Some(stats) = self.step() {
                break stats;
            }
        }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn step_idx(&self) -> usize {
        self.step_idx
    }

    pub fn generation_idx(&self) -> usize {
        self.generation_idx
    }
}
