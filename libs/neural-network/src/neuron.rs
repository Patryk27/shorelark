use crate::*;

#[derive(Clone, Debug)]
pub struct Neuron {
    crate bias: f32,
    crate weights: Vec<f32>,
}

impl Neuron {
    pub fn new(bias: f32, weights: Vec<f32>) -> Self {
        Self { bias, weights }
    }

    pub fn random(output_size: usize, rng: &mut ChaCha8Rng) -> Self {
        let bias = rng.gen_range(-1.0, 1.0);
        let weights = (0..output_size).map(|_| rng.gen_range(-1.0, 1.0)).collect();

        Self::new(bias, weights)
    }

    pub fn propagate(&self, input: &[f32]) -> f32 {
        assert_eq!(input.len(), self.weights.len());

        let output = input
            .iter()
            .zip(&self.weights)
            .map(|(input, weight)| input * weight)
            .sum::<f32>();

        (self.bias + output).max(0.0)
    }
}

impl Neuron {
    crate fn from_weights(output_size: usize, weights: &mut dyn Iterator<Item = f32>) -> Self {
        let bias = weights.next().expect("got not enough weights");

        let weights = (0..output_size)
            .map(|_| weights.next().expect("got not enough weights"))
            .collect();

        Self::new(bias, weights)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod random {
        use super::*;
        use rand::SeedableRng;

        #[test]
        fn creates_neuron_with_random_bias_and_weights() {
            let mut rng = ChaCha8Rng::from_seed(Default::default());
            let neuron = Neuron::random(4, &mut rng);

            approx::assert_relative_eq!(neuron.bias, -0.6255188);

            approx::assert_relative_eq!(
                neuron.weights.as_slice(),
                [0.67383933, 0.81812596, 0.26284885, 0.5238805].as_slice(),
            );
        }
    }

    mod propagate {
        use super::*;

        #[test]
        fn returns_propagated_input() {
            let actual = Neuron::new(0.1, vec![-0.3, 0.6, 0.9]).propagate(&[0.5, -0.6, 0.7]);
            let expected = 0.1 + (0.5 * -0.3) + (-0.6 * 0.6) + (0.7 * 0.9);

            approx::assert_relative_eq!(actual, expected);
        }

        #[test]
        fn restricts_output() {
            let neuron = Neuron::new(0.0, vec![0.5]);
            let v1 = neuron.propagate(&[-1.0]);
            let v2 = neuron.propagate(&[-0.5]);
            let v3 = neuron.propagate(&[0.0]);
            let v4 = neuron.propagate(&[0.5]);
            let v5 = neuron.propagate(&[1.0]);

            approx::assert_relative_eq!(v1, v2);
            approx::assert_relative_eq!(v2, v3);
            approx::assert_relative_ne!(v3, v4);
            approx::assert_relative_ne!(v4, v5);
        }
    }

    mod from_weights {
        use super::*;

        #[test]
        fn restores_neuron_from_given_weights() {
            let actual = Neuron::from_weights(3, &mut vec![0.1, 0.2, 0.3, 0.4].into_iter());
            let expected = Neuron::new(0.1, vec![0.2, 0.3, 0.4]);

            approx::assert_relative_eq!(actual.bias, expected.bias);
            approx::assert_relative_eq!(actual.weights.as_slice(), expected.weights.as_slice());
        }
    }
}
