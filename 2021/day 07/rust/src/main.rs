use std::fs;

fn get_crab_input(file: &str) -> Vec<i32> {
    let contents =
        fs::read_to_string(file).expect("could not read input file");
    return contents.split(",").map(|it| it.parse::<i32>().unwrap()).collect();
}

fn calculate_mean(crabs: Vec<i32>) -> i32 {
    if crabs.len() % 2 != 0 {
        let lower_mean_index = crabs.len() / 2;
        let upper_mean_index = lower_mean_index + 1;
        return (crabs[lower_mean_index] + crabs[upper_mean_index]) / 2
    } else {
        let mean_index = crabs.len() / 2;
        return crabs[mean_index];
    }
}

#[test]
fn test_calculate_mean() {
    let mean = calculate_mean(vec![16,1,2,0,4,2,7,1,2,14]);
    assert_eq!(mean, 2);
}

fn main() {
    let mut crab_input = get_crab_input("input.txt");
    crab_input.sort();

    let best_position = calculate_mean(crab_input.to_owned());
    let fuel_cost = crab_input.into_iter().fold(0, |sum, current| sum + (current - best_position).abs());
    println!("Total fuel cost to best position: {}", fuel_cost);
}