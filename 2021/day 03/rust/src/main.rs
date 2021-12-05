use std::fs;

fn get_lines_from_data(file: String) -> Vec<String> {
    let contents = fs::read_to_string(file).expect("could not read data.txt");
    return contents.split("\n").map(|it| it.to_owned()).collect();
}

fn get_most_common_bit_in_column(input_vec: Vec<String>, column: usize) -> i32 {
    let mut positive_bits = 0;
    let mut negative_bits = 0;

    for line in input_vec {
        let column_bit = line.chars().nth(column).unwrap();
        if column_bit == '0' {
            negative_bits += 1;
        } else {
            positive_bits += 1;
        }
    }

    return if positive_bits >= negative_bits { 1 } else { 0 };
}

#[test]
fn test_get_most_common_bit_in_column() {
    let input_vec = get_lines_from_data(String::from("data/test.txt"));

    assert_eq!(get_most_common_bit_in_column(input_vec.to_owned(), 0), 1);
    assert_eq!(get_most_common_bit_in_column(input_vec.to_owned(), 1), 0);
    assert_eq!(get_most_common_bit_in_column(input_vec.to_owned(), 2), 1);
    assert_eq!(get_most_common_bit_in_column(input_vec.to_owned(), 3), 1);
    assert_eq!(get_most_common_bit_in_column(input_vec.to_owned(), 4), 0);
}


fn get_columns(input_vec: Vec<String>) -> Option<usize> {
    if let Some(first_input) = input_vec.first() {
        return Some(first_input.len());
    } else {
        return None;
    }
}

#[test]
fn test_get_columns() {
    let test_input = get_lines_from_data(String::from("data/test.txt"));
    let data_input = get_lines_from_data(String::from("data/input.txt"));

    assert_eq!(get_columns(test_input.to_owned()), Some(5));
    assert_eq!(get_columns(data_input.to_owned()), Some(12));
}

fn get_gamma_rate(input_vec: Vec<String>) -> Option<i32> {
    let columns = get_columns(input_vec.to_owned())?;

    let mut gamma_rate = 0;

    for current_column in 0..columns {
        let reversed_column_index = columns - 1 - current_column;
        let columns_most_common_bit = get_most_common_bit_in_column(input_vec.to_owned(), reversed_column_index);
        gamma_rate ^= columns_most_common_bit << current_column;
    }

    return Some(gamma_rate);
}

#[test]
fn test_get_gamma_rate() {
    let test_input = get_lines_from_data(String::from("data/test.txt"));
    assert_eq!(get_gamma_rate(test_input), Some(22));
}

fn get_epsilon_rate(input_vec: Vec<String>, integer_size: usize) -> Option<i32> {
    let gamma_rate = get_gamma_rate(input_vec)?;
    // remove the first 20 flipped bits since we use 32-bit integers but work with 12/5 bits
    return Some(!gamma_rate << 32 - integer_size >> 32 - integer_size);
}

#[test]
fn test_get_epsilon_rate() {
    let test_input = get_lines_from_data(String::from("data/test.txt"));
    assert_eq!(get_epsilon_rate(test_input, 5), Some(9));
}

fn get_power_consumption(input_vec: Vec<String>) -> Option<i32> {
    let integer_size = get_columns(input_vec.to_owned())?;
    let gamma_rate = get_gamma_rate(input_vec.to_owned())?;
    let epsilon_rate = get_epsilon_rate(input_vec.to_owned(), integer_size)?;

    return Some(gamma_rate * epsilon_rate);
}

#[test]
fn test_get_power_consumption() {
    let test_input = get_lines_from_data(String::from("data/test.txt"));
    assert_eq!(get_power_consumption(test_input), Some(198));
}

fn get_bit_at_index(input: i32, index: usize) -> bool {
    if index < 32 {
        input & (1 << index) != 0
    } else {
        false
    }
}

#[test]
fn test_get_bit_at_index() {
    assert_eq!(get_bit_at_index(1, 0), true);
    assert_eq!(get_bit_at_index(1, 1), false);
    assert_eq!(get_bit_at_index(2, 1), true);
    assert_eq!(get_bit_at_index(2, 0), false);
    assert_eq!(get_bit_at_index(8, 3), true);
    assert_eq!(get_bit_at_index(8, 2), false);
    assert_eq!(get_bit_at_index(8, 1), false);
    assert_eq!(get_bit_at_index(8, 0), false);
}

fn filter_by_common_bit(input_vec: Vec<String>, most_common: bool) -> Option<i32> {
    let columns = get_columns(input_vec.to_owned())?;
    // let mut result_vec: Vec<i32> = input_vec.to_owned().into_iter().map(|it| i32::from_str_radix(it.as_ref(), 2).unwrap()).collect();
    let mut result_vec: Vec<String> = input_vec.to_owned();

    let mut current_column = 0;
    while result_vec.len() > 1 {
        let columns_most_common_bit = get_most_common_bit_in_column(result_vec.to_owned(), current_column);
        let target_bool = if most_common == true { columns_most_common_bit == 1 } else { columns_most_common_bit == 0 };
        let target_bit_index = columns - current_column - 1;

        result_vec = result_vec.into_iter().filter(|it| {
            let result_as_int = i32::from_str_radix(it.as_ref(), 2).unwrap();
            return get_bit_at_index(result_as_int, target_bit_index) == target_bool
        }).collect();

        current_column += 1;
    }

    let last_result = result_vec.first()?;
    let result_as_int = i32::from_str_radix(last_result.as_ref(), 2).unwrap();
    return Some(result_as_int.to_owned());
}

#[test]
fn test_filter_by_most_common_bit() {
    let test_input = get_lines_from_data(String::from("data/test.txt"));
    assert_eq!(filter_by_common_bit(test_input, true), Some(0b10111))
}

#[test]
fn test_filter_by_least_common_bit() {
    let test_input = get_lines_from_data(String::from("data/test.txt"));
    assert_eq!(filter_by_common_bit(test_input, false), Some(0b01010))
}

fn main() {
    let data_input = get_lines_from_data(String::from("data/input.txt"));
    let power_consumption = get_power_consumption(data_input.to_owned());

    if let Some(result) = power_consumption {
        println!("Power Consumption: {}", result);
    }

    let oxygen_generation_rating = filter_by_common_bit(data_input.to_owned(), true);
    let co2_scrubber_rating = filter_by_common_bit(data_input.to_owned(), false);

    if let (Some(oxygen_rating), Some(co2_rating)) = (oxygen_generation_rating, co2_scrubber_rating) {
        let life_support_rating = oxygen_rating * co2_rating;
        println!("Life Support Rating: {}", life_support_rating);
    }

}
