use crate::*;

#[derive(Clone, Debug)]
pub struct Brain {
    crate network: nn::Network,
}

impl Brain {
    crate fn random(config: &Config, rng: &mut dyn RngCore) -> Self {
        let network = nn::Network::random(rng, &Self::network_topology(config));

        Self { network }
    }

    crate fn from_chromosome(config: &Config, chromosome: ga::Chromosome) -> Self {
        let network = nn::Network::from_weights(&Self::network_topology(config), chromosome);

        Self { network }
    }

    crate fn chromosome(&self) -> ga::Chromosome {
        self.network.weights().collect()
    }

    crate fn step(&self, eye: &Eye) -> (f32, na::Rotation2<f32>) {
        let response = self.network.propagate(eye.energies.to_vec());
        let force_left = response[0].clamp(0.0, 1.0);
        let force_right = response[1].clamp(0.0, 1.0);

        let speed = (force_left + force_right).clamp(0.2, 1.0);
        let rotation = na::Rotation2::new(force_left - force_right);

        (speed, rotation)
    }

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
