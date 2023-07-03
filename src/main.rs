use std::io;

use blocos::{
    bloco1::e02_bouquet_price::{bouquet_price, format_bouquet_answer, BouquetConfig},
    exerciser::{input_exercise_number, ExerciseConfig},
};

fn main() -> io::Result<()> {
    let stdin = io::stdin(); // We get `Stdin` here.
    let exercise = input_exercise_number(&stdin);

    println!("exercise {} ", exercise);

    let config = BouquetConfig::build(&stdin);
    let result = bouquet_price(config);

    println!("{}", format_bouquet_answer(result));

    Ok(())
}
