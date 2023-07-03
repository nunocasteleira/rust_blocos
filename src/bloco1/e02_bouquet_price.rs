use std::io::Stdin;

use crate::exerciser::{retry_input, ExerciseConfig};

/// Exercise Two

#[derive(Debug)]
pub struct BouquetConfig {
    roses_qty: u32,
    tulip_qty: u32,
    rose_price: f64,
    tulip_price: f64,
}

impl ExerciseConfig for BouquetConfig {
    fn build(stdin: &Stdin) -> BouquetConfig {
        let mut roses_qty_input = String::new();
        let mut tulip_qty_input = String::new();
        let mut rose_price_input = String::new();
        let mut tulip_price_input = String::new();

        let roses_qty: u32 = retry_input(stdin, &mut roses_qty_input, "How many roses?", "");
        let tulip_qty: u32 = retry_input(stdin, &mut tulip_qty_input, "How many tulips?", "");
        let rose_price: f64 = retry_input(
            stdin,
            &mut rose_price_input,
            "How much does the roses cost?",
            "",
        );
        let tulip_price: f64 = retry_input(
            stdin,
            &mut tulip_price_input,
            "How much does the tulips cost?",
            "",
        );

        BouquetConfig {
            roses_qty,
            tulip_qty,
            rose_price,
            tulip_price,
        }
    }
}

pub fn bouquet_price(config: BouquetConfig) -> f64 {
    let roses = config.roses_qty as f64 * config.rose_price;
    let tulips = config.tulip_qty as f64 * config.tulip_price;

    roses + tulips
}

pub fn format_bouquet_answer(result: f64) -> String {
    format!("The bouquet will cost €{:.2}", result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bouquet_price() {
        let expected = 25.;

        let result = bouquet_price(BouquetConfig {
            roses_qty: 2,
            tulip_qty: 1,
            rose_price: 10.,
            tulip_price: 5.,
        });

        assert_eq!(expected, result);
    }
}
