use crate::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub brain_neurons: usize,

    pub eye_fov_range: f32,
    pub eye_fov_angle: f32,
    pub eye_cells: usize,

    pub food_size: f32,

    pub ga_reverse: usize,
    pub ga_mut_chance: f32,
    pub ga_mut_coeff: f32,

    pub sim_speed_min: f32,
    pub sim_speed_max: f32,
    pub sim_speed_accel: f32,
    pub sim_rotation_accel: f32,
    pub sim_generation_length: usize,

    pub world_animals: usize,
    pub world_foods: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            brain_neurons: 9,
            //
            eye_fov_range: 0.25,
            eye_fov_angle: PI + FRAC_PI_4,
            eye_cells: 9,
            //
            food_size: 0.01,
            //
            ga_reverse: 0,
            ga_mut_chance: 0.01,
            ga_mut_coeff: 0.3,
            //
            sim_speed_min: 0.001,
            sim_speed_max: 0.005,
            sim_speed_accel: 0.2,
            sim_rotation_accel: FRAC_PI_2,
            sim_generation_length: 2500,
            //
            world_animals: 40,
            world_foods: 60,
        }
    }
}
