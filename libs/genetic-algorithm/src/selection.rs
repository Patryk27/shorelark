pub use self::roulette_wheel::*;

use crate::*;

mod roulette_wheel;

pub trait SelectionPolicy {
    fn select<'a, I: Individual>(&self, population: &'a [I], rng: &mut dyn RngCore) -> &'a I;
}
