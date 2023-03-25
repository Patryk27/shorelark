use crate::*;

#[derive(Debug)]
pub struct World {
    pub(crate) animals: Vec<Animal>,
    pub(crate) foods: Vec<Food>,
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
    pub(crate) fn random(config: &Config, rng: &mut dyn RngCore) -> Self {
        let animals = (0..config.world_animals)
            .map(|_| Animal::random(config, rng))
            .collect();

        let foods = (0..config.world_foods).map(|_| Food::random(rng)).collect();

        Self { animals, foods }
    }
}
