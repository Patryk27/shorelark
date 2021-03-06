use crate::*;

pub trait Individual {
    fn create(genome: Genome) -> Self;
    fn genome(&self) -> &Genome;
    fn fitness(&self) -> f32;
}

#[cfg(test)]
#[derive(Clone, Debug)]
pub struct TestIndividual {
    fitness: f32,
}

#[cfg(test)]
impl TestIndividual {
    pub fn new(fitness: f32) -> Self {
        Self { fitness }
    }
}

#[cfg(test)]
impl Individual for TestIndividual {
    fn create(_: Genome) -> Self {
        panic!("not supported for TestIndividual")
    }

    fn genome(&self) -> &Genome {
        panic!("not supported for TestIndividual")
    }

    fn fitness(&self) -> f32 {
        self.fitness
    }
}
