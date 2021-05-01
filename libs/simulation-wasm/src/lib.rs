pub use self::{animal::*, config::*, food::*, point::*, world::*};

use lib_simulation as sim;
use nalgebra as na;
use rand::prelude::*;
use serde::Serialize;
use wasm_bindgen::prelude::*;

mod animal;
mod config;
mod food;
mod point;
mod world;

#[wasm_bindgen]
pub struct Simulation {
    rng: ThreadRng,
    sim: sim::Simulation,
}

#[wasm_bindgen]
impl Simulation {
    #[wasm_bindgen(constructor)]
    pub fn new(config: &Config) -> Self {
        let mut rng = thread_rng();
        let sim = sim::Simulation::new(config.into(), &mut rng);

        Self { rng, sim }
    }

    pub fn step(&mut self) -> String {
        self.sim
            .step(&mut self.rng)
            .map(|stats| stats.to_string())
            .unwrap_or_default()
    }

    pub fn train(&mut self) -> String {
        self.sim.train(&mut self.rng).to_string()
    }

    pub fn world(&self) -> JsValue {
        let world = World::from(self.sim.world());
        JsValue::from_serde(&world).unwrap()
    }
}
