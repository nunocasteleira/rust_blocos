/// Exercise One
pub fn class_percentage(girls: i32, boys: i32) -> (f64, f64) {
    let total = girls + boys;
    let girl_percentage = girls as f64 / total as f64;
    let boy_percentage = boys as f64 / total as f64;

    (girl_percentage, boy_percentage)
}
