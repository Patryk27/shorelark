use crate::*;
use std::fmt;

#[derive(Clone, Debug)]
pub struct GenerationSummary {
    pub generation_idx: usize,
    pub statistics: ga::Statistics,
}

impl fmt::Display for GenerationSummary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "generation #{}:\navg[{:.1}] max[{:.1}] min[{:.1}] sum[{:.1}]",
            self.generation_idx,
            self.statistics.avg_fitness(),
            self.statistics.max_fitness(),
            self.statistics.min_fitness(),
            self.statistics.sum_fitness(),
        )
    }
}
