use std::fs;

fn get_totals(file_path: &str) -> Vec<u32> {
    let mut totals: Vec<u32> = Vec::new();
    let mut stack: Vec<u32> = Vec::new();

    let file_contents =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    for line in file_contents.lines() {
        if line.starts_with("$ cd") {
            let cd_arg = line.split(' ').skip(2).next().unwrap();
            if cd_arg == ".." {
                // Exit current directory.  Save its total size.
                totals.push(stack.pop().unwrap());
            } else {
                // Entering a new directory
                // Initialize the size (so far) to 0.
                stack.push(0);
            }
        } else {
            // Try to parse a number at the start of the line (i.e. a file size)
            let file_name_start = line.split(' ').next().unwrap();
            if let Ok(file_size) = file_name_start.parse::<u32>() {
                // Add the size of this file to all ancestor directories
                for dir in stack.iter_mut() {
                    *dir += file_size;
                }
            }
        }
    }

    // Pop any directories still on the stack
    while let Some(v) = stack.pop() {
        totals.push(v);
    }

    return totals;
}

fn get_sum_of_totals(file_path: &str) -> u32 {
    let totals = get_totals(file_path);

    totals
        .into_iter()
        .filter(|total| total < &100000)
        .sum::<u32>()
}

fn get_size_of_smallest_directory_large_enough_to_be_removed(file_path: &str) -> u32 {
    let mut totals = get_totals(file_path);

    let total_amount_of_unused_space = totals.last().unwrap();

    let remaining_unused_disk_space = 70000000 - total_amount_of_unused_space;

    let space_required_to_free_up = 30000000 - remaining_unused_disk_space;

    totals.sort();

    let smallest_directories_large_enough_to_be_removed = totals
        .into_iter()
        .filter(|total| total > &space_required_to_free_up)
        .collect::<Vec<u32>>();

    return smallest_directories_large_enough_to_be_removed[0];
}

fn main() {
    get_sum_of_totals("./test.txt");
    get_size_of_smallest_directory_large_enough_to_be_removed("./input.txt");
}

#[cfg(test)]
mod tests {
    use crate::get_size_of_smallest_directory_large_enough_to_be_removed;
    use crate::get_sum_of_totals;

    #[test]
    fn it_returns_expected_sum_of_totals_for_test_file() {
        let sum_of_totals = get_sum_of_totals("./test.txt");
        assert_eq!(sum_of_totals, 95437);
    }

    #[test]
    fn it_returns_expected_sum_of_totals_for_input_file() {
        let sum_of_totals = get_sum_of_totals("./input.txt");
        assert_eq!(sum_of_totals, 1428881);
    }

    #[test]
    fn it_returns_expected_size_of_directory_to_be_removed_for_test_file() {
        let sum_of_totals = get_size_of_smallest_directory_large_enough_to_be_removed("./test.txt");
        assert_eq!(sum_of_totals, 24933642);
    }

    #[test]
    fn it_returns_expected_size_of_directory_to_be_removed_for_input_file() {
        let sum_of_totals =
            get_size_of_smallest_directory_large_enough_to_be_removed("./input.txt");
        assert_eq!(sum_of_totals, 10475598);
    }
}
