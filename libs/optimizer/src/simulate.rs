use crate::{OptConfig, OptContext, OptLog, OptStatistics};
use itertools::iproduct;
use lib_simulation::{Config, Simulation};
use ordered_float::OrderedFloat;
use rand::seq::SliceRandom;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct SimulateCmd {
    output: PathBuf,
}

impl SimulateCmd {
    #[allow(clippy::approx_constant)]
    pub fn run(self) {
        const SIM_ITERATIONS: usize = 15;
        const SIM_GENERATIONS: usize = 30;

        let combinations = iproduct!(
            [2, 3, 5, 10],              // brain_neurons
            [0.1, 0.25, 0.33, 0.5],     // eye_fov_range
            [1.0, 2.0, 3.14, 6.0],      // eye_fov_angle
            [2, 3, 6, 9, 12],           // eye_cells
            [0.001, 0.01, 0.1, 0.5],    // ga_mut_chance
            [0.01, 0.1, 0.3, 0.5, 1.0]  // ga_mut_coeff
        );

        let combinations =
            combinations.flat_map(|config| (0..SIM_ITERATIONS).map(move |iter| (config, iter)));

        let mut combinations: Vec<_> = combinations.collect();

        combinations.shuffle(&mut rand::thread_rng());

        // ---

        let started_at = Instant::now();

        let is_done = AtomicBool::new(false);
        let done_steps = AtomicUsize::new(0);
        let total_steps = combinations.len() * SIM_GENERATIONS;

        let (log_tx, log_rx) = mpsc::channel();
        let mut output = BufWriter::new(File::create(self.output).unwrap());

        thread::scope(|s| {
            s.spawn({
                let is_done = &is_done;

                move || {
                    while let Ok(rx) = log_rx.recv() {
                        let rx = serde_json::to_string(&rx).unwrap();

                        writeln!(&mut output, "{}", rx).unwrap();
                    }

                    is_done.store(true, Ordering::SeqCst);
                }
            });

            // ---

            s.spawn(|| {
                while !is_done.load(Ordering::SeqCst) {
                    let done_steps = done_steps.load(Ordering::SeqCst);
                    let remaining_steps = total_steps - done_steps;

                    thread::sleep(Duration::from_secs(1));

                    let eta = if done_steps > 0 {
                        let avg_step_time =
                            (started_at.elapsed().as_secs() as f32) / (done_steps as f32);

                        (avg_step_time * (remaining_steps as f32)) as u32
                    } else {
                        0
                    };

                    println!("{} / {} done | eta[{}s]", done_steps, total_steps, eta);
                }
            });

            // ---

            let ctxt = (log_tx, &done_steps);

            combinations
                .into_par_iter()
                .for_each_with(ctxt, |ctxt, (config, iter)| {
                    let (log_tx, done_steps) = ctxt;

                    let opt_cfg = OptConfig {
                        brain_neurons: config.0,
                        eye_fov_range: OrderedFloat(config.1),
                        eye_fov_angle: OrderedFloat(config.2),
                        eye_cells: config.3,
                        ga_mut_chance: OrderedFloat(config.4),
                        ga_mut_coeff: OrderedFloat(config.5),
                    };

                    let config = Config {
                        brain_neurons: opt_cfg.brain_neurons as usize,
                        eye_fov_range: opt_cfg.eye_fov_range.0,
                        eye_fov_angle: opt_cfg.eye_fov_angle.0,
                        eye_cells: opt_cfg.eye_cells,
                        ga_mut_chance: opt_cfg.ga_mut_chance.0,
                        ga_mut_coeff: opt_cfg.ga_mut_coeff.0,
                        ..Default::default()
                    };

                    let mut rng = rand::thread_rng();
                    let mut sim = Simulation::random(config, &mut rng);

                    for gen in 0..SIM_GENERATIONS {
                        let stats = sim.train(&mut rng);

                        log_tx
                            .send(OptLog {
                                cfg: opt_cfg,
                                ctxt: OptContext { gen, iter },
                                stats: OptStatistics {
                                    min_fitness: stats.ga.min_fitness(),
                                    max_fitness: stats.ga.max_fitness(),
                                    avg_fitness: stats.ga.avg_fitness(),
                                    median_fitness: stats.ga.median_fitness(),
                                },
                            })
                            .unwrap();

                        done_steps.fetch_add(1, Ordering::SeqCst);
                    }
                });
        });

        println!();
        println!("completed; tt={:?}", started_at.elapsed());
    }
}
