use crate::*;

#[derive(Clone, Copy, Debug, Serialize)]
pub struct Food {
    pub position: Point2,
}

impl From<&sim::Food> for Food {
    fn from(food: &sim::Food) -> Self {
        Self {
            position: Point2::from(food.position()),
        }
    }
}
