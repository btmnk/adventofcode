use std::fs;

fn get_lines_from_data() -> Vec<String> {
    let contents = fs::read_to_string("data/data.txt").expect("could not read part1.txt");
    return contents.split("\n").map(|it| it.to_owned()).collect();
}

fn part_one() {
    let line_vec: Vec<String> = get_lines_from_data();
    
    let mut previous_line: std::option::Option<String> = None;
    let mut counter = 0;

    for line in line_vec {
        if let Some(old_line) = previous_line {
            let old_line_int = old_line.parse::<i32>().unwrap();
            let current_line_int = line.parse::<i32>().unwrap();

            if old_line_int < current_line_int {
                counter = counter + 1;
            }
        }
        
        previous_line = Some(line);
    }

    println!("Result Part One: {}", counter);
}

fn get_chunk_sum_at_index(target_vec: Vec<String>, index: usize, chunk_size: usize) -> Option<i32> {
    let ending_index = index + chunk_size;
    let vec_partial = &target_vec[index..ending_index];
    let vec_partial_int: Vec<i32> = vec_partial.into_iter().map(|it| it.parse::<i32>().unwrap()).collect();
    return vec_partial_int.into_iter().reduce(|left, right| left + right);
}

fn part_two() {
    let line_vec: Vec<String> = get_lines_from_data();
    
    let mut previous_sum = None;
    let mut counter = 0;

    for (index, _) in line_vec.iter().enumerate() {
        if index < line_vec.len() - 3 {
            let current_sum_opt = get_chunk_sum_at_index(line_vec.to_owned(), index, 3);

            if current_sum_opt > previous_sum {
                counter = counter + 1;
            }
            
            previous_sum = current_sum_opt;
        }
    }

    println!("Result Part Two: {}", counter);
}

fn main() {
    part_one();
    part_two();
}
