pub use self::roulette_wheel::*;

mod roulette_wheel;

use crate::*;

pub trait SelectionPolicy {
    type Selector: Selector;

    fn init<I: Individual>(&self, population: &[I]) -> Self::Selector;
}

pub trait Selector {
    fn select<'a, I: Individual>(&self, population: &'a [I], rng: &mut ChaCha8Rng) -> &'a I;
}
