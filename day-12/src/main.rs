use std::fs;

fn get_fewest_number_of_steps(file_path: &str) -> i32 {
    let file_contents =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut grid: Vec<Vec<u8>> = Vec::new();
    let mut start: (usize, usize);
    let mut end: (usize, usize);

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

    0
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
