use std::collections::HashMap;
use std::fs;

const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn get_surrounding_points(
    position: (usize, usize),
    width: i32,
    height: i32,
) -> Vec<(usize, usize)> {
    let i_position = (position.0 as i32, position.1 as i32);

    DIRECTIONS
        .iter()
        .map(|direction| (i_position.0 + direction.0, i_position.1 + direction.1))
        .filter(|position| {
            position.0 >= 0 && position.1 >= 0 && position.0 < width && position.1 < height
        })
        .map(|position| (position.0 as usize, position.1 as usize))
        .collect()
}

fn get_fewest_number_of_steps(file_path: &str) -> usize {
    let file_contents =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut grid: Vec<Vec<u8>> = Vec::new();
    let mut start: (usize, usize) = (0, 0);
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

    let mut steps_to_visit: Vec<(usize, usize)> = Vec::new();

    steps_to_visit.extend(get_surrounding_points(start, width, height));

    let mut shortest_locations: HashMap<(usize, usize), usize> = HashMap::new();

    shortest_locations.insert(start, 0);
    while let Some(location) = steps_to_visit.pop() {
        let current_elevation = grid[location.0][location.1];
        let points = get_surrounding_points(location, width, height);

        let valid_path_locations = points
            .iter()
            .filter(|position| grid[position.0][position.1] + 1 > current_elevation)
            .map(|position| *position)
            .collect::<Vec<(usize, usize)>>();

        let new_path_distance = valid_path_locations
            .iter()
            .filter_map(|position| shortest_locations.get(position))
            .min()
            .unwrap()
            + 1;

        let current_path_distance = shortest_locations.entry(location).or_insert(usize::MAX);

        if *current_path_distance > new_path_distance {
            *current_path_distance = new_path_distance;
            steps_to_visit.extend(points);
        }
    }

    return *shortest_locations.get(&end).unwrap();
}

fn main() {
    get_fewest_number_of_steps("./test.txt");
}

#[cfg(test)]
mod tests {
    use crate::get_fewest_number_of_steps;

    #[test]
    fn it_returns_expected_fewest_number_of_steps_for_test_file() {
        let number_of_steps = get_fewest_number_of_steps("./test.txt");
        assert_eq!(number_of_steps, 31);
    }
}
