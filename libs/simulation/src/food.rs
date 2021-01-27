use crate::*;

#[derive(Clone, Debug)]
pub struct Food {
    crate position: na::Vector2<f32>,
}

impl Food {
    pub fn position(&self) -> na::Vector2<f32> {
        self.position
    }
}

impl Food {
    crate fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            position: na::Vector2::new(rng.gen(), rng.gen()),
        }
    }

    crate fn reset(&mut self, rng: &mut dyn RngCore) {
        self.position = na::Vector2::new(rng.gen(), rng.gen());
    }
}
