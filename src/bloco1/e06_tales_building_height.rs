use crate::exerciser::{retry_input, ExerciseConfig};

use super::e05_building_height::BuildingResult;

/// Exercise Six
pub struct TalesBuildingConfig {
    building_shadow_length: f64,
    person_shadow_length: f64,
    person_height: f64,
}

impl ExerciseConfig for TalesBuildingConfig {
    fn get_title() -> String {
        String::from("Let's calculate the height of a building using the theorem of Tales")
    }

    fn build(stdin: &std::io::Stdin) -> Self {
        let mut building_shadow_input = String::new();
        let mut person_shadow_input = String::new();
        let mut person_height_input = String::new();

        let building_shadow_length: f64 = retry_input(
            stdin,
            &mut building_shadow_input,
            "How many meters did the building's shadow measure?",
            "",
        );
        let person_shadow_length: f64 = retry_input(
            stdin,
            &mut person_shadow_input,
            "How many meters did the person's shadow measure?",
            "",
        );
        let person_height: f64 = retry_input(
            stdin,
            &mut person_height_input,
            "How tall is the person?",
            "",
        );

        TalesBuildingConfig {
            building_shadow_length,
            person_shadow_length,
            person_height,
        }
    }
}

pub fn tales_building_height(config: TalesBuildingConfig) -> BuildingResult {
    let TalesBuildingConfig {
        building_shadow_length,
        person_shadow_length,
        person_height,
    } = config;

    BuildingResult {
        height: (person_height * building_shadow_length) / person_shadow_length,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tales_building_height() {
        let expected = 20.;
        let building_shadow_length = 40.;
        let person_shadow_length = 4.;
        let person_height = 2.;

        let result = tales_building_height(TalesBuildingConfig {
            building_shadow_length,
            person_shadow_length,
            person_height,
        });

        assert_eq!(expected, result.height);
    }
}
