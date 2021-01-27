pub use self::skew::*;

use crate::*;

mod skew;

pub trait MutationPolicy {
    fn mutate(&self, child: &mut Genome, rng: &mut ChaCha8Rng);
}
