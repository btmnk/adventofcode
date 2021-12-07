use std::fs;

fn get_crab_input(file: &str) -> Vec<i32> {
    let contents =
        fs::read_to_string(file).expect("could not read input file");
    return contents.split(",").map(|it| it.parse::<i32>().unwrap()).collect();
}

fn calculate_median(crabs: Vec<i32>) -> i32 {
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
fn test_calculate_median() {
    assert_eq!(calculate_median(vec![16,1,2,0,4,2,7,1,2,14]), 2);
}

fn calculate_mean(crabs: Vec<i32>) -> f32 {
    return crabs.iter().sum::<i32>() as f32 / crabs.len() as f32;
}

#[test]
fn test_calculate_mean() {
    assert_eq!(calculate_mean(vec![16,1,2,0,4,2,7,1,2,14]).round() as i32, 5);
    assert_eq!(calculate_mean(vec![9, 10, 12, 13, 13, 13, 15, 15, 16, 16, 18, 22, 23, 24, 24, 25]).round() as i32, 17);
}

fn calculate_mean_fuel_cost(crab_input: Vec<i32>) -> i32 {
    let crab_mean = calculate_mean(crab_input.to_owned()) as i32;

    let fuel_cost_to_mean = (crab_mean..).take(2).map(|mean| {
        return crab_input.iter().map(|position| {
            let distance = (position - mean).abs();
            return distance * (distance + 1) / 2;
        }).sum::<i32>();
    }).min().unwrap();

    return fuel_cost_to_mean;
}

#[test]
fn test_calculate_mean_fuel_cost() {
    assert_eq!(calculate_mean_fuel_cost(vec![16,1,2,0,4,2,7,1,2,14]), 168)
}

fn main() {
    let mut crab_input = get_crab_input("input.txt");
    crab_input.sort();

    let crab_median = calculate_median(crab_input.to_owned());
    let fuel_cost_to_median = crab_input.iter().fold(0, |sum, current| sum + (current - crab_median).abs());
    println!("Total fuel cost to median position for part one: {}", fuel_cost_to_median);
    
    let fuel_cost_to_mean = calculate_mean_fuel_cost(crab_input.to_owned());
    println!("Total fuel cost to mean position for part two: {}", fuel_cost_to_mean);
}