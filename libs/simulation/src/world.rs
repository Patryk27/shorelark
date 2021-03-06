use crate::*;

#[derive(Clone, Debug)]
pub struct World {
    crate animals: Vec<Animal>,
    crate foods: Vec<Food>,
}

impl World {
    pub fn animals(&self) -> &[Animal] {
        &self.animals
    }

    pub fn foods(&self) -> &[Food] {
        &self.foods
    }
}

impl World {
    crate fn random(config: &Config, rng: &mut dyn RngCore) -> Self {
        let animals = (0..config.animals)
            .map(|_| Animal::random(config, rng))
            .collect();

        let foods = (0..config.foods).map(|_| Food::random(rng)).collect();

        Self { animals, foods }
    }

    crate fn reset(&mut self, rng: &mut dyn RngCore, animals: Vec<Animal>) {
        for food in &mut self.foods {
            food.reset(rng);
        }

        self.animals = animals;
    }
}
