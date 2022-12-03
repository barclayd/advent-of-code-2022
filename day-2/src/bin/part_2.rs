use std::fs;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
enum HandShape {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Game {
    opponent: HandShape,
    result: Result,
}

fn build_desired_outcome_game(opponent: HandShape, result: Result) -> Game {
    Game {
        opponent,
        result,
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
enum Result {
    Win,
    Draw,
    Lose,
}

fn get_desired_games_from_file(file_path: &str, char_to_hand_shape: HashMap<&str, HandShape>, char_to_result: HashMap<&str, Result>) -> Vec<Game> {
    let file_contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let files_contents_by_line = file_contents.lines();

    let mut games: Vec<Game> = Vec::new();

    for line in files_contents_by_line {

        let opponent_move_desired_outcome: Vec<&str> = line.split(" ").collect();

        let game = build_desired_outcome_game(char_to_hand_shape[opponent_move_desired_outcome[0]], char_to_result[opponent_move_desired_outcome[1]]);

        games.push(game);
    }

    return games;
}

fn calculate_total_score(file_path: &str) -> i128 {
    let char_to_hand_shape = HashMap::from([
        ("A", HandShape::Rock),
        ("X", HandShape::Rock),
        ("B", HandShape::Paper),
        ("Y", HandShape::Paper),
        ("C", HandShape::Scissors),
        ("Z", HandShape::Scissors)
    ]);

    let char_to_result = HashMap::from([
        ("X", Result::Lose),
        ("Y", Result::Draw),
        ("Z", Result::Win)
    ]);

    let result_to_points = HashMap::from([
        (Result::Win, 6),
        (Result::Draw, 3),
        (Result::Lose, 0),
    ]);

    let hand_shape_to_points = HashMap::from([
        (HandShape::Rock, 1),
        (HandShape::Paper, 2),
        (HandShape::Scissors, 3),
    ]);

    let opponent_hand_shape_and_result_to_hand_shape = HashMap::from([
        (Game { opponent: HandShape::Rock, result: Result::Draw }, HandShape::Rock),
        (Game { opponent: HandShape::Rock, result: Result::Win }, HandShape::Paper),
        (Game { opponent: HandShape::Rock, result: Result::Lose }, HandShape::Scissors),
        (Game { opponent: HandShape::Paper, result: Result::Lose }, HandShape::Rock),
        (Game { opponent: HandShape::Paper, result: Result::Draw }, HandShape::Paper),
        (Game { opponent: HandShape::Paper, result: Result::Win }, HandShape::Scissors),
        (Game { opponent: HandShape::Scissors, result: Result::Draw }, HandShape::Scissors),
        (Game { opponent: HandShape::Scissors, result: Result::Lose }, HandShape::Paper),
        (Game { opponent: HandShape::Scissors, result: Result::Win }, HandShape::Rock),
    ]);

    let get_score_for_desired_outcome_game = |game: &Game| -> i128 {
        let hand_shape = opponent_hand_shape_and_result_to_hand_shape[game];
        let hand_shape_points = hand_shape_to_points[&hand_shape];
        let result_points = result_to_points[&game.result];
        hand_shape_points + result_points
    };

    let games: Vec<Game> = get_desired_games_from_file(file_path, char_to_hand_shape, char_to_result);
    let scores_for_games = games.iter().map(|game| get_score_for_desired_outcome_game(game));

    let total_score = scores_for_games.sum::<i128>();

    return total_score;
}

fn main() {
    let total_score_for_desired_outcome_games = calculate_total_score("./scores.txt");
    println!("{}", total_score_for_desired_outcome_games);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_expected_result_for_test_file() {
        let total_score = calculate_total_score("./test.txt");
        assert_eq!(total_score, 12);
    }

    #[test]
    fn it_returns_expected_result_for_scores_file() {
        let total_score = calculate_total_score("./scores.txt");
        assert_eq!(total_score, 13726);
    }
}