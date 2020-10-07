use crate::*;

#[derive(Clone, Copy, Debug, Serialize)]
pub struct Food {
    pub position: Vector,
}

impl From<&sim::Food> for Food {
    fn from(food: &sim::Food) -> Self {
        Self {
            position: Vector::from(food.position()),
        }
    }
}
