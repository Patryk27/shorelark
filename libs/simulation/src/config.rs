use std::f32::consts::{FRAC_PI_4, PI};

#[derive(Clone, Debug)]
pub struct Config {
    pub animals: usize,
    pub brain_neurons: usize,
    pub eye_fov_angle: f32,
    pub eye_fov_distance: f32,
    pub eye_photoreceptors: usize,
    pub food_size: f32,
    pub foods: usize,
    pub generation_length: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            animals: 40,
            brain_neurons: 15,
            eye_photoreceptors: 9,
            eye_fov_angle: PI + FRAC_PI_4,
            eye_fov_distance: 0.25,
            foods: 60,
            food_size: 0.01,
            generation_length: 2500,
        }
    }
}
