use crate::*;
use std::iter::FromIterator;

#[derive(Clone, Debug)]
pub struct Brain {
    crate network: nn::Network,
}

impl Brain {
    crate fn random(config: &Config, rng: &mut ChaCha8Rng) -> Self {
        let network = nn::Network::random(&Self::network_topology(config), rng);

        Self { network }
    }

    crate fn from_genome(config: &Config, genome: ga::Genome) -> Self {
        let network = nn::Network::from_weights(&Self::network_topology(config), genome);

        Self { network }
    }

    crate fn genome(&self) -> ga::Genome {
        ga::Genome::from_iter(self.network.weights())
    }

    crate fn step(&mut self, eye: &Eye) -> (f32, na::Rotation2<f32>) {
        let response = self.network.propagate(&eye.energies);
        let force_left = response[0].clamp(0.0, 1.0);
        let force_right = response[1].clamp(0.0, 1.0);

        let speed = (force_left + force_right).clamp(0.2, 1.0);
        let rotation = na::Rotation2::new(force_left - force_right);

        (speed, rotation)
    }

    fn network_topology(config: &Config) -> [nn::LayerTopology; 3] {
        [
            nn::LayerTopology {
                size: config.eye_photoreceptors,
            },
            nn::LayerTopology {
                size: config.brain_neurons,
            },
            nn::LayerTopology { size: 2 },
        ]
    }
}
