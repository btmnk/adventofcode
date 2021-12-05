use std::fs;

fn get_lines_from_data() -> Vec<String> {
    let contents = fs::read_to_string("data/data.txt").expect("could not read data.txt");
    return contents.split("\n").map(|it| it.to_owned()).collect();
}

fn parse_command(command_line: String) -> Option<(String, String)> {    
    let command_parts: Vec<String> = command_line.split(" ").map(|it| it.to_owned()).collect();
    let command_type_op = command_parts.first();
    let command_value_opt = command_parts.last();

    if let (Some(command_type), Some(command_value)) = (command_type_op, command_value_opt) {
        return Some((command_type.to_owned(), command_value.to_owned()));
    } else {
        return None;
    }
}

fn part_one(command_vec: Vec<String>) {
    let mut horizontal_position = 0;
    let mut depth = 0;

    for command_line in command_vec {
        let command_opt = parse_command(command_line);

        if let Some(command) = command_opt {
            let command_type = command.0;
            let command_value = command.1.parse::<i32>().unwrap();

            match command_type.as_ref() {
                "forward" => horizontal_position = horizontal_position + command_value,
                "up" => depth = depth - command_value,
                "down" => depth = depth + command_value,
                _ => println!("Something else!")
            }
        }
    }

    println!("Final position is: {} horizontal and a depth of {}", horizontal_position, depth);
    println!("Result for part one: horizontal * depth = {}", horizontal_position * depth);
}

fn part_two(command_vec: Vec<String>) {
    let mut horizontal_position = 0;
    let mut depth = 0;
    let mut aim = 0;

    for command_line in command_vec {
        let command_opt = parse_command(command_line);

        if let Some(command) = command_opt {
            let command_type = command.0;
            let command_value = command.1.parse::<i32>().unwrap();

            match command_type.as_ref() {
                "forward" => {
                    horizontal_position = horizontal_position + command_value;
                    depth = depth + aim * command_value
                },
                "up" => aim = aim - command_value,
                "down" => aim = aim + command_value,
                _ => println!("Something else!")
            }
        }
    }

    println!("Final position is: {} horizontal and a depth of {}", horizontal_position, depth);
    println!("Result for part two: horizontal * depth = {}", horizontal_position * depth);
}

fn main() {
    let data = get_lines_from_data();
    part_one(data.clone());
    part_two(data.clone());
}
