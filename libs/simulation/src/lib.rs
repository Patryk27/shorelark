#![feature(crate_visibility_modifier)]

pub use self::{animal::*, brain::*, config::*, eye::*, food::*, generation_summary::*, world::*};

mod animal;
mod brain;
mod config;
mod eye;
mod food;
mod generation_summary;
mod world;

use lib_genetic_algorithm as ga;
use lib_neural_network as nn;
use nalgebra as na;
use rand::prelude::Rng;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

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
        struct GaAnimal {
            genome: ga::Genome,
            fitness: f32,
        }

        impl ga::Individual for GaAnimal {
            fn create(genome: ga::Genome) -> Self {
                Self {
                    genome,
                    fitness: 0.0,
                }
            }

            fn genome(&self) -> ga::Genome {
                self.genome.clone()
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
            let ga = ga::Engine::new(
                ga::UniformCrossover::new(),
                0.6,
                ga::SkewMutation::new(0.5, 0.5),
                0.2,
                ga::RouletteWheelSelection::new(),
            );

            let animals: Vec<_> = self
                .world
                .animals
                .iter()
                .map(|animal| GaAnimal {
                    genome: animal.brain.genome(),
                    fitness: animal.satiation as f32,
                })
                .collect();

            let (animals, statistics) = ga.iterate(&animals, &mut self.rng);

            let animals = animals
                .into_iter()
                .map(|animal| Animal::from_genome(&self.config, &mut self.rng, animal.genome))
                .collect();

            let summary = GenerationSummary {
                generation_idx: self.generation_idx,
                statistics,
            };

            self.world.reset(animals, &mut self.rng);
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
