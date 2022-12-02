use std::fmt::Debug;
use std::fs;

pub fn log_result<Value: Debug, Task: Debug>(value: Value, task: Task) {
    println!("Result for {:?}: {:?}", task, value);
}

pub fn get_puzzle_input(file: &str, delimiter: &str) -> Vec<String> {
    let contents = fs::read_to_string(file).expect("could not read input file");

    contents
        .split(delimiter)
        .map(|it| it.parse::<String>().unwrap())
        .collect()
}
