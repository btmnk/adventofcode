use std::fs;

mod ventdetector;
mod ventline;

fn get_lines_from_data(file: &str) -> Vec<String> {
    let contents = fs::read_to_string(file).expect("could not read data.txt");
    return contents.split("\n").map(|it| it.to_owned()).collect();
}

#[test]
pub fn test_dangerous_points() {
    let input_lines = get_lines_from_data("test.txt");
    let vent_lines: Vec<ventline::VentLine> = input_lines
        .into_iter()
        .map(|line| ventline::from(line.as_ref()).unwrap())
        .collect();

    let dangerous_points = ventdetector::count_dangerous_points(vent_lines, false);
    assert_eq!(dangerous_points, 5);
}

#[test]
pub fn test_dangerous_points_with_diagonals() {
    let input_lines = get_lines_from_data("test.txt");
    let vent_lines: Vec<ventline::VentLine> = input_lines
        .into_iter()
        .map(|line| ventline::from(line.as_ref()).unwrap())
        .collect();

    let dangerous_points = ventdetector::count_dangerous_points(vent_lines, true);
    assert_eq!(dangerous_points, 12);
}

fn main() {
    let input_lines = get_lines_from_data("input.txt");
    let vent_lines: Vec<ventline::VentLine> = input_lines
        .into_iter()
        .map(|line| ventline::from(line.as_ref()).unwrap())
        .collect();

    let dangerous_points = ventdetector::count_dangerous_points(vent_lines.to_owned(), false);

    println!("Dangerous points: {}", dangerous_points);

    let dangerous_points = ventdetector::count_dangerous_points(vent_lines, true);

    println!("Dangerous points with diagonals: {}", dangerous_points);
}
