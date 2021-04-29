use crate::*;

#[derive(Clone, Debug)]
pub struct Brain {
    network: nn::Network,
}

impl Brain {
    crate fn random(config: &Config, rng: &mut dyn RngCore) -> Self {
        Self {
            network: nn::Network::random(rng, &Self::network_topology(config)),
        }
    }

    crate fn from_chromosome(config: &Config, chromosome: ga::Chromosome) -> Self {
        Self {
            network: nn::Network::from_weights(&Self::network_topology(config), chromosome),
        }
    }

    crate fn chromosome(&self) -> ga::Chromosome {
        self.network.weights().collect()
    }

    crate fn step(&self, eye: &Eye) -> (f32, f32) {
        let response = self.network.propagate(eye.energies.to_vec());
        let force_left = response[0].clamp(0.0, 1.0) - 0.5;
        let force_right = response[1].clamp(0.0, 1.0) - 0.5;

        let speed_delta = force_left + force_right;
        let rotation_delta = force_left - force_right;

        (speed_delta, rotation_delta)
    }
}

impl Brain {
    fn network_topology(config: &Config) -> [nn::LayerTopology; 3] {
        [
            nn::LayerTopology {
                neurons: config.eye_photoreceptors,
            },
            nn::LayerTopology {
                neurons: config.brain_neurons,
            },
            nn::LayerTopology { neurons: 2 },
        ]
    }
}
