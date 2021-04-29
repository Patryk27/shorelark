use crate::*;

pub struct AnimalIndividual {
    pub chromosome: ga::Chromosome,
    pub fitness: f32,
}

impl AnimalIndividual {
    pub fn new(animal: &Animal) -> Self {
        Self {
            chromosome: animal.brain.chromosome(),
            fitness: animal.satiation as f32,
        }
    }
}

impl ga::Individual for AnimalIndividual {
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
