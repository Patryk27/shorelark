pub use self::uniform::*;

mod uniform;

use crate::*;

pub trait CrossoverPolicy {
    fn crossover(&self, parent_a: &Genome, parent_b: &Genome, rng: &mut ChaCha8Rng) -> Genome;
}
