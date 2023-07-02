/// Exercise Six
pub fn tales_building_height(
    building_shadow_length: f64,
    person_shadow_length: f64,
    person_height: f64,
) -> f64 {
    (person_height * building_shadow_length) / person_shadow_length
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

        let result =
            tales_building_height(building_shadow_length, person_shadow_length, person_height);

        assert_eq!(expected, result);
    }
}
