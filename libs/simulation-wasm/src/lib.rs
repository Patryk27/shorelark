mod animal;
mod food;
mod world;

pub use self::animal::*;
pub use self::food::*;
pub use self::world::*;
use lib_simulation as sim;
use rand::prelude::*;
use serde::Serialize;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Simulation {
    rng: ThreadRng,
    sim: sim::Simulation,
}

#[wasm_bindgen]
impl Simulation {
    #[wasm_bindgen(constructor)]
    pub fn new(config: JsValue) -> Self {
        let config: sim::Config = serde_wasm_bindgen::from_value(config).unwrap();
        let mut rng = thread_rng();
        let sim = sim::Simulation::random(config, &mut rng);

        Self { rng, sim }
    }

    pub fn default_config() -> JsValue {
        serde_wasm_bindgen::to_value(&sim::Config::default()).unwrap()
    }

    pub fn config(&self) -> JsValue {
        serde_wasm_bindgen::to_value(self.sim.config()).unwrap()
    }

    pub fn world(&self) -> World {
        World::from(self.sim.world())
    }

    pub fn step(&mut self) -> Option<String> {
        self.sim.step(&mut self.rng).map(|stats| stats.to_string())
    }

    pub fn train(&mut self) -> String {
        self.sim.train(&mut self.rng).to_string()
    }
}
