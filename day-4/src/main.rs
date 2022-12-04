use std::{cmp, fs};

fn are_ranges_fully_overlapping(first_range: (i32, i32), second_range: (i32, i32)) -> bool {
    let start_of_overlap = cmp::max(first_range.0, second_range.0);
    let end_of_overlap = cmp::min(first_range.1, second_range.1);

    return (start_of_overlap, end_of_overlap) == first_range
        || (start_of_overlap, end_of_overlap) == second_range;
}

fn are_ranges_overlapping(first_range: (i32, i32), second_range: (i32, i32)) -> bool {
    return cmp::max(first_range.0, second_range.0) <= cmp::min(first_range.1, second_range.1);
}

fn convert_dashed_range_to_tuple(dashed_range: &str) -> (i32, i32) {
    let number_range = dashed_range.split("-").collect::<Vec<&str>>();

    let mut tuple = (0, 0);

    if let [first_number, second_number] = &number_range[..] {
        tuple.0 = first_number.parse().unwrap();
        tuple.1 = second_number.parse().unwrap();
    }

    return tuple;
}

fn get_ranges_from_file(file_path: &str) -> Vec<((i32, i32), (i32, i32))> {
    let file_contents =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    let file_contents_by_line = file_contents.lines();

    let mut ranges: Vec<((i32, i32), (i32, i32))> = Vec::new();

    for line in file_contents_by_line {
        let string_ranges_for_line = line.split(",").collect::<Vec<&str>>();

        let first_tuple = convert_dashed_range_to_tuple(string_ranges_for_line[0]);
        let second_tuple = convert_dashed_range_to_tuple(string_ranges_for_line[1]);

        ranges.push((first_tuple, second_tuple));
    }

    return ranges;
}

fn get_number_of_overlaps(
    file_path: &str,
    are_overlaps: &dyn Fn((i32, i32), (i32, i32)) -> bool,
) -> i32 {
    let ranges = get_ranges_from_file(file_path);

    return ranges
        .iter()
        .map(|range| {
            if are_overlaps(range.0, range.1) {
                return 1;
            }
            return 0;
        })
        .sum();
}

fn main() {
    // part 1
    let number_of_full_overlaps =
        get_number_of_overlaps("./test.txt", &are_ranges_fully_overlapping);
    println!("Part 1: {}", number_of_full_overlaps);
    // part 2
    let number_of_overlaps = get_number_of_overlaps("./test.txt", &are_ranges_overlapping);
    println!("Part 2: {}", number_of_overlaps);
}

#[cfg(test)]
mod tests {
    use crate::are_ranges_fully_overlapping;
    use crate::are_ranges_overlapping;
    use crate::get_number_of_overlaps;

    #[test]
    fn it_returns_expected_number_of_full_overlaps_for_test_file() {
        let number_of_overlaps =
            get_number_of_overlaps("./test.txt", &are_ranges_fully_overlapping);
        assert_eq!(number_of_overlaps, 2);
    }

    #[test]
    fn it_returns_expected_number_of_full_overlaps_for_input_file() {
        let number_of_overlaps =
            get_number_of_overlaps("./input.txt", &are_ranges_fully_overlapping);
        assert_eq!(number_of_overlaps, 500);
    }

    #[test]
    fn it_returns_expected_number_of_overlaps_for_test_file() {
        let number_of_overlaps = get_number_of_overlaps("./test.txt", &are_ranges_overlapping);
        assert_eq!(number_of_overlaps, 4);
    }

    #[test]
    fn it_returns_expected_number_of_overlaps_for_input_file() {
        let number_of_overlaps = get_number_of_overlaps("./input.txt", &are_ranges_overlapping);
        assert_eq!(number_of_overlaps, 815);
    }
}
