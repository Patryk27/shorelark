use crate::*;
use std::cmp::Ordering;

#[derive(Clone, Debug)]
pub struct Statistics {
    min_fitness: f32,
    max_fitness: f32,
    avg_fitness: f32,
    median_fitness: f32,
}

impl Statistics {
    pub(crate) fn new<I>(population: &[I]) -> Self
    where
        I: Individual,
    {
        assert!(!population.is_empty());

        let len = population.len();

        let fitnesses = {
            let mut fitnesses: Vec<_> = population.iter().map(|i| i.fitness()).collect();
            fitnesses.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
            fitnesses
        };

        let min_fitness = fitnesses[0];
        let max_fitness = fitnesses[len - 1];
        let avg_fitness = fitnesses.iter().sum::<f32>() / (len as f32);

        let median_fitness = if len % 2 == 0 {
            (fitnesses[len / 2 - 1] + fitnesses[len / 2]) / 2.0
        } else {
            fitnesses[len / 2]
        };

        Self {
            min_fitness,
            max_fitness,
            avg_fitness,
            median_fitness,
        }
    }

    pub fn min_fitness(&self) -> f32 {
        self.min_fitness
    }

    pub fn max_fitness(&self) -> f32 {
        self.max_fitness
    }

    pub fn avg_fitness(&self) -> f32 {
        self.avg_fitness
    }

    pub fn median_fitness(&self) -> f32 {
        self.median_fitness
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_even() {
        let stats = Statistics::new(&[
            TestIndividual::new(30.0),
            TestIndividual::new(10.0),
            TestIndividual::new(20.0),
            TestIndividual::new(40.0),
        ]);

        approx::assert_relative_eq!(stats.min_fitness(), 10.0);
        approx::assert_relative_eq!(stats.max_fitness(), 40.0);
        approx::assert_relative_eq!(stats.avg_fitness(), (10.0 + 20.0 + 30.0 + 40.0) / 4.0);
        approx::assert_relative_eq!(stats.median_fitness(), (20.0 + 30.0) / 2.0);
    }

    #[test]
    fn test_odd() {
        let stats = Statistics::new(&[
            TestIndividual::new(30.0),
            TestIndividual::new(20.0),
            TestIndividual::new(40.0),
        ]);

        approx::assert_relative_eq!(stats.min_fitness(), 20.0);
        approx::assert_relative_eq!(stats.max_fitness(), 40.0);
        approx::assert_relative_eq!(stats.avg_fitness(), (20.0 + 30.0 + 40.0) / 3.0);
        approx::assert_relative_eq!(stats.median_fitness(), 30.0);
    }
}
