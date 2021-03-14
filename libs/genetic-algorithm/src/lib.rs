#![feature(type_alias_impl_trait)]
#![feature(crate_visibility_modifier)]

pub use self::{
    chromosome::*, crossover::*, individual::*, mutation::*, selection::*, statistics::*,
};

use rand::seq::SliceRandom;
use rand::{Rng, RngCore};

mod chromosome;
mod crossover;
mod individual;
mod mutation;
mod selection;
mod statistics;

pub struct GeneticAlgorithm<S> {
    selection_method: S,
    crossover_method: Box<dyn CrossoverMethod>,
    mutation_method: Box<dyn MutationMethod>,
}

impl<S> GeneticAlgorithm<S>
where
    S: SelectionMethod,
{
    pub fn new(
        selection_method: S,
        crossover_method: impl CrossoverMethod + 'static,
        mutation_method: impl MutationMethod + 'static,
    ) -> Self {
        Self {
            selection_method,
            crossover_method: Box::new(crossover_method),
            mutation_method: Box::new(mutation_method),
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
            let parent_a = self.selection_method.select(rng, &population).chromosome();
            let parent_b = self.selection_method.select(rng, &population).chromosome();

            let mut child = self.crossover_method.crossover(rng, &parent_a, &parent_b);

            self.mutation_method.mutate(rng, &mut child);

            new_population.push(I::create(child));
        }

        (new_population, Statistics::new(population))
    }
}
