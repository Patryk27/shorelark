use crate::*;
use std::cmp::Ordering;
use std::f32::consts::{FRAC_PI_2, PI, TAU};

#[derive(Clone, Debug)]
pub struct Eye {
    crate energies: Vec<f32>,
}

// pub struct EyeConfig { } TODO

impl Eye {
    pub fn colors(&self) -> impl Iterator<Item = f32> + '_ {
        self.energies.iter().map(|&energy| (1.0 + energy) / 2.0)
    }
}

impl Eye {
    crate fn new(config: &Config) -> Self {
        Self {
            energies: vec![-1.0; config.eye_photoreceptors],
        }
    }

    crate fn step(
        &mut self,
        config: &Config,
        foods: &[Food],
        position: na::Vector2<f32>,
        rotation: na::Rotation2<f32>,
    ) {
        for energy in &mut self.energies {
            *energy = 0.0;
        }

        let rotation = na::wrap(rotation.angle() + FRAC_PI_2, 0.0, TAU);
        let angle_per_cell = (config.eye_fov_angle as f32) / (config.eye_photoreceptors as f32);

        for food in foods.iter() {
            let vec = food.position - position;
            let dist = vec.norm();
            let angle = na::wrap(vec.y.atan2(vec.x) - rotation, -PI, PI);

            if dist > config.eye_fov_distance {
                continue;
            }

            if angle <= -config.eye_fov_angle / 2.0 || angle >= config.eye_fov_angle / 2.0 {
                continue;
            }

            let cell_id =
                ((angle + (config.eye_fov_angle as f32) / 2.0) / angle_per_cell).trunc() as usize;

            if let Some(energy) = self.energies.get_mut(cell_id) {
                *energy += (config.eye_fov_distance - dist).powi(3);
            }
        }

        let max_energy = self
            .energies
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))
            .cloned()
            .unwrap_or_default();

        for energy in &mut self.energies {
            if max_energy > 0.0 {
                *energy /= max_energy;
            }

            *energy = -1.0 + 2.0 * *energy;
        }
    }
}
