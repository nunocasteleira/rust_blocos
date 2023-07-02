use std::f64::consts::PI;

/// Exercise Three
pub fn cilinder_volume(radius: f64, height: f64) -> f64 {
    PI * radius.powi(2) * height
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

        let result = cilinder_volume(radius, height);
        assert_float_eq!(expected, result, abs <= 0.01);
    }

    #[test]
    fn another_test_cilinder_volume() {
        let expected = 565486.68;
        let radius = 60;
        let height = 50;

        let result = cilinder_volume(radius as f64, height as f64);
        assert_float_eq!(expected, result, abs <= 0.01);
    }
}
