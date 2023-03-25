use crate::*;

pub struct AnimalIndividual {
    pub(crate) fitness: f32,
    pub(crate) chromosome: ga::Chromosome,
}

impl AnimalIndividual {
    pub fn from_animal(animal: &Animal) -> Self {
        Self {
            fitness: animal.satiation as f32,
            chromosome: animal.as_chromosome(),
        }
    }

    pub fn into_animal(self, config: &Config, rng: &mut dyn RngCore) -> Animal {
        Animal::from_chromosome(config, rng, self.chromosome)
    }
}

impl ga::Individual for AnimalIndividual {
    fn create(chromosome: ga::Chromosome) -> Self {
        Self {
            fitness: 0.0,
            chromosome,
        }
    }

    fn chromosome(&self) -> &ga::Chromosome {
        &self.chromosome
    }

    fn fitness(&self) -> f32 {
        self.fitness
    }
}
