use crate::*;

#[derive(Clone, Debug)]
pub struct Neuron {
    crate weights: Vec<f32>,
}

impl Neuron {
    pub fn new(weights: Vec<f32>) -> Self {
        Self { weights }
    }

    pub fn random(output_size: usize, rng: &mut ChaCha8Rng) -> Self {
        let weights = (0..output_size).map(|_| rng.gen_range(-1.0, 1.0)).collect();

        Self::new(weights)
    }

    pub fn propagate(&self, input: &[f32]) -> f32 {
        assert_eq!(input.len(), self.weights.len());

        input
            .iter()
            .map(|input| input.clamp(-1.0, 1.0))
            .zip(self.weights.iter())
            .map(|(input, weight)| input * weight)
            .sum::<f32>()
            .tanh()
    }
}

impl Neuron {
    crate fn from_weights(output_size: usize, weights: &mut dyn Iterator<Item = f32>) -> Self {
        let weights = (0..output_size)
            .map(|_| weights.next().expect("got not enough weights"))
            .collect();

        Self::new(weights)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod random {
        use super::*;
        use rand::SeedableRng;

        #[test]
        fn creates_neuron_with_random_weights() {
            let mut rng = ChaCha8Rng::from_seed(Default::default());
            let actual = Neuron::random(4, &mut rng).weights;
            let expected = vec![-0.6255188, 0.67383933, 0.81812596, 0.26284885];

            approx::assert_relative_eq!(actual.as_slice(), expected.as_slice());
        }
    }

    mod propagate {
        use super::*;

        #[test]
        fn returns_propagated_input() {
            let actual = Neuron::new(vec![-0.3, 0.6, 0.9]).propagate(&[0.5, -0.6, 0.7]);
            let expected = ((0.5f32 * -0.3f32) + (-0.6 * 0.6) + (0.7 * 0.9)).tanh();

            approx::assert_relative_eq!(actual, expected);
        }

        #[test]
        fn restricts_input() {
            let neuron = Neuron::new(vec![0.5]);
            let v1 = neuron.propagate(&[-1.5]);
            let v2 = neuron.propagate(&[-1.0]);
            let v3 = neuron.propagate(&[0.0]);
            let v4 = neuron.propagate(&[1.0]);
            let v5 = neuron.propagate(&[1.5]);

            approx::assert_relative_eq!(v1, v2);
            approx::assert_relative_ne!(v2, v3);
            approx::assert_relative_ne!(v3, v4);
            approx::assert_relative_eq!(v4, v5);
        }
    }

    mod from_weights {
        use super::*;

        #[test]
        fn restores_neuron_from_given_weights() {
            let actual = Neuron::from_weights(3, &mut vec![0.1, 0.2, 0.3].into_iter()).weights;
            let expected = vec![0.1, 0.2, 0.3];

            approx::assert_relative_eq!(actual.as_slice(), expected.as_slice());
        }
    }
}
