use std::fs;
use std::ops::Range;

type Grid = Vec<Vec<u32>>;

#[derive(Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

fn get_slice_for_direction(
    direction: Direction,
    grid: &Grid,
    (current_row_index, current_column_index): (usize, usize),
) -> Vec<u32> {
    let number_of_columns = grid[0].len();
    let number_of_rows = grid.len();

    match direction {
        Direction::Left => grid[current_row_index][0..current_column_index].to_vec(),
        Direction::Right => {
            grid[current_row_index][current_column_index + 1..number_of_columns].to_vec()
        }
        Direction::Up => {
            let up_range = 0..current_row_index;
            get_vertical_slice_of_trees(up_range, grid, current_column_index)
        }
        Direction::Down => {
            let down_range = current_row_index + 1..number_of_rows;
            get_vertical_slice_of_trees(down_range, grid, current_column_index)
        }
    }
}

fn get_vertical_slice_of_trees(
    range: Range<usize>,
    grid: &Grid,
    current_column_index: usize,
) -> Vec<u32> {
    let mut slice = Vec::new();

    for i in range {
        slice.push(grid[i][current_column_index]);
    }

    return slice;
}

fn is_reverse(direction: Direction) -> bool {
    match direction {
        Direction::Left | Direction::Up => true,
        Direction::Right | Direction::Down => false,
    }
}

fn get_number_of_visible_trees(
    direction: Direction,
    grid: &Grid,
    grid_position: (usize, usize),
    current_tree_height: &u32,
) -> u32 {
    let slice = get_slice_for_direction(direction, grid, grid_position);

    let mut number_of_trees_visible = 0;

    let mut range = slice.clone();

    if is_reverse(direction) {
        range.reverse();
    }

    for tree_height in range.iter() {
        number_of_trees_visible += 1;

        if tree_height >= current_tree_height {
            break;
        }
    }

    return number_of_trees_visible;
}

fn are_trees_shorter_than_current_tree(
    direction: Direction,
    grid: &Grid,
    grid_position: (usize, usize),
    current_tree_height: u32,
) -> bool {
    let slice = get_slice_for_direction(direction, grid, grid_position);

    let trees_shorter_than_current_tree = slice
        .iter()
        .filter(|tree_height| tree_height < &&current_tree_height)
        .collect::<Vec<&u32>>();

    return trees_shorter_than_current_tree.len() == slice.len();
}

fn is_at_edge_of_grid(
    grid: &Grid,
    (current_row_index, current_column_index): (usize, usize),
) -> bool {
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

fn is_tree_visible(grid: &Grid, grid_position: (usize, usize)) -> bool {
    let (current_row_index, current_column_index) = grid_position;
    let current_tree_height = grid[current_row_index][current_column_index];

    if is_at_edge_of_grid(grid, (current_row_index, current_column_index)) {
        return true;
    }

    if are_trees_shorter_than_current_tree(
        Direction::Left,
        grid,
        grid_position,
        current_tree_height,
    ) {
        return true;
    }

    if are_trees_shorter_than_current_tree(
        Direction::Right,
        grid,
        grid_position,
        current_tree_height,
    ) {
        return true;
    }

    if are_trees_shorter_than_current_tree(Direction::Up, grid, grid_position, current_tree_height)
    {
        return true;
    }

    if are_trees_shorter_than_current_tree(
        Direction::Down,
        grid,
        grid_position,
        current_tree_height,
    ) {
        return true;
    }

    return false;
}

fn get_score_for_tree(grid: &Grid, grid_position: (usize, usize)) -> u32 {
    let (current_row_index, current_column_index) = grid_position;

    let current_tree_height = &grid[current_row_index][current_column_index];

    let mut score = 1;

    score *= get_number_of_visible_trees(Direction::Left, grid, grid_position, current_tree_height);

    score *=
        get_number_of_visible_trees(Direction::Right, grid, grid_position, current_tree_height);

    score *= get_number_of_visible_trees(Direction::Up, grid, grid_position, current_tree_height);

    score *= get_number_of_visible_trees(Direction::Down, grid, grid_position, current_tree_height);

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
