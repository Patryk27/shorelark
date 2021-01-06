use crate::*;

#[derive(Clone, Debug)]
pub struct Layer {
    crate neurons: Vec<Neuron>,
    crate outputs: Vec<f32>,
}

impl Layer {
    pub fn new(neurons: Vec<Neuron>) -> Self {
        assert!(!neurons.is_empty());

        assert!(neurons
            .iter()
            .all(|neuron| neuron.weights.len() == neurons[0].weights.len()));

        Self {
            outputs: vec![0.0; neurons.len()],
            neurons,
        }
    }

    pub fn random(input_size: usize, output_size: usize, rng: &mut ChaCha8Rng) -> Self {
        let neurons = (0..output_size)
            .map(|_| Neuron::random(input_size, rng))
            .collect();

        Self::new(neurons)
    }

    pub fn propagate(&mut self, input: &[f32]) -> &[f32] {
        let neurons = self.neurons.iter().zip(self.outputs.iter_mut());

        for (neuron, output) in neurons {
            *output = neuron.propagate(input);
        }

        &self.outputs
    }
}

impl Layer {
    crate fn from_weights(
        input_size: usize,
        output_size: usize,
        weights: &mut dyn Iterator<Item = f32>,
    ) -> Self {
        let neurons = (0..output_size)
            .map(|_| Neuron::from_weights(input_size, weights))
            .collect();

        Self::new(neurons)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod random {
        use super::*;
        use rand::SeedableRng;

        #[test]
        fn creates_layer_with_random_neurons() {
            let mut rng = ChaCha8Rng::from_seed(Default::default());
            let layer = Layer::random(3, 2, &mut rng);

            let actual_biases: Vec<_> = layer.neurons.iter().map(|neuron| neuron.bias).collect();
            let expected_biases = vec![-0.6255188, 0.5238805];

            let actual_weights: Vec<_> = layer
                .neurons
                .iter()
                .map(|neuron| neuron.weights.as_slice())
                .collect();

            let expected_weights: Vec<&[f32]> = vec![
                &[0.67383933, 0.81812596, 0.26284885],
                &[-0.5351684, 0.069369555, -0.7648182],
            ];

            approx::assert_relative_eq!(actual_biases.as_slice(), expected_biases.as_slice());
            approx::assert_relative_eq!(actual_weights.as_slice(), expected_weights.as_slice());
        }
    }

    mod propagate {
        use super::*;

        #[test]
        fn returns_propagated_input() {
            let neurons = (
                Neuron::new(0.0, vec![0.1, 0.2, 0.3]),
                Neuron::new(0.0, vec![0.4, 0.5, 0.6]),
            );
            let input = &[-0.5, 0.0, 0.5];

            let mut layer = Layer::new(vec![neurons.0.clone(), neurons.1.clone()]);
            let actual = layer.propagate(input);
            let expected = vec![neurons.0.propagate(input), neurons.1.propagate(input)];

            approx::assert_relative_eq!(actual, expected.as_slice());
        }
    }

    mod from_weights {
        use super::*;

        #[test]
        fn restores_layer_from_given_weights() {
            let layer = Layer::from_weights(
                3,
                2,
                &mut vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8].into_iter(),
            );

            let actual_biases: Vec<_> = layer.neurons.iter().map(|neuron| neuron.bias).collect();
            let expected_biases = vec![0.1, 0.5];

            let actual_weights: Vec<_> = layer
                .neurons
                .iter()
                .map(|neuron| neuron.weights.as_slice())
                .collect();

            let expected_weights: Vec<&[f32]> = vec![&[0.2, 0.3, 0.4], &[0.6, 0.7, 0.8]];

            approx::assert_relative_eq!(actual_biases.as_slice(), expected_biases.as_slice());
            approx::assert_relative_eq!(actual_weights.as_slice(), expected_weights.as_slice());
        }
    }
}
