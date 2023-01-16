use std::collections::{HashSet, VecDeque};
use std::fs;

const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

type Grid = Vec<Vec<u8>>;

fn breadth_first_search(
    grid: &Grid,
    start_point: (usize, usize),
    end_point: (usize, usize),
    width: i32,
    height: i32,
) -> Option<usize> {
    let mut visited_points = HashSet::new();
    let mut queue = VecDeque::new();

    visited_points.insert(start_point);
    queue.push_back((start_point, 0));

    while let Some(position) = queue.pop_front() {
        let valid_points = get_surrounding_points(grid, position.0, width, height);

        for valid_point in valid_points {
            if !visited_points.insert(valid_point) {
                continue;
            }
            if valid_point == end_point {
                return Some(position.1 + 1);
            }
            queue.push_back((valid_point, position.1 + 1));
        }
    }

    None
}

fn get_surrounding_points(
    grid: &Grid,
    position: (usize, usize),
    width: i32,
    height: i32,
) -> Vec<(usize, usize)> {
    let i_position = (position.0 as i32, position.1 as i32);
    let current_elevation = grid[position.0][position.1];

    DIRECTIONS
        .iter()
        .map(|direction| (i_position.0 + direction.0, i_position.1 + direction.1))
        .filter(|position| {
            position.0 >= 0 && position.1 >= 0 && position.0 < height && position.1 < width
        })
        .map(|position| (position.0 as usize, position.1 as usize))
        .filter(|position| grid[position.0][position.1] <= current_elevation + 1)
        .collect()
}

fn get_fewest_number_of_steps(
    file_path: &str,
    start_point: Option<(usize, usize)>,
) -> Option<usize> {
    let file_contents =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut grid: Grid = Vec::new();
    let mut start: (usize, usize) = if let Some(start_point) = start_point {
        start_point
    } else {
        (0, 0)
    };
    let mut end: (usize, usize) = (0, 0);

    for (row, line) in file_contents.lines().enumerate() {
        let mut grid_line = line.chars().map(|char| char as u8).collect::<Vec<u8>>();
        if let Some(start_point) = grid_line.iter().position(|&p| p == b'S') {
            start = (row, start_point);
            grid_line[start_point] = b'a';
        }
        if let Some(end_point) = grid_line.iter().position(|&p| p == b'E') {
            end = (row, end_point);
            grid_line[end_point] = b'z';
        }
        grid.push(grid_line);
    }

    let width = grid[0].len() as i32;
    let height = grid.len() as i32;

    let mut start_points = Vec::new();
    for (row, line) in grid.iter().enumerate() {
        for (col, ch) in line.iter().enumerate() {
            if *ch == b'a' {
                start_points.push((row, col))
            }
        }
    }

    start_points
        .iter()
        .filter_map(|p| breadth_first_search(&grid, *p, end, width, height))
        .min()
}

fn main() {
    let steps = get_fewest_number_of_steps("./input.txt", None);
    println!("steps: {:?}", steps);
}

#[cfg(test)]
mod tests {
    use crate::get_fewest_number_of_steps;

    #[test]
    fn it_returns_expected_fewest_number_of_steps_for_test_file() {
        let number_of_steps =
            get_fewest_number_of_steps("./test.txt", Some((0, 0))).unwrap_or_default();
        assert_eq!(number_of_steps, 31);
    }

    #[test]
    fn it_returns_expected_fewest_number_of_steps_for_input_file() {
        let number_of_steps =
            get_fewest_number_of_steps("./input.txt", Some((0, 0))).unwrap_or_default();
        assert_eq!(number_of_steps, 484);
    }

    #[test]
    fn it_returns_expected_fewest_number_of_steps_from_any_square_for_test_file() {
        let number_of_steps = get_fewest_number_of_steps("./test.txt", None).unwrap_or_default();
        assert_eq!(number_of_steps, 29);
    }

    #[test]
    fn it_returns_expected_fewest_number_of_steps_from_any_square_for_input_file() {
        let number_of_steps = get_fewest_number_of_steps("./input.txt", None).unwrap_or_default();
        assert_eq!(number_of_steps, 478);
    }
}
