use super::shape::Shape;

pub fn get_shape_by_code(code: &str) -> Shape {
    match code {
        "A" | "X" => Shape::ROCK,
        "B" | "Y" => Shape::PAPER,
        "C" | "Z" => Shape::SCISSORS,
        _ => panic!("Expected A, B, C, X, Y or Z but got {}", code),
    }
}

pub fn get_shape_score(shape: Shape) -> i32 {
    match shape {
        Shape::ROCK => 1,
        Shape::PAPER => 2,
        Shape::SCISSORS => 3,
    }
}

pub fn get_game_result(your_shape: Shape, enemy_shape: Shape) -> i32 {
    if your_shape == enemy_shape {
        return 3;
    }

    if your_shape == get_winning_shape(enemy_shape) {
        return 6;
    }

    0
}

pub fn get_winning_shape(shape: Shape) -> Shape {
    match shape {
        Shape::ROCK => Shape::PAPER,
        Shape::SCISSORS => Shape::ROCK,
        Shape::PAPER => Shape::SCISSORS,
    }
}

pub fn get_losing_shape(shape: Shape) -> Shape {
    match shape {
        Shape::ROCK => Shape::SCISSORS,
        Shape::SCISSORS => Shape::PAPER,
        Shape::PAPER => Shape::ROCK,
    }
}

pub fn get_shape_by_game_result(attack_shape: Shape, target_result: &str) -> Shape {
    match target_result {
        // Should end in lose
        "X" => get_losing_shape(attack_shape),
        // Should end in draw
        "Y" => attack_shape,
        // Should end as win
        "Z" => get_winning_shape(attack_shape),
        _ => panic!(
            "expected X, Y or Z for target_result but got {}",
            target_result
        ),
    }
}
