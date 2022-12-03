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
    player: HandShape,
}

fn build_game(opponent: HandShape, player: HandShape) -> Game {
    Game {
        opponent,
        player,
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
enum Result {
    Win,
    Draw,
    Lose,
}

fn get_games_from_file(file_path: &str, char_to_hand_shape: HashMap<&str, HandShape>) -> Vec<Game> {
    let file_contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let file_contents_by_line = file_contents.lines();

    let mut games: Vec<Game> = Vec::new();

    for line in file_contents_by_line {

        let choices: Vec<&str> = line.split(" ").collect();

        let game = build_game(char_to_hand_shape[choices[0]], char_to_hand_shape[choices[1]]);

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

    let game_to_result = HashMap::from([
        (Game { opponent: HandShape::Rock, player: HandShape::Rock }, Result::Draw),
        (Game { opponent: HandShape::Scissors, player: HandShape::Scissors }, Result::Draw),
        (Game { opponent: HandShape::Paper, player: HandShape::Paper }, Result::Draw),
        (Game { opponent: HandShape::Rock, player: HandShape::Paper }, Result::Win),
        (Game { opponent: HandShape::Rock, player: HandShape::Scissors }, Result::Lose),
        (Game { opponent: HandShape::Paper, player: HandShape::Scissors }, Result::Win),
        (Game { opponent: HandShape::Paper, player: HandShape::Rock }, Result::Lose),
        (Game { opponent: HandShape::Scissors, player: HandShape::Rock }, Result::Win),
        (Game { opponent: HandShape::Scissors, player: HandShape::Paper }, Result::Lose),
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

    let get_score_from_game = |game: &Game| -> i128 {
        let result = game_to_result[&game];
        let points_from_game = result_to_points[&result];
        let points_from_hand_shape = hand_shape_to_points[&game.player];
        points_from_game + points_from_hand_shape
    };

    let games: Vec<Game> = get_games_from_file(file_path, char_to_hand_shape);

    let scores_for_games = games.iter().map(|game| get_score_from_game(game));

    let total_score = scores_for_games.sum::<i128>();

    return total_score;
}

fn main() {
    let total_score = calculate_total_score("./scores.txt");
    println!("{}", total_score);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_expected_result_for_test_file() {
        let total_score = calculate_total_score("./test.txt");
        assert_eq!(total_score, 15);
    }

    #[test]
    fn it_returns_expected_result_for_scores_file() {
        let total_score = calculate_total_score("./scores.txt");
        assert_eq!(total_score, 12855);
    }
}