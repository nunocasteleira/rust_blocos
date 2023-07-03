use std::io;

use blocos::{bloco1::exercises::Bloco1Exercises, exerciser::input_exercise_number};

fn main() -> io::Result<()> {
    let stdin = io::stdin(); // We get `Stdin` here.
    let exercise = input_exercise_number(&stdin);

    println!("exercise {} ", exercise);

    let config = Bloco1Exercises::build(exercise, &stdin);
    let result = config.call();

    result.print_answer();

    Ok(())
}
