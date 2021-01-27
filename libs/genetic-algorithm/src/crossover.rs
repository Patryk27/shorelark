pub use self::uniform::*;

use crate::*;

mod uniform;

pub trait CrossoverPolicy {
    fn crossover(&self, parent_a: &Genome, parent_b: &Genome, rng: &mut ChaCha8Rng) -> Genome;
}
