pub use self::skew::*;

use crate::*;

mod skew;

pub trait MutationMethod {
    fn mutate(&self, child: &mut Genome, rng: &mut dyn RngCore);
}
