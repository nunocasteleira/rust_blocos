use crate::exerciser::{retry_input, ExerciseConfig, ExerciseResult};

/// Exercise Five
pub struct BuildingConfig {
    seconds: u32,
}

impl ExerciseConfig for BuildingConfig {
    fn get_title() -> String {
        String::from("Let's calculate the height of a building bu throwing a rock from its roof and counting the seconds it took to land.")
    }

    fn build(stdin: &std::io::Stdin) -> Self {
        let mut seconds_input = String::new();

        let seconds: u32 = retry_input(stdin, &mut seconds_input, "How many seconds?", "");

        BuildingConfig { seconds }
    }
}

pub struct BuildingResult {
    pub(crate) height: f64,
}

impl ExerciseResult for BuildingResult {
    fn print_answer(&self) {
        println!("The building has the height of {}", self.height)
    }
}

pub fn building_height(config: BuildingConfig) -> BuildingResult {
    const ACCELERATION: f64 = 9.8;

    BuildingResult {
        height: (ACCELERATION * config.seconds.pow(2) as f64) / 2.,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_building_height() {
        let expected = 19.6;
        let seconds = 2;
        let result = building_height(BuildingConfig { seconds });

        assert_eq!(expected, result.height);
    }

    #[test]
    fn other_test_building_height() {
        let expected = 78.4;
        let seconds = 4;
        let result = building_height(BuildingConfig { seconds });

        assert_eq!(expected, result.height);
    }
}
