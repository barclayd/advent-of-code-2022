use std::collections::HashSet;
use std::fs;

type Position = (i32, i32);

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn parse(direction_abbreviation: &str) -> Self {
        match direction_abbreviation {
            "L" => Self::Left,
            "R" => Self::Right,
            "U" => Self::Up,
            "D" => Self::Down,
            _ => panic!("invalid direction '${direction_abbreviation}'"),
        }
    }
}
#[derive(Default)]
struct Rope {
    head: Position,
    tail: Position,
    visited: HashSet<Position>,
}

impl Rope {
    const DIRECTION: [Position; 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    fn make_move(&mut self, direction: Direction) {
        println!("{direction:?}")
    }
}

fn get_number_of_positions_the_tail_visits(file_path: &str) -> usize {
    let file_contents =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut steps: Vec<(Direction, i32)> = Vec::new();

    for line in file_contents.lines() {
        let (direction_abbreviation, distance) = line.split_once(" ").unwrap();
        let direction = Direction::parse(direction_abbreviation);
        let distance = distance.parse::<i32>().unwrap();
        steps.push((direction, distance));
    }

    let mut rope = Rope::default();
    for (direction, distance) in steps {
        for _ in 0..distance {
            rope.make_move(direction);
        }
    }

    return rope.visited.len();
}

fn main() {
    println!(
        "number of positions {}",
        get_number_of_positions_the_tail_visits("test.txt")
    );
}

#[cfg(test)]
mod tests {
    use crate::get_number_of_positions_the_tail_visits;

    #[test]
    fn it_returns_expected_number_of_positions_for_test_file() {
        let count = get_number_of_positions_the_tail_visits("./test.txt");
        assert_eq!(count, 13);
    }

    #[test]
    fn it_returns_expected_number_of_positions_for_input_file() {
        let count = get_number_of_positions_the_tail_visits("./input.txt");
        assert_eq!(count, 0);
    }
}
