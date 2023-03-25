use crate::*;

#[derive(Debug)]
pub struct Eye {
    pub(crate) fov_range: f32,
    pub(crate) fov_angle: f32,
    pub(crate) cells: usize,
}

impl Eye {
    pub(crate) fn new(config: &Config) -> Self {
        Self::new_ex(config.eye_fov_range, config.eye_fov_angle, config.eye_cells)
    }

    pub(crate) fn process_vision(
        &self,
        position: na::Point2<f32>,
        rotation: na::Rotation2<f32>,
        foods: &[Food],
    ) -> Vec<f32> {
        let mut cells = vec![0.0; self.cells];

        for food in foods {
            let vec = food.position - position;
            let dist = vec.norm();

            if dist > self.fov_range {
                continue;
            }

            let angle = na::Rotation2::rotation_between(&na::Vector2::x(), &vec).angle();
            let angle = angle - rotation.angle();
            let angle = na::wrap(angle, -PI, PI);

            if angle < -self.fov_angle / 2.0 || angle > self.fov_angle / 2.0 {
                continue;
            }

            let angle = angle + self.fov_angle / 2.0;
            let cell = angle / self.fov_angle * (self.cells as f32);
            let cell = (cell as usize).min(cells.len() - 1);

            cells[cell] += (self.fov_range - dist) / self.fov_range;
        }

        cells
    }
}

impl Eye {
    fn new_ex(fov_range: f32, fov_angle: f32, cells: usize) -> Self {
        assert!(fov_range > 0.0);
        assert!(fov_angle > 0.0);
        assert!(cells > 0);

        Self {
            fov_range,
            fov_angle,
            cells,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_EYE_CELLS: usize = 13;

    fn food(x: f32, y: f32) -> Food {
        Food {
            position: na::Point2::new(x, y),
        }
    }

    struct TestCase {
        foods: Vec<Food>,
        fov_range: f32,
        fov_angle: f32,
        x: f32,
        y: f32,
        rot: f32,
        expected: &'static str,
    }

    impl TestCase {
        fn run(self) {
            let eye = Eye::new_ex(self.fov_range, self.fov_angle, TEST_EYE_CELLS);

            let actual = eye.process_vision(
                na::Point2::new(self.x, self.y),
                na::Rotation2::new(self.rot),
                &self.foods,
            );

            let actual = actual
                .into_iter()
                .map(|cell| {
                    if cell >= 0.7 {
                        "#"
                    } else if cell >= 0.3 {
                        "+"
                    } else if cell > 0.0 {
                        "."
                    } else {
                        " "
                    }
                })
                .collect::<Vec<_>>()
                .join("");

            assert_eq!(actual, self.expected);
        }
    }

    mod different_fov_ranges {
        use super::*;
        use test_case::test_case;

        #[test_case(1.0, "      +      ")]
        #[test_case(0.9, "      +      ")]
        #[test_case(0.8, "      +      ")]
        #[test_case(0.7, "      .      ")]
        #[test_case(0.6, "      .      ")]
        #[test_case(0.5, "             ")]
        #[test_case(0.4, "             ")]
        #[test_case(0.3, "             ")]
        #[test_case(0.2, "             ")]
        #[test_case(0.1, "             ")]
        fn test(fov_range: f32, expected: &'static str) {
            TestCase {
                foods: vec![food(1.0, 0.5)],
                fov_angle: FRAC_PI_2,
                x: 0.5,
                y: 0.5,
                rot: 0.0,
                fov_range,
                expected,
            }
            .run()
        }
    }

    mod different_fov_angles {
        use super::*;
        use test_case::test_case;

        #[test_case(0.25 * PI, " +         + ")]
        #[test_case(0.50 * PI, ".  +     +  .")]
        #[test_case(0.75 * PI, "  . +   + .  ")]
        #[test_case(1.00 * PI, "   . + + .   ")]
        #[test_case(1.25 * PI, "   . + + .   ")]
        #[test_case(1.50 * PI, ".   .+ +.   .")]
        #[test_case(1.75 * PI, ".   .+ +.   .")]
        #[test_case(2.00 * PI, "+.  .+ +.  .+")]
        fn test(fov_angle: f32, expected: &'static str) {
            TestCase {
                foods: vec![
                    food(0.0, 0.0),
                    food(0.0, 0.33),
                    food(0.0, 0.66),
                    food(0.0, 1.0),
                    food(1.0, 0.0),
                    food(1.0, 0.33),
                    food(1.0, 0.66),
                    food(1.0, 1.0),
                ],
                fov_range: 1.0,
                x: 0.5,
                y: 0.5,
                rot: 0.0,
                fov_angle,
                expected,
            }
            .run()
        }
    }

    mod different_positions {
        use super::*;
        use test_case::test_case;

        // Checking the X axis:
        #[test_case(0.9, 0.5, "#           #")]
        #[test_case(0.8, 0.5, "  #       #  ")]
        #[test_case(0.7, 0.5, "   +     +   ")]
        #[test_case(0.6, 0.5, "    +   +    ")]
        #[test_case(0.5, 0.5, "    +   +    ")]
        #[test_case(0.4, 0.5, "     + +     ")]
        #[test_case(0.3, 0.5, "     . .     ")]
        #[test_case(0.2, 0.5, "     . .     ")]
        #[test_case(0.1, 0.5, "     . .     ")]
        #[test_case(0.0, 0.5, "             ")]
        //
        // Checking the Y axis:
        #[test_case(0.5, 0.0, "            +")]
        #[test_case(0.5, 0.1, "          + .")]
        #[test_case(0.5, 0.2, "         +  +")]
        #[test_case(0.5, 0.3, "        + +  ")]
        #[test_case(0.5, 0.4, "      +  +   ")]
        #[test_case(0.5, 0.6, "   +  +      ")]
        #[test_case(0.5, 0.7, "  + +        ")]
        #[test_case(0.5, 0.8, "+  +         ")]
        #[test_case(0.5, 0.9, ". +          ")]
        #[test_case(0.5, 1.0, "+            ")]
        fn test(x: f32, y: f32, expected: &'static str) {
            TestCase {
                foods: vec![food(1.0, 0.4), food(1.0, 0.6)],
                fov_range: 1.0,
                fov_angle: FRAC_PI_2,
                rot: 0.0,
                x,
                y,
                expected,
            }
            .run()
        }
    }

    mod different_rotations {
        use super::*;
        use test_case::test_case;

        #[test_case(0.00 * PI, "         +   ")]
        #[test_case(0.25 * PI, "        +    ")]
        #[test_case(0.50 * PI, "      +      ")]
        #[test_case(0.75 * PI, "    +        ")]
        #[test_case(1.00 * PI, "   +         ")]
        #[test_case(1.25 * PI, " +           ")]
        #[test_case(1.50 * PI, "            +")]
        #[test_case(1.75 * PI, "           + ")]
        #[test_case(2.00 * PI, "         +   ")]
        #[test_case(2.25 * PI, "        +    ")]
        #[test_case(2.50 * PI, "      +      ")]
        fn test(rot: f32, expected: &'static str) {
            TestCase {
                foods: vec![food(0.5, 1.0)],
                fov_range: 1.0,
                fov_angle: 2.0 * PI,
                x: 0.5,
                y: 0.5,
                rot,
                expected,
            }
            .run()
        }
    }
}
