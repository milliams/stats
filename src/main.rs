use std::io::{self, BufRead};

fn get_single_column() -> Vec<f64> {
    let stdin = io::stdin();
    let mut data: Vec<f64> = vec![];
    for line in stdin.lock().lines() {
        let line_text = match line {
            Ok(line) => line,
            Err(err) => panic!("IO error: {}", err),
        };
        data.push(line_text.parse::<f64>()
            .expect(&format!("ERROR: Could not parse '{}' as an f64", line_text)));
    }
    data
}

fn main() {
    let data = get_single_column();
    let sum: f64 = data.iter().sum();
    let count = data.len();
    let mean = sum / count as f64;
    println!("{}", mean);
}
