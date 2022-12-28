extern crate core;

use std::fs;

type Grid = Vec<Vec<u32>>;

fn is_at_edge_of_grid(grid: &Grid, grid_position: (usize, usize)) -> bool {
    let (current_row_index, current_column_index) = grid_position;

    if current_row_index == 0 {
        return true;
    }
    if current_column_index == 0 {
        return true;
    }
    if current_row_index == grid.len() - 1 {
        return true;
    }
    if current_column_index == grid[current_row_index].len() - 1 {
        return true;
    }
    return false;
}

fn is_tree_visible_from_left(grid: &Grid, grid_position: (usize, usize)) -> bool {
    let (current_row_index, current_column_index) = grid_position;

    let current_tree_height = grid[current_row_index][current_column_index];

    let left_slice = &grid[current_row_index][0..current_column_index];

    let trees_shorter_than_current_tree = left_slice
        .iter()
        .filter(|tree_height| tree_height < &&current_tree_height)
        .collect::<Vec<&u32>>();

    if trees_shorter_than_current_tree.len() == left_slice.len() {
        return true;
    }

    return false;
}

fn calculate_scenic_score_from_left(grid: &Grid, grid_position: (usize, usize)) -> u32 {
    let (current_row_index, current_column_index) = grid_position;

    let current_tree_height = grid[current_row_index][current_column_index];

    let left_slice = &grid[current_row_index][0..current_column_index];

    let mut number_of_trees_visible = 0;

    for tree_height in left_slice.iter().rev() {
        number_of_trees_visible += 1;

        if tree_height >= &current_tree_height {
            break;
        }
    }

    return number_of_trees_visible;
}

fn calculate_scenic_score_from_right(grid: &Grid, grid_position: (usize, usize)) -> u32 {
    let (current_row_index, current_column_index) = grid_position;

    let current_tree_height = grid[current_row_index][current_column_index];

    let number_of_columns = grid[0].len();

    let right_slice = &grid[current_row_index][current_column_index + 1..number_of_columns];

    let mut number_of_trees_visible = 0;

    for tree_height in right_slice.iter() {
        number_of_trees_visible += 1;

        if tree_height >= &current_tree_height {
            break;
        }
    }

    return number_of_trees_visible;
}

fn calculate_scenic_score_from_up(grid: &Grid, grid_position: (usize, usize)) -> u32 {
    let (current_row_index, current_column_index) = grid_position;

    let current_tree_height = grid[current_row_index][current_column_index];

    let mut up_slice: Vec<u32> = Vec::new();

    for i in 0..current_row_index {
        up_slice.push(grid[i][current_column_index]);
    }

    let mut number_of_trees_visible = 0;

    for tree_height in up_slice.iter().rev() {
        number_of_trees_visible += 1;

        if tree_height >= &current_tree_height {
            break;
        }
    }

    return number_of_trees_visible;
}

fn calculate_scenic_score_from_down(grid: &Grid, grid_position: (usize, usize)) -> u32 {
    let (current_row_index, current_column_index) = grid_position;

    let current_tree_height = grid[current_row_index][current_column_index];

    let number_of_rows = grid.len();

    let mut down_slice: Vec<u32> = Vec::new();

    for i in current_row_index + 1..number_of_rows {
        down_slice.push(grid[i][current_column_index]);
    }

    let mut number_of_trees_visible = 0;

    for tree_height in down_slice.iter() {
        number_of_trees_visible += 1;

        if tree_height >= &current_tree_height {
            break;
        }
    }

    return number_of_trees_visible;
}

fn is_tree_visible_from_right(grid: &Grid, grid_position: (usize, usize)) -> bool {
    let (current_row_index, current_column_index) = grid_position;

    let current_tree_height = grid[current_row_index][current_column_index];

    let number_of_columns = grid[0].len();

    let right_slice = &grid[current_row_index][current_column_index + 1..number_of_columns];

    let trees_shorter_than_current_tree = right_slice
        .iter()
        .filter(|tree_size| tree_size < &&current_tree_height)
        .collect::<Vec<&u32>>();

    if trees_shorter_than_current_tree.len() == right_slice.len() {
        return true;
    }

    return false;
}

fn is_tree_visible_from_up(grid: &Grid, grid_position: (usize, usize)) -> bool {
    let (current_row_index, current_column_index) = grid_position;

    let current_tree_height = grid[current_row_index][current_column_index];

    let mut up_slice: Vec<u32> = Vec::new();

    for i in 0..current_row_index {
        up_slice.push(grid[i][current_column_index]);
    }

    let trees_shorter_than_current_tree = up_slice
        .iter()
        .filter(|tree_size| tree_size < &&current_tree_height)
        .collect::<Vec<&u32>>();

    if trees_shorter_than_current_tree.len() == up_slice.len() {
        return true;
    }

    return false;
}

fn is_tree_visible_from_down(grid: &Grid, grid_position: (usize, usize)) -> bool {
    let (current_row_index, current_column_index) = grid_position;

    let current_tree_height = grid[current_row_index][current_column_index];

    let number_of_rows = grid.len();

    let mut down_slice: Vec<u32> = Vec::new();

    for i in current_row_index + 1..number_of_rows {
        down_slice.push(grid[i][current_column_index]);
    }

    let trees_shorter_than_current_tree = down_slice
        .iter()
        .filter(|tree_size| tree_size < &&current_tree_height)
        .collect::<Vec<&u32>>();

    if trees_shorter_than_current_tree.len() == down_slice.len() {
        return true;
    }

    return false;
}

fn is_tree_visible(grid: &Grid, grid_position: (usize, usize)) -> bool {
    if is_at_edge_of_grid(grid, grid_position) {
        return true;
    }

    if is_tree_visible_from_left(grid, grid_position) {
        return true;
    }

    if is_tree_visible_from_up(grid, grid_position) {
        return true;
    }

    if is_tree_visible_from_right(grid, grid_position) {
        return true;
    }

    if is_tree_visible_from_down(grid, grid_position) {
        return true;
    }

    return false;
}

fn get_score_for_tree(grid: &Grid, grid_position: (usize, usize)) -> u32 {
    let mut score = 1;

    score *= calculate_scenic_score_from_left(&grid, grid_position);

    score *= calculate_scenic_score_from_right(&grid, grid_position);

    score *= calculate_scenic_score_from_up(&grid, grid_position);

    score *= calculate_scenic_score_from_down(&grid, grid_position);

    return score;
}

fn count_visible_trees(grid: Grid) -> usize {
    let mut count = 0;
    grid.iter().enumerate().for_each(|(row_index, row)| {
        row.iter().enumerate().for_each(|(column_index, _)| {
            if is_tree_visible(&grid, (row_index, column_index)) {
                count += 1;
            }
        });
    });
    return count;
}

fn get_scenic_score_for_trees(grid: &Grid) -> Vec<u32> {
    let mut scores = Vec::new();

    grid.iter().enumerate().for_each(|(row_index, row)| {
        row.iter().enumerate().for_each(|(column_index, _)| {
            scores.push(get_score_for_tree(grid, (row_index, column_index)));
        })
    });

    scores.sort();

    return scores;
}

fn get_largest_scenic_score(scores: Vec<u32>) -> u32 {
    *scores.last().unwrap()
}

fn create_grid(file_path: &str) -> Grid {
    let mut grid: Grid = Vec::new();

    let file_contents =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    for line in file_contents.lines() {
        grid.push(
            line.chars()
                .map(|char| char.to_digit(10).unwrap())
                .collect(),
        );
    }

    return grid;
}

fn count_number_of_visible_trees(file_path: &str) -> usize {
    let grid = create_grid(file_path);

    return count_visible_trees(grid);
}

fn get_highest_scenic_score(file_path: &str) -> u32 {
    let grid = create_grid(file_path);
    let scenic_scores_for_trees = get_scenic_score_for_trees(&grid);
    return get_largest_scenic_score(scenic_scores_for_trees);
}

fn main() {
    count_number_of_visible_trees("./input.txt");
    get_highest_scenic_score("./input.txt");
}

#[cfg(test)]
mod tests {
    use crate::count_number_of_visible_trees;
    use crate::get_highest_scenic_score;

    #[test]
    fn it_returns_expected_count_of_visible_trees_for_test_file() {
        let count = count_number_of_visible_trees("./test.txt");
        assert_eq!(count, 21);
    }

    #[test]
    fn it_returns_expected_count_of_visible_trees_for_input_file() {
        let count = count_number_of_visible_trees("./input.txt");
        assert_eq!(count, 1787);
    }

    #[test]
    fn it_returns_expected_highest_scenic_score_for_test_file() {
        let count = get_highest_scenic_score("./test.txt");
        assert_eq!(count, 8);
    }

    #[test]
    fn it_returns_expected_highest_scenic_score_for_input_file() {
        let count = get_highest_scenic_score("./input.txt");
        assert_eq!(count, 440640);
    }
}
