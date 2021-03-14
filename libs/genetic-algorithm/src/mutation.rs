pub use self::gaussian::*;

use crate::*;

mod gaussian;

pub trait MutationMethod {
    fn mutate(&self, rng: &mut dyn RngCore, child: &mut Chromosome);
}
