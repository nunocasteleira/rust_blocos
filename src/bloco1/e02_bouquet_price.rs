/// Exercise Two
pub fn bouquet_price(roses_qty: i32, tulip_qty: i32, rose_price: f64, tulip_price: f64) -> f64 {
    let roses = roses_qty as f64 * rose_price;
    let tulips = tulip_qty as f64 * tulip_price;

    roses + tulips
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bouquet_price() {
        let expected = 25.;
        let roses_qty = 2;
        let tulip_qty = 1;
        let rose_price = 10.;
        let tulip_price = 5.;

        let result = bouquet_price(roses_qty, tulip_qty, rose_price, tulip_price);

        assert_eq!(expected, result);
    }
}
