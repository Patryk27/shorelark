use crate::*;

#[derive(Debug)]
pub struct Animal {
    crate position: na::Point2<f32>,
    crate rotation: na::Rotation2<f32>,
    crate vision: Vec<f32>,
    crate speed: f32,
    crate eye: Eye,
    crate brain: Brain,
    crate satiation: usize,
}

impl Animal {
    pub fn position(&self) -> na::Point2<f32> {
        self.position
    }

    pub fn rotation(&self) -> na::Rotation2<f32> {
        self.rotation
    }

    pub fn vision(&self) -> &[f32] {
        &self.vision
    }
}

impl Animal {
    crate fn random(config: &Config, rng: &mut dyn RngCore) -> Self {
        let brain = Brain::random(config, rng);

        Self::new(config, rng, brain)
    }

    crate fn from_chromosome(
        config: &Config,
        rng: &mut dyn RngCore,
        chromosome: ga::Chromosome,
    ) -> Self {
        let brain = Brain::from_chromosome(config, chromosome);

        Self::new(config, rng, brain)
    }

    crate fn as_chromosome(&self) -> ga::Chromosome {
        self.brain.as_chromosome()
    }

    crate fn process_brain(&mut self, config: &Config, foods: &[Food]) {
        self.vision = self.eye.process_vision(self.position, self.rotation, foods);

        let (speed, rotation) = self.brain.propagate(self.vision.clone());

        self.speed = (self.speed + speed).clamp(config.sim_speed_min, config.sim_speed_max);
        self.rotation = na::Rotation2::new(self.rotation.angle() + rotation);
    }

    crate fn process_movement(&mut self) {
        self.position += self.rotation * na::Vector2::new(self.speed, 0.0);
        self.position.x = na::wrap(self.position.x, 0.0, 1.0);
        self.position.y = na::wrap(self.position.y, 0.0, 1.0);
    }
}

impl Animal {
    fn new(config: &Config, rng: &mut dyn RngCore, brain: Brain) -> Self {
        Self {
            position: rng.gen(),
            rotation: rng.gen(),
            vision: vec![0.0; config.eye_cells],
            speed: config.sim_speed_max,
            eye: Eye::new(config),
            brain,
            satiation: 0,
        }
    }
}
