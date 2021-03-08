use crate::*;

#[derive(Clone, Copy, Debug)]
pub struct UniformCrossover;

impl UniformCrossover {
    pub fn new() -> Self {
        Self
    }
}

impl CrossoverMethod for UniformCrossover {
    fn crossover(
        &self,
        rng: &mut dyn RngCore,
        parent_a: &Chromosome,
        parent_b: &Chromosome,
    ) -> Chromosome {
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
    use rand_chacha::ChaCha8Rng;
    use std::iter::FromIterator;

    #[test]
    fn test() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let parent_a = Chromosome::from_iter((0..100).map(|n| n as f32));
        let parent_b = Chromosome::from_iter((100..200).map(|n| n as f32));

        let child: Vec<_> = UniformCrossover::new()
            .crossover(&mut rng, &parent_a, &parent_b)
            .into_iter()
            .collect();

        // Number of genes different between `child` and `parent_a`
        let diff_a = child
            .iter()
            .zip(parent_a.iter())
            .filter(|(&c, &p)| c != p)
            .count();

        // Number of genes different between `child` and `parent_b`
        let diff_b = child
            .iter()
            .zip(parent_b.iter())
            .filter(|(&c, &p)| c != p)
            .count();

        // Roughly looks like 50%, which proves that chance for picking either
        // gene *is* 50%
        assert_eq!(diff_a, 49);
        assert_eq!(diff_b, 51);
        assert_eq!(diff_a + diff_b, 100);
    }
}
