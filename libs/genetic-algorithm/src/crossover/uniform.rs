use crate::*;

#[derive(Clone, Copy, Debug)]
pub struct UniformCrossover;

impl UniformCrossover {
    pub fn new() -> Self {
        Self
    }
}

impl CrossoverPolicy for UniformCrossover {
    fn crossover(&self, parent_a: &Genome, parent_b: &Genome, rng: &mut ChaCha8Rng) -> Genome {
        assert_eq!(parent_a.len(), parent_b.len());

        let parent_a = parent_a.iter();
        let parent_b = parent_b.iter();

        parent_a
            .zip(parent_b)
            .map(|(&a, &b)| if rng.gen_bool(0.5) { a } else { b })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use std::iter::FromIterator;

    #[test]
    fn test() {
        let parent_a = Genome::from_iter(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
        let parent_b = Genome::from_iter(vec![10.0, 20.0, 30.0, 40.0, 50.0]);
        let mut rng = ChaCha8Rng::from_seed(Default::default());

        let actual: Vec<_> = UniformCrossover::new()
            .crossover(&parent_a, &parent_b, &mut rng)
            .into_iter()
            .collect();

        let expected = vec![10.0, 20.0, 3.0, 4.0, 5.0];

        approx::assert_relative_eq!(actual.as_slice(), expected.as_slice());
    }
}
