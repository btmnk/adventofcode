pub struct Elf {
    pub calories: Vec<i32>,
}

impl Elf {
    pub fn get_calories_sum(&self) -> i32 {
        self.calories.iter().fold(0, |sum, food| sum + food)
    }
}

pub fn get_elves_from_input(input: Vec<String>) -> Vec<Elf> {
    input
        .iter()
        .map(|it| Elf {
            calories: it
                .split("\n")
                .map(|it| it.parse::<i32>().unwrap())
                .collect(),
        })
        .collect()
}
