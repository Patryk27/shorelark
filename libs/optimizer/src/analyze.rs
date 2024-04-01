use crate::{OptLog, OptStatistics};
use std::collections::{btree_map, BTreeMap};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct AnalyzeCmd {
    input: PathBuf,
}

impl AnalyzeCmd {
    pub fn run(self) {
        let input = BufReader::new(File::open(self.input).unwrap());
        let mut results = BTreeMap::<_, (usize, OptStatistics)>::new();

        for line in input.lines() {
            let line = line.unwrap();
            let log: OptLog = serde_json::from_str(&line).unwrap();

            match results.entry(log.cfg) {
                btree_map::Entry::Occupied(entry) => {
                    let (entry_samples, entry_stats) = entry.into_mut();

                    *entry_samples += 1;
                    entry_stats.min_fitness += log.stats.min_fitness;
                    entry_stats.max_fitness += log.stats.max_fitness;
                    entry_stats.avg_fitness += log.stats.avg_fitness;
                    entry_stats.median_fitness += log.stats.median_fitness;
                }

                btree_map::Entry::Vacant(entry) => {
                    entry.insert((1, log.stats));
                }
            }
        }

        // ---

        print!("brain_neurons");
        print!(",eye_fov_range");
        print!(",eye_fov_angle");
        print!(",eye_cells");
        print!(",ga_mut_chance");
        print!(",ga_mut_coeff");
        print!(",min_fitness");
        print!(",max_fitness");
        print!(",avg_fitness");
        print!(",median_fitness");
        println!();

        for (config, (samples, mut stats)) in results {
            let samples = samples as f32;

            stats.min_fitness /= samples;
            stats.max_fitness /= samples;
            stats.avg_fitness /= samples;
            stats.median_fitness /= samples;

            print!("{}", config.brain_neurons);
            print!(",{}", config.eye_fov_range);
            print!(",{}", config.eye_fov_angle);
            print!(",{}", config.eye_cells);
            print!(",{}", config.ga_mut_chance);
            print!(",{}", config.ga_mut_coeff);
            print!(",{}", stats.min_fitness);
            print!(",{}", stats.max_fitness);
            print!(",{}", stats.avg_fitness);
            print!(",{}", stats.median_fitness);
            println!();
        }
    }
}
