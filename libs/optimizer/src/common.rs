use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OptLog {
    pub cfg: OptConfig,
    pub ctxt: OptContext,
    pub stats: OptStatistics,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct OptConfig {
    #[serde(rename = "c")]
    pub brain_neurons: u8,

    #[serde(rename = "d")]
    pub eye_fov_range: OrderedFloat<f32>,

    #[serde(rename = "e")]
    pub eye_fov_angle: OrderedFloat<f32>,

    #[serde(rename = "f")]
    pub eye_cells: usize,

    #[serde(rename = "g")]
    pub ga_mut_chance: OrderedFloat<f32>,

    #[serde(rename = "h")]
    pub ga_mut_coeff: OrderedFloat<f32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OptContext {
    #[serde(rename = "g")]
    pub gen: usize,

    #[serde(rename = "i")]
    pub iter: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OptStatistics {
    #[serde(rename = "a")]
    pub min_fitness: f32,

    #[serde(rename = "b")]
    pub max_fitness: f32,

    #[serde(rename = "c")]
    pub avg_fitness: f32,

    #[serde(rename = "d")]
    pub median_fitness: f32,
}
