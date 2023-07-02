/// Exercise Five
pub fn building_height(seconds: u32) -> f64 {
    const ACCELERATION: f64 = 9.8;

    (ACCELERATION * seconds.pow(2) as f64) / 2.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_building_height() {
        let expected = 19.6;
        let seconds = 2;
        let result = building_height(seconds);

        assert_eq!(result, expected);
    }

    #[test]
    fn other_test_building_height() {
        let expected = 78.4;
        let seconds = 4;

        assert_eq!(building_height(seconds), expected);
    }
}
