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
    segment: Vec<Position>,
    visited: HashSet<Position>,
}

impl Rope {
    const DIRECTION: [Position; 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    fn create(length: usize) -> Self {
        Self {
            segment: vec![(0, 0); length],
            visited: HashSet::new(),
        }
    }

    fn make_move(&mut self, direction: Direction) {
        let rope_direction = Self::DIRECTION[direction as usize];
        self.segment[0].0 += rope_direction.0;
        self.segment[0].1 += rope_direction.1;

        for i in 1..self.segment.len() {
            let tail_to_head_row_difference = self.segment[i - 1].0 - self.segment[i].0;
            let tail_to_head_col_difference = self.segment[i - 1].1 - self.segment[i].1;

            if tail_to_head_row_difference.abs() > 1 || tail_to_head_col_difference.abs() > 1 {
                self.segment[i].0 += tail_to_head_row_difference.signum();
                self.segment[i].1 += tail_to_head_col_difference.signum();
            }

            self.visited.insert(self.segment[self.segment.len() - 1]);
        }
    }
}

fn get_number_of_positions_the_tail_visits(file_path: &str, rope_length: usize) -> usize {
    let file_contents =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut steps: Vec<(Direction, i32)> = Vec::new();

    for line in file_contents.lines() {
        let (direction_abbreviation, distance) = line.split_once(" ").unwrap();
        let direction = Direction::parse(direction_abbreviation);
        let distance = distance.parse::<i32>().unwrap();
        steps.push((direction, distance));
    }

    let mut rope = Rope::create(rope_length);
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
        get_number_of_positions_the_tail_visits("test.txt", 2)
    );
}

#[cfg(test)]
mod tests {
    use crate::get_number_of_positions_the_tail_visits;

    #[test]
    fn it_returns_expected_number_of_positions_for_length_of_2_with_test_file() {
        let number_of_positions = get_number_of_positions_the_tail_visits("./test.txt", 2);
        assert_eq!(number_of_positions, 13);
    }

    #[test]
    fn it_returns_expected_number_of_positions_for_length_of_10_with_test_file_2() {
        let number_of_positions = get_number_of_positions_the_tail_visits("./test-2.txt", 10);
        assert_eq!(number_of_positions, 36);
    }

    #[test]
    fn it_returns_expected_number_of_positions_for_input_file() {
        let number_of_positions = get_number_of_positions_the_tail_visits("./input.txt", 2);
        assert_eq!(number_of_positions, 6464);
    }

    #[test]
    fn it_returns_expected_number_of_positions_for_length_of_10_with_input_file() {
        let number_of_positions = get_number_of_positions_the_tail_visits("./input.txt", 10);
        assert_eq!(number_of_positions, 2604);
    }
}
