use crate::exerciser::{retry_input, ExerciseConfig, ExerciseResult};

/// Exercise Four
pub struct ThunderConfig {
    seconds: u32,
}

impl ExerciseConfig for ThunderConfig {
    fn get_title() -> String {
        String::from("Let's calculate the distance of a thunder with the seconds it took to be listened.")
    }

    fn build(stdin: &std::io::Stdin) -> Self {
        let mut seconds_input = String::new();

        let seconds: u32 = retry_input(stdin, &mut seconds_input, "How many seconds?", "");

        ThunderConfig { seconds }
    }
}

pub struct ThunderResult {
    distance: f64,
}

impl ExerciseResult for ThunderResult {
    fn print_answer(&self) {
        println!("The thunder is at {}", self.distance)
    }
}

pub fn thunder_distance(config: ThunderConfig) -> ThunderResult {
    let ThunderConfig { seconds } = config;
    ThunderResult {
        distance: seconds as f64 * 0.34,
    }
}

#[cfg(test)]
mod tests {
    use float_eq::assert_float_eq;

    use super::*;

    #[test]
    fn test_thunder_distance() {
        let expected = 3.4;
        let seconds = 10;

        assert_float_eq!(
            expected,
            thunder_distance(ThunderConfig { seconds }).distance,
            abs <= 0.5
        );
    }

    #[test]
    fn other_test_thunder_distance() {
        let expected = 5.;
        let seconds = 14;

        assert_float_eq!(
            expected,
            thunder_distance(ThunderConfig { seconds }).distance,
            abs <= 0.5
        );
    }
}
