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
        // println!("edge");
        return true;
    }

    if is_tree_visible_from_left(grid, grid_position) {
        println!("left, coords: {:?}", grid[grid_position.0][grid_position.1]);
        return true;
    }

    if is_tree_visible_from_up(grid, grid_position) {
        println!("up, coords: {:?}", grid[grid_position.0][grid_position.1]);
        return true;
    }

    if is_tree_visible_from_right(grid, grid_position) {
        println!(
            "right, coords: {:?}",
            grid[grid_position.0][grid_position.1]
        );
        return true;
    }

    if is_tree_visible_from_down(grid, grid_position) {
        println!("down, coords: {:?}", grid[grid_position.0][grid_position.1]);
        return true;
    }

    return false;
}

fn count_visible_trees(grid: &Grid) -> usize {
    let mut count = 0;
    grid.iter().enumerate().for_each(|(row_index, row)| {
        row.iter().enumerate().for_each(|(column_index, _)| {
            if is_tree_visible(grid, (row_index, column_index)) {
                count += 1;
            }
        });
    });
    return count;
}

fn count_number_of_visible_trees(file_path: &str) {
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

    let count = count_visible_trees(&grid);

    println!("Count {:?}", count);
}

fn main() {
    count_number_of_visible_trees("./input.txt");
}
