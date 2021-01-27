pub use self::roulette_wheel::*;

use crate::*;

mod roulette_wheel;

pub trait SelectionPolicy {
    type Selector: Selector;

    fn init<I: Individual>(&self, population: &[I]) -> Self::Selector;
}

pub trait Selector {
    fn select<'a, I: Individual>(&self, population: &'a [I], rng: &mut dyn RngCore) -> &'a I;
}
