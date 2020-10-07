use crate::*;

#[derive(Clone, Debug)]
pub struct Statistics {
    crate min_fitness: f32,
    crate max_fitness: f32,
    crate avg_fitness: f32,
    crate sum_fitness: f32,
    crate diversity: f32,
}

impl Statistics {
    crate fn new<I: Individual>(population: &[I]) -> Self {
        assert!(!population.is_empty());

        let mut min_fitness = population[0].fitness();
        let mut max_fitness = min_fitness;
        let mut sum_fitness = 0.0;
        let mut diversity = 0.0;

        for individual in population {
            let fitness = individual.fitness();

            min_fitness = min_fitness.min(fitness);
            max_fitness = max_fitness.max(fitness);
            sum_fitness += fitness;
        }

        let genomes: Vec<_> = population
            .iter()
            .map(|individual| individual.genome())
            .collect();

        let genome_len = genomes[0].len();

        for gene_idx in 0..genome_len {
            let mut gene_diversity = 0.0;

            for genome_a in &genomes {
                for genome_b in &genomes {
                    gene_diversity += (genome_b[gene_idx] - genome_a[gene_idx]).abs();
                }
            }

            diversity += gene_diversity / (genomes.len() as f32);
        }

        diversity /= genome_len as f32;

        Self {
            min_fitness,
            max_fitness,
            avg_fitness: sum_fitness / (population.len() as f32),
            sum_fitness,
            diversity,
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

    pub fn sum_fitness(&self) -> f32 {
        self.sum_fitness
    }

    pub fn diversity(&self) -> f32 {
        self.diversity
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let stats = Statistics::new(&[
            TestIndividual::new(30.0),
            TestIndividual::new(10.0),
            TestIndividual::new(20.0),
            TestIndividual::new(40.0),
        ]);

        approx::assert_relative_eq!(stats.min_fitness(), 10.0);
        approx::assert_relative_eq!(stats.max_fitness(), 40.0);
        approx::assert_relative_eq!(stats.avg_fitness(), (10.0 + 20.0 + 30.0 + 40.0) / 4.0);
        approx::assert_relative_eq!(stats.sum_fitness(), 10.0 + 20.0 + 30.0 + 40.0);
    }
}
