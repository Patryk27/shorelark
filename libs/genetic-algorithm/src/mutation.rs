pub use self::skew::*;

use crate::*;

mod skew;

pub trait MutationMethod {
    fn mutate(&self, rng: &mut dyn RngCore, child: &mut Genome);
}
