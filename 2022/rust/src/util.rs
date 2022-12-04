use std::fmt::Debug;
use std::fs;
use std::ops::RangeInclusive;

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

/**
 * Expects a string input like "3-43" and returns a Range of 3..=43
 */
pub fn range_from_string(input: &str) -> RangeInclusive<i32> {
    let indexes: Vec<i32> = input
        .split("-")
        .map(|it| it.parse::<i32>().unwrap())
        .collect();

    return RangeInclusive::new(indexes[0], indexes[1]);
}
