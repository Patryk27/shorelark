use crate::*;

#[derive(Debug)]
pub struct Food {
    pub(crate) position: na::Point2<f32>,
}

impl Food {
    pub fn position(&self) -> na::Point2<f32> {
        self.position
    }
}

impl Food {
    pub(crate) fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            position: rng.gen(),
        }
    }
}
