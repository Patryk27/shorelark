use crate::*;

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Animal {
    pub x: f32,
    pub y: f32,
    pub rotation: f32,

    #[wasm_bindgen(getter_with_clone)]
    pub vision: Vec<f32>,
}

impl From<&sim::Animal> for Animal {
    fn from(animal: &sim::Animal) -> Self {
        Self {
            x: animal.position().x,
            y: animal.position().y,
            rotation: animal.rotation().angle(),
            vision: animal.vision().to_owned(),
        }
    }
}
