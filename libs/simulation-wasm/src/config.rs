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
    #[allow(clippy::new_without_default)] // `impl Default` wouldn't get exported via `wasm_bindgen`
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

impl From<&Config> for sim::Config {
    fn from(config: &Config) -> Self {
        Self {
            animals: config.animals,
            brain_neurons: config.neurons,
            eye_photoreceptors: config.photoreceptors,
            foods: config.foods,
            ..Default::default()
        }
    }
}
