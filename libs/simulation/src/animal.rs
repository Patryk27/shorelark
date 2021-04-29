use crate::*;
use std::f32::consts::FRAC_PI_4;

const SPEED_MIN: f32 = 0.001;
const SPEED_MAX: f32 = 0.005;
const SPEED_FACTOR: f32 = 0.002;
const ROTATION_FACTOR: f32 = FRAC_PI_4;

#[derive(Clone, Debug)]
pub struct Animal {
    crate brain: Brain,
    crate eye: Eye,
    crate position: na::Point2<f32>,
    crate rotation: na::Rotation2<f32>,
    crate speed: f32,
    crate satiation: usize,
}

impl Animal {
    pub fn eye(&self) -> &Eye {
        &self.eye
    }

    pub fn position(&self) -> na::Point2<f32> {
        self.position
    }

    pub fn rotation(&self) -> na::Rotation2<f32> {
        self.rotation
    }
}

impl Animal {
    crate fn random(config: &Config, rng: &mut dyn RngCore) -> Self {
        let brain = Brain::random(config, rng);
        let eye = Eye::new(config);

        Self::new(rng, brain, eye)
    }

    crate fn from_chromosome(
        config: &Config,
        rng: &mut dyn RngCore,
        chromosome: ga::Chromosome,
    ) -> Self {
        let brain = Brain::from_chromosome(config, chromosome);
        let eye = Eye::new(config);

        Self::new(rng, brain, eye)
    }

    crate fn step(&mut self, config: &Config, foods: &[Food]) {
        self.eye.step(config, foods, self.position, self.rotation);
        let (speed_delta, rotation_delta) = self.brain.step(&self.eye);

        self.speed = (self.speed + speed_delta * SPEED_FACTOR).clamp(SPEED_MIN, SPEED_MAX);

        self.rotation =
            na::Rotation2::new(self.rotation.angle() + rotation_delta * ROTATION_FACTOR);

        self.position += self.rotation * na::Vector2::new(0.0, self.speed);
        self.position.x = na::wrap(self.position.x, 0.0, 1.0);
        self.position.y = na::wrap(self.position.y, 0.0, 1.0);
    }
}

impl Animal {
    fn new(rng: &mut dyn RngCore, brain: Brain, eye: Eye) -> Self {
        Self {
            brain,
            eye,
            position: rng.gen(),
            rotation: rng.gen(),
            speed: SPEED_MAX / 2.0,
            satiation: 0,
        }
    }
}
