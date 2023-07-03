use std::io::Stdin;

use crate::exerciser::{retry_input, ExerciseConfig, ExerciseResult};

/// Exercise One
pub struct ClassConfig {
    boys: u32,
    girls: u32,
}

impl ExerciseConfig for ClassConfig {
    fn build(stdin: &Stdin) -> Self {
        let mut boys_qty_input = String::new();
        let mut girls_qty_input = String::new();

        let boys: u32 = retry_input(stdin, &mut boys_qty_input, "How many boys?", "");
        let girls: u32 = retry_input(stdin, &mut girls_qty_input, "How many girls?", "");

        ClassConfig { boys, girls }
    }
}

pub struct ClassResult {
    boy_percentage: f64,
    girl_percentage: f64,
}

impl ExerciseResult for ClassResult {
    fn print_answer(&self) {
        let ClassResult {
            boy_percentage,
            girl_percentage,
        } = self;

        println!(
            "The class has {:.2}% of boys and {:.2}% of girls",
            boy_percentage, girl_percentage,
        )
    }
}

pub fn class_percentage(config: ClassConfig) -> ClassResult {
    let ClassConfig { boys, girls } = config;
    let total = girls + boys;
    let girl_percentage = girls as f64 / total as f64;
    let boy_percentage = boys as f64 / total as f64;

    ClassResult {
        boy_percentage,
        girl_percentage,
    }
}

pub fn format_class_result(result: (f64, f64)) -> String {
    let (boys, girls) = result;
    format!(
        "The class has {:.2}% of boys and {:.2}% of girls",
        boys, girls
    )
}
