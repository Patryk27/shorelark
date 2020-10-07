//! This is a CLI application that runs `num_cpus::get()` different simulations
//! at the same time and later returns the best ones.
//!
//! TODO refactor, definitely

#![feature(drain_filter)]

use lib_simulation as sim;
use std::f32::consts::{FRAC_PI_4, PI};
use std::ops::Deref;
use std::sync::{mpsc, Arc};
use std::thread;
use std::time::{Duration, Instant};

struct Experiment {
    config: Arc<sim::Config>,
    rx: mpsc::Receiver<ExperimentNotification>,
    stats: Vec<sim::GenerationSummary>,
    started_at: Instant,
}

enum ExperimentNotification {
    GenerationCompleted { stats: sim::GenerationSummary },
    Completed,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct ExperimentStats {
    avg_fitness: usize,
    avg_max_fitness: usize,
    avg_min_fitness: usize,
    avg_sum_fitness: usize,
}

impl Experiment {
    pub fn start(config: sim::Config) -> Self {
        let config = Arc::new(config);
        let config2 = Arc::clone(&config);
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            for _ in 0..5 {
                let mut sim = sim::Engine::new(config.deref().to_owned());

                for _ in 0..20 {
                    sim.train();
                }

                for _ in 0..10 {
                    let stats = sim.train();

                    tx.send(ExperimentNotification::GenerationCompleted { stats })
                        .unwrap();
                }
            }

            tx.send(ExperimentNotification::Completed).unwrap();
        });

        Self {
            config: config2,
            rx,
            stats: Default::default(),
            started_at: Instant::now(),
        }
    }

    pub fn config(&self) -> &sim::Config {
        &self.config
    }

    pub fn completed(&mut self) -> bool {
        for msg in self.rx.try_iter() {
            match msg {
                ExperimentNotification::GenerationCompleted { stats } => {
                    self.stats.push(stats);
                }

                ExperimentNotification::Completed => {
                    return true;
                }
            }
        }

        false
    }

    pub fn elapsed(&self) -> Duration {
        self.started_at.elapsed()
    }

    pub fn stats(&self) -> ExperimentStats {
        let avg_sum_fitness = self
            .stats
            .iter()
            .map(|stat| stat.statistics.sum_fitness())
            .sum::<f32>()
            / (self.stats.len() as f32);

        let avg_max_fitness = self
            .stats
            .iter()
            .map(|stat| stat.statistics.max_fitness())
            .sum::<f32>()
            / (self.stats.len() as f32);

        let avg_min_fitness = self
            .stats
            .iter()
            .map(|stat| stat.statistics.min_fitness())
            .sum::<f32>()
            / (self.stats.len() as f32);

        let avg_fitness = self
            .stats
            .iter()
            .map(|stat| stat.statistics.avg_fitness())
            .sum::<f32>()
            / (self.stats.len() as f32);

        ExperimentStats {
            avg_sum_fitness: avg_sum_fitness as usize,
            avg_max_fitness: avg_max_fitness as usize,
            avg_min_fitness: avg_min_fitness as usize,
            avg_fitness: avg_fitness as usize,
        }
    }
}

fn main() {
    let cpus = num_cpus::get();

    println!("cpus = {}", cpus);
    println!();

    let mut queued = Vec::new();

    for brain_neurons in vec![2, 4, 6, 8, 10, 12] {
        for eye_photoreceptors in vec![3, 5, 7, 9, 11, 13] {
            for eye_fov_distance in vec![0.1, 0.2, 0.3] {
                for eye_fov_angle in vec![PI - FRAC_PI_4, PI, PI + FRAC_PI_4, 2.0 * PI] {
                    let config = sim::Config {
                        brain_neurons,
                        eye_photoreceptors,
                        eye_fov_distance,
                        eye_fov_angle,
                        ..Default::default()
                    };

                    queued.push(config);
                }
            }
        }
    }

    let mut pending = Vec::new();
    let mut completed = Vec::new();

    let mut elapsed_total = Duration::default();
    let mut elapsed_count = 0;

    while !queued.is_empty() || !pending.is_empty() {
        while pending.len() < cpus {
            if let Some(config) = queued.pop() {
                println!("started: {:?}", config);
                pending.push(Experiment::start(config));
            } else {
                break;
            }
        }

        let completed_now: Vec<_> = pending.drain_filter(Experiment::completed).collect();

        for experiment in completed_now {
            elapsed_total += experiment.elapsed();
            elapsed_count += 1;

            let elapsed = elapsed_total / elapsed_count;
            let remaining = (pending.len() + queued.len()) as u32;

            println!(
                "completed: {:?} | completed[{}] remaining[{}] | elapsed[{:?}] eta[{:#?}]",
                experiment.config,
                completed.len(),
                remaining,
                experiment.elapsed(),
                elapsed * remaining / (cpus as u32),
            );

            completed.push(experiment);
        }

        thread::sleep(Duration::from_millis(250));
    }

    completed.sort_by(|a, b| a.stats().cmp(&b.stats()));

    println!();
    println!("-- Summary --");
    println!();

    for experiment in completed {
        println!("Experiment:");
        println!("- config: {:?}", experiment.config());
        println!("- stats: {:?}", experiment.stats());
        println!();
    }
}
