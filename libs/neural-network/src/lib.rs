#![feature(array_methods)]
#![feature(crate_visibility_modifier)]

pub use self::{layer::*, layer_topology::*, neuron::*};

use rand::Rng;
use rand_chacha::ChaCha8Rng;
use std::iter::once;

mod layer;
mod layer_topology;
mod neuron;

#[derive(Clone, Debug)]
pub struct Network {
    layers: Vec<Layer>,
}

impl Network {
    pub fn new(layers: Vec<Layer>) -> Self {
        Self { layers }
    }

    pub fn random(layers: &[LayerTopology], rng: &mut ChaCha8Rng) -> Self {
        assert!(layers.len() > 1);

        let layers = layers
            .windows(2)
            .map(|layers| Layer::random(layers[0].size, layers[1].size, rng))
            .collect();

        Self::new(layers)
    }

    pub fn from_weights(layers: &[LayerTopology], weights: impl IntoIterator<Item = f32>) -> Self {
        assert!(layers.len() > 1);

        let mut weights = weights.into_iter();

        let layers = layers
            .windows(2)
            .map(|layers| Layer::from_weights(layers[0].size, layers[1].size, &mut weights))
            .collect();

        if weights.next().is_some() {
            panic!("got too many weights");
        }

        Self::new(layers)
    }

    pub fn propagate<'a>(&'a mut self, input: &'a [f32]) -> &'a [f32] {
        self.layers
            .iter_mut()
            .fold(input, |input, layer| layer.propagate(input))
    }

    pub fn weights(&self) -> impl Iterator<Item = f32> + '_ {
        self.layers
            .iter()
            .flat_map(|layer| layer.neurons.iter())
            .flat_map(|neuron| once(neuron.bias).chain(neuron.weights.iter().cloned()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod random {
        use super::*;
        use rand_chacha::rand_core::SeedableRng;

        #[test]
        fn creates_network_with_random_neurons() {
            let mut rng = ChaCha8Rng::from_seed(Default::default());

            let network = Network::random(
                &[
                    LayerTopology { size: 3 },
                    LayerTopology { size: 2 },
                    LayerTopology { size: 1 },
                ],
                &mut rng,
            );

            assert_eq!(network.layers.len(), 2);
            assert_eq!(network.layers[0].neurons.len(), 2);

            approx::assert_relative_eq!(network.layers[0].neurons[0].bias, -0.6255188);

            approx::assert_relative_eq!(
                network.layers[0].neurons[0].weights.as_slice(),
                &[0.67383933, 0.81812596, 0.26284885].as_slice()
            );

            approx::assert_relative_eq!(network.layers[0].neurons[1].bias, 0.5238805);

            approx::assert_relative_eq!(
                network.layers[0].neurons[1].weights.as_slice(),
                &[-0.5351684, 0.069369555, -0.7648182].as_slice()
            );

            assert_eq!(network.layers[1].neurons.len(), 1);

            approx::assert_relative_eq!(
                network.layers[1].neurons[0].weights.as_slice(),
                &[-0.48879623, -0.19277143].as_slice()
            );
        }
    }

    mod from_weights {
        use super::*;

        #[test]
        fn restores_network_from_given_weights() {
            let layers = &[LayerTopology { size: 3 }, LayerTopology { size: 2 }];
            let weights = vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8];
            let network = Network::from_weights(layers, weights);

            let actual: Vec<_> = network.weights().collect();
            let expected = vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8];

            approx::assert_relative_eq!(actual.as_slice(), expected.as_slice());
        }
    }

    mod propagate {
        use super::*;

        #[test]
        fn returns_propagated_input() {
            let mut layers = (
                Layer::new(vec![
                    Neuron::new(0.0, vec![-0.5, -0.4, -0.3]),
                    Neuron::new(0.0, vec![-0.2, -0.1, 0.0]),
                ]),
                Layer::new(vec![Neuron::new(0.0, vec![-0.5, 0.5])]),
            );

            let mut network = Network::new(vec![layers.0.clone(), layers.1.clone()]);
            let actual = network.propagate(&[0.5, 0.6, 0.7]);
            let expected = layers.1.propagate(layers.0.propagate(&[0.5, 0.6, 0.7]));

            approx::assert_relative_eq!(actual, expected);
        }
    }

    mod weights {
        use super::*;

        #[test]
        fn returns_weights() {
            let network = Network::new(vec![
                Layer::new(vec![Neuron::new(0.1, vec![0.2, 0.3, 0.4])]),
                Layer::new(vec![Neuron::new(0.5, vec![0.6, 0.7, 0.8])]),
            ]);

            let actual: Vec<_> = network.weights().collect();
            let expected = vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8];

            approx::assert_relative_eq!(actual.as_slice(), expected.as_slice());
        }
    }
}
