use std::fs;

mod school;

fn get_fish_input(file: &str) -> Vec<i32> {
    let contents =
        fs::read_to_string(file).expect("could not read input file");
    return contents.split(",").map(|it| it.parse::<i32>().unwrap()).collect();
}

#[test]
fn test_fishes() {
    let test_input = get_fish_input("test.txt");
    let mut school = school::from(test_input);
    school.pass_days(18);
    assert_eq!(school.count_fish(), [6,0,6,4,5,6,0,1,1,2,6,0,1,1,1,2,2,3,3,4,6,7,8,8,8,8].len())
}

fn main() {
    let fish_input = get_fish_input("input.txt");
    let mut school = school::from(fish_input);
    school.pass_days(80);

    println!("Fishes after 80 days: {}", school.count_fish());

    school.pass_days(256 - 80);

    println!("Fishes after 256 days: {}", school.count_fish());
}
