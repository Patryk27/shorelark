mod roulette_wheel;

pub use self::roulette_wheel::*;
use crate::*;

pub trait SelectionMethod {
    fn select<'a, I>(&self, rng: &mut dyn RngCore, population: &'a [I]) -> &'a I
    where
        I: Individual;
}
