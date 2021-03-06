#![feature(type_alias_impl_trait)]
#![feature(crate_visibility_modifier)]

pub use self::{crossover::*, genome::*, individual::*, mutation::*, selection::*, statistics::*};

use rand::seq::SliceRandom;
use rand::{Rng, RngCore};

mod crossover;
mod genome;
mod individual;
mod mutation;
mod selection;
mod statistics;

pub struct GeneticAlgorithm<S> {
    selection_method: S,
    crossover_method: Box<dyn CrossoverMethod>,
    crossover_probability: f32,
    mutation_method: Box<dyn MutationMethod>,
    mutation_probability: f32,
}

impl<S> GeneticAlgorithm<S>
where
    S: SelectionMethod,
{
    pub fn new(
        selection_method: S,
        crossover_method: impl CrossoverMethod + 'static,
        crossover_probability: f32,
        mutation_method: impl MutationMethod + 'static,
        mutation_probability: f32,
    ) -> Self {
        Self {
            selection_method,
            crossover_method: Box::new(crossover_method),
            crossover_probability,
            mutation_method: Box::new(mutation_method),
            mutation_probability,
        }
    }

    // TODO missing tests
    pub fn evolve<I>(&self, rng: &mut dyn RngCore, population: &[I]) -> (Vec<I>, Statistics)
    where
        I: Individual,
    {
        assert!(!population.is_empty());

        let mut new_population = Vec::with_capacity(population.len());

        while new_population.len() < population.len() {
            let parent_a = self.selection_method.select(rng, &population).genome();
            let parent_b = self.selection_method.select(rng, &population).genome();

            let mut child = if rng.gen_bool(self.crossover_probability as _) {
                self.crossover_method.crossover(rng, &parent_a, &parent_b)
            } else {
                if rng.gen_bool(0.5) {
                    parent_a
                } else {
                    parent_b
                }
            };

            if rng.gen_bool(self.mutation_probability as _) {
                self.mutation_method.mutate(rng, &mut child);
            }

            new_population.push(I::create(child));
        }

        (new_population, Statistics::new(population))
    }
}
