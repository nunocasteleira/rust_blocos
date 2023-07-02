/// Exercise Four
pub fn thunder_distance(seconds: u32) -> f64 {
    seconds as f64 * 0.34
}

#[cfg(test)]
mod tests {
    use float_eq::assert_float_eq;

    use super::*;

    #[test]
    fn test_thunder_distance() {
        let expected = 3.4;
        let seconds = 10;

        assert_float_eq!(expected, thunder_distance(seconds), abs <= 0.5);
    }

    #[test]
    fn other_test_thunder_distance() {
        let expected = 5.;
        let seconds = 14;

        assert_float_eq!(expected, thunder_distance(seconds), abs <= 0.5);
    }
}
