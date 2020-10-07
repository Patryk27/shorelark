use crate::*;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct Config {
    animals: usize,
    foods: usize,
    neurons: usize,
    photoreceptors: usize,
}

#[wasm_bindgen]
impl Config {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let default = sim::Config::default();

        Self {
            animals: default.animals,
            photoreceptors: default.eye_photoreceptors,
            foods: default.foods,
            neurons: default.brain_neurons,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn animals(&self) -> usize {
        self.animals
    }

    #[wasm_bindgen(setter)]
    pub fn set_animals(&mut self, animals: usize) {
        self.animals = animals;
    }

    #[wasm_bindgen(getter)]
    pub fn foods(&self) -> usize {
        self.foods
    }

    #[wasm_bindgen(setter)]
    pub fn set_foods(&mut self, foods: usize) {
        self.foods = foods;
    }

    #[wasm_bindgen(getter)]
    pub fn neurons(&self) -> usize {
        self.neurons
    }

    #[wasm_bindgen(setter)]
    pub fn set_neurons(&mut self, neurons: usize) {
        self.neurons = neurons;
    }

    #[wasm_bindgen(getter)]
    pub fn photoreceptors(&self) -> usize {
        self.photoreceptors
    }

    #[wasm_bindgen(setter)]
    pub fn set_photoreceptors(&mut self, photoreceptors: usize) {
        self.photoreceptors = photoreceptors;
    }
}

impl Into<sim::Config> for &Config {
    fn into(self) -> sim::Config {
        sim::Config {
            animals: self.animals,
            brain_neurons: self.neurons,
            eye_photoreceptors: self.photoreceptors,
            foods: self.foods,
            ..Default::default()
        }
    }
}
