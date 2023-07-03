use std::f64::consts::PI;

use crate::exerciser::{retry_input, ExerciseConfig, ExerciseResult};

/// Exercise Three
pub struct CilinderConfig {
    radius: f64,
    height: f64,
}

impl ExerciseConfig for CilinderConfig {
    fn build(stdin: &std::io::Stdin) -> Self {
        let mut radius_input = String::new();
        let mut height_input = String::new();

        let radius: f64 = retry_input(stdin, &mut radius_input, "What is the radius?", "");
        let height: f64 = retry_input(stdin, &mut height_input, "What is the height?", "");

        CilinderConfig { height, radius }
    }
}

pub struct CilinderResult {
    volume: f64,
}

impl ExerciseResult for CilinderResult {
    fn print_answer(&self) {
        println!("The cylinder has a volume of {}", self.get_volume())
    }
}

impl CilinderResult {
    fn get_volume(&self) -> f64 {
        self.volume
    }
}

pub fn cilinder_volume(config: CilinderConfig) -> CilinderResult {
    let CilinderConfig { radius, height } = config;
    CilinderResult {
        volume: PI * radius.powi(2) * height,
    }
}

#[cfg(test)]
mod tests {
    use float_eq::assert_float_eq;

    use super::*;

    #[test]
    fn test_cilinder_volume() {
        let expected = 50.26;
        let radius = 2.;
        let height = 4.;

        let result = cilinder_volume(CilinderConfig { radius, height });
        assert_float_eq!(expected, result.get_volume(), abs <= 0.01);
    }

    #[test]
    fn another_test_cilinder_volume() {
        let expected = 565486.68;
        let radius = 60;
        let height = 50;

        let result = cilinder_volume(CilinderConfig {
            radius: radius as f64,
            height: height as f64,
        });
        assert_float_eq!(expected, result.get_volume(), abs <= 0.01);
    }
}
