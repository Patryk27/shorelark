pub use self::uniform::*;

use crate::*;

mod uniform;

pub trait CrossoverMethod {
    fn crossover(&self, parent_a: &Genome, parent_b: &Genome, rng: &mut dyn RngCore) -> Genome;
}
