use std::fs;

mod game;

fn get_bingo_input() -> Vec<String> {
    let contents =
        fs::read_to_string("bingo_input.txt").expect("could not read file bingo_input.txt");
    return contents.split(",").map(str::to_string).collect();
}

fn main() {
    let bingo_input = get_bingo_input();
    let game = &mut game::from("boards.txt");

    for bingo_value in bingo_input {
        game.play_round(bingo_value.as_ref());
    }

    let winners = game.get_winners();

    let first_winner = winners.first().unwrap();
    let last_winner = winners.last().unwrap();

    println!(
        "Part One, first winner score: {}",
        first_winner.calculate_score()
    );
    println!(
        "Part Two, last winner score: {}",
        last_winner.calculate_score()
    );
}
