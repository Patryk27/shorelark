use crate::*;

#[derive(Clone, Copy, Debug, Serialize)]
pub struct Point2 {
    pub x: f32,
    pub y: f32,
}

impl From<na::Point2<f32>> for Point2 {
    fn from(point: na::Point2<f32>) -> Self {
        Self {
            x: point.x,
            y: point.y,
        }
    }
}
