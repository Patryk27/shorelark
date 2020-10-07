pub use self::skew::*;

mod skew;

use crate::*;

pub trait MutationPolicy {
    fn mutate(&self, child: &mut Genome, rng: &mut ChaCha8Rng);
}
