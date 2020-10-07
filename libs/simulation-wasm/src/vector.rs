use crate::*;

#[derive(Clone, Copy, Debug, Serialize)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
}

impl From<na::Vector2<f32>> for Vector {
    fn from(vector: na::Vector2<f32>) -> Self {
        Self {
            x: vector.x,
            y: vector.y,
        }
    }
}
