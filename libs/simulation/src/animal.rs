use crate::*;
use std::f32::consts::{FRAC_PI_4, TAU};

#[derive(Clone, Debug)]
pub struct Animal {
    crate brain: Brain,
    crate eye: Eye,
    crate position: na::Vector2<f32>,
    crate rotation: na::Rotation2<f32>,
    crate speed: f32,
    crate satiation: usize,
}

impl Animal {
    pub fn eye(&self) -> &Eye {
        &self.eye
    }

    pub fn position(&self) -> na::Vector2<f32> {
        self.position
    }

    pub fn rotation(&self) -> na::Rotation2<f32> {
        self.rotation
    }

    pub fn direction(&self) -> na::Vector2<f32> {
        self.rotation * na::Vector2::new(0.0, 0.005 * self.speed)
    }
}

impl Animal {
    crate fn random(config: &Config, rng: &mut dyn RngCore) -> Self {
        let brain = Brain::random(config, rng);
        let eye = Eye::new(config);
        let position = na::Vector2::new(rng.gen(), rng.gen());
        let rotation = na::Rotation2::new(rng.gen_range(0.0..=TAU));
        let speed = 0.5;
        let satiation = 0;

        Self {
            brain,
            eye,
            position,
            rotation,
            speed,
            satiation,
        }
    }

    crate fn from_chromosome(
        config: &Config,
        rng: &mut dyn RngCore,
        chromosome: ga::Chromosome,
    ) -> Self {
        let brain = Brain::from_chromosome(config, chromosome);
        let eye = Eye::new(config);
        let position = na::Vector2::new(rng.gen(), rng.gen());
        let rotation = na::Rotation2::new(rng.gen_range(0.0..=TAU));
        let speed = 0.5;
        let satiation = 0;

        Self {
            brain,
            eye,
            position,
            rotation,
            speed,
            satiation,
        }
    }

    crate fn step(&mut self, config: &Config, foods: &[Food]) {
        self.eye.step(config, foods, self.position, self.rotation);

        let (speed, rotation) = self.brain.step(&self.eye);

        self.speed += {
            let delta = (speed - self.speed).clamp(-0.5, 0.5);
            delta * 0.33
        };

        self.rotation = {
            let delta = rotation.angle().clamp(-FRAC_PI_4, FRAC_PI_4);
            na::Rotation2::new(self.rotation.angle() + delta)
        };

        self.position += self.direction();
        self.position.x = na::wrap(self.position.x, 0.0, 1.0);
        self.position.y = na::wrap(self.position.y, 0.0, 1.0);
    }
}
