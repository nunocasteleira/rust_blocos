use std::io;

pub fn input_exercise_number() -> u32 {
    let mut exercise = String::new();
    let stdin = io::stdin(); // We get `Stdin` here.

    let mut result: Result<u32, &'static str>;
    loop {
        println!("Please input what exercise would you like to run:");
        let _ = stdin.read_line(&mut exercise);
        result = validate_exercise_number(&exercise);
        match result {
            Ok(_) => break,
            Err(e) => {
                println!("Error parsing exercise: {e}");
                exercise.clear();
            }
        }
    }
    result.unwrap()
}

pub fn validate_exercise_number(exercise_number: &str) -> Result<u32, &'static str> {
    match exercise_number.trim().parse::<u32>() {
        Ok(n) => Ok(n),
        Err(_) => Err("Invalid exercise. Did you pass a valid exercise number?"),
    }
}
