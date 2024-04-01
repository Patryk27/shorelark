mod gaussian;

pub use self::gaussian::*;
use crate::*;

pub trait MutationMethod {
    fn mutate(&self, rng: &mut dyn RngCore, child: &mut Chromosome);
}
