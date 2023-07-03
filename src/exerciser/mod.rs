use std::{io::Stdin, str::FromStr};

pub trait ExerciseConfig {
    fn get_title() -> String;
    fn build(stdin: &Stdin) -> Self;
}

pub trait ExerciseResult {
    fn print_answer(&self);
}

pub fn input_exercise_number(stdin: &Stdin) -> u32 {
    let mut exercise = String::new();

    retry_input(
        stdin,
        &mut exercise,
        "Please input what exercise would you like to run:",
        "Error parsing exercise",
    )
}

pub fn validate_input<F: FromStr>(input: &str) -> Result<F, &'static str> {
    match input.trim().parse::<F>() {
        Ok(n) => Ok(n),
        Err(_) => Err("Failed to parse user input"),
        // Invalid exercise. Did you pass a valid exercise number?
    }
}

pub fn retry_input<F: FromStr>(
    stdin: &Stdin,
    buf: &mut String,
    prompt: &str,
    error_msg: &str,
) -> F {
    let mut result: Result<F, &'static str>;
    loop {
        println!("{}", prompt);
        let _ = stdin.read_line(buf);
        result = validate_input(buf);
        match result {
            Ok(_) => break,
            Err(e) => {
                println!("{}: {}", error_msg, e);
                buf.clear();
            }
        }
    }
    result.unwrap()
}
