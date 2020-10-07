pub use self::{animal::*, config::*, food::*, vector::*, world::*};

use lib_simulation as sim;
use nalgebra as na;
use serde::Serialize;
use wasm_bindgen::prelude::*;

mod animal;
mod config;
mod food;
mod vector;
mod world;

#[wasm_bindgen]
pub struct Engine {
    engine: sim::Engine,
}

#[wasm_bindgen]
impl Engine {
    #[wasm_bindgen(constructor)]
    pub fn new(config: &Config) -> Self {
        let engine = sim::Engine::new(config.into());
        Self { engine }
    }

    pub fn step(&mut self) -> String {
        self.engine
            .step()
            .map(|stats| stats.to_string())
            .unwrap_or_default()
    }

    pub fn train(&mut self) -> String {
        self.engine.train().to_string()
    }

    pub fn world(&self) -> JsValue {
        let world = World::from(self.engine.world());
        JsValue::from_serde(&world).unwrap()
    }
}
