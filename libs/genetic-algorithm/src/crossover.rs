pub use self::uniform::*;

use crate::*;

mod uniform;

pub trait CrossoverMethod {
    fn crossover(&self, rng: &mut dyn RngCore, parent_a: &Genome, parent_b: &Genome) -> Genome;
}
