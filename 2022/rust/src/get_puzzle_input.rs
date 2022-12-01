use std::fs;

pub fn get_puzzle_input_lines(file: &str, delimiter: &str) -> Vec<String> {
    let contents = fs::read_to_string(file).expect("could not read input file");

    contents
        .split(delimiter)
        .map(|it| it.parse::<String>().unwrap())
        .collect()
}
