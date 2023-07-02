use std::io;

use blocos::exerciser::input_exercise_number;

fn main() -> io::Result<()> {
    let exercise = input_exercise_number();

    println!("exercise {} ", exercise);

    Ok(())
}
