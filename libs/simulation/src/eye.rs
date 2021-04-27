use crate::*;
use std::cmp::Ordering;
use std::f32::consts::{FRAC_PI_2, PI, TAU};

#[derive(Clone, Debug)]
pub struct Eye {
    crate energies: Box<[f32]>,
}

impl Eye {
    pub fn energies(&self) -> &[f32] {
        &self.energies
    }
}

impl Eye {
    crate fn new(config: &Config) -> Self {
        Self {
            energies: vec![-1.0; config.eye_photoreceptors].into_boxed_slice(),
        }
    }

    crate fn step(
        &mut self,
        config: &Config,
        foods: &[Food],
        position: na::Point2<f32>,
        rotation: na::Rotation2<f32>,
    ) {
        for energy in self.energies.iter_mut() {
            *energy = 0.0;
        }

        let rotation = na::wrap(rotation.angle() + FRAC_PI_2, 0.0, TAU);

        let fov_dist = config.eye_fov_distance;
        let fov_angle_half = config.eye_fov_angle / 2.0;
        let fov_angle_per_cell = config.eye_fov_angle / (config.eye_photoreceptors as f32);

        for food in foods.iter() {
            let vec = food.position - position;
            let dist = vec.norm();
            let angle = na::wrap(vec.y.atan2(vec.x) - rotation, -PI, PI);

            if dist >= fov_dist {
                continue;
            }

            if angle <= -fov_angle_half || angle >= fov_angle_half {
                continue;
            }

            let cell_id = ((angle + fov_angle_half) / fov_angle_per_cell).trunc() as usize;

            if let Some(energy) = self.energies.get_mut(cell_id) {
                *energy += (fov_dist - dist).powi(3);
            }
        }

        let max_energy = self
            .energies
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Less))
            .cloned()
            .unwrap_or_default();

        if max_energy > 0.0 {
            for energy in self.energies.iter_mut() {
                *energy /= max_energy;
            }
        }
    }
}
