mod layer;
mod layer_topology;
mod neuron;

use self::layer::*;
pub use self::layer_topology::*;
use self::neuron::*;
use rand::{Rng, RngCore};
use std::iter::once;

#[derive(Clone, Debug)]
pub struct Network {
    layers: Vec<Layer>,
}

impl Network {
    pub(crate) fn new(layers: Vec<Layer>) -> Self {
        Self { layers }
    }

    pub fn random(rng: &mut dyn RngCore, layers: &[LayerTopology]) -> Self {
        assert!(layers.len() > 1);

        let layers = layers
            .windows(2)
            .map(|layers| Layer::random(rng, layers[0].neurons, layers[1].neurons))
            .collect();

        Self::new(layers)
    }

    pub fn from_weights(layers: &[LayerTopology], weights: impl IntoIterator<Item = f32>) -> Self {
        assert!(layers.len() > 1);

        let mut weights = weights.into_iter();

        let layers = layers
            .windows(2)
            .map(|layers| Layer::from_weights(layers[0].neurons, layers[1].neurons, &mut weights))
            .collect();

        if weights.next().is_some() {
            panic!("got too many weights");
        }

        Self::new(layers)
    }

    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.layers
            .iter()
            .fold(inputs, |inputs, layer| layer.propagate(inputs))
    }

    pub fn weights(&self) -> impl Iterator<Item = f32> + '_ {
        self.layers
            .iter()
            .flat_map(|layer| layer.neurons.iter())
            .flat_map(|neuron| once(&neuron.bias).chain(&neuron.weights))
            .cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn random() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());

        let network = Network::random(
            &mut rng,
            &[
                LayerTopology { neurons: 3 },
                LayerTopology { neurons: 2 },
                LayerTopology { neurons: 1 },
            ],
        );

        assert_eq!(network.layers.len(), 2);
        assert_eq!(network.layers[0].neurons.len(), 2);

        assert_relative_eq!(network.layers[0].neurons[0].bias, -0.6255188);

        assert_relative_eq!(
            network.layers[0].neurons[0].weights.as_slice(),
            &[0.67383957, 0.8181262, 0.26284897].as_slice()
        );

        assert_relative_eq!(network.layers[0].neurons[1].bias, 0.5238807);

        assert_relative_eq!(
            network.layers[0].neurons[1].weights.as_slice(),
            &[-0.5351684, 0.069369555, -0.7648182].as_slice()
        );

        assert_eq!(network.layers[1].neurons.len(), 1);

        assert_relative_eq!(
            network.layers[1].neurons[0].weights.as_slice(),
            &[-0.48879623, -0.19277143].as_slice()
        );
    }

    #[test]
    fn from_weights() {
        let layers = &[LayerTopology { neurons: 3 }, LayerTopology { neurons: 2 }];
        let weights = vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8];

        let actual: Vec<_> = Network::from_weights(layers, weights.clone())
            .weights()
            .collect();

        assert_relative_eq!(actual.as_slice(), weights.as_slice());
    }

    #[test]
    fn propagate() {
        let layers = (
            Layer::new(vec![
                Neuron::new(0.0, vec![-0.5, -0.4, -0.3]),
                Neuron::new(0.0, vec![-0.2, -0.1, 0.0]),
            ]),
            Layer::new(vec![Neuron::new(0.0, vec![-0.5, 0.5])]),
        );
        let network = Network::new(vec![layers.0.clone(), layers.1.clone()]);

        let actual = network.propagate(vec![0.5, 0.6, 0.7]);
        let expected = layers.1.propagate(layers.0.propagate(vec![0.5, 0.6, 0.7]));

        assert_relative_eq!(actual.as_slice(), expected.as_slice());
    }

    #[test]
    fn weights() {
        let network = Network::new(vec![
            Layer::new(vec![Neuron::new(0.1, vec![0.2, 0.3, 0.4])]),
            Layer::new(vec![Neuron::new(0.5, vec![0.6, 0.7, 0.8])]),
        ]);

        let actual: Vec<_> = network.weights().collect();
        let expected = vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8];

        assert_relative_eq!(actual.as_slice(), expected.as_slice());
    }
}
