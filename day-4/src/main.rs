use std::{cmp, fs};

fn are_ranges_fully_overlapping(first_range: (i32, i32), second_range: (i32, i32)) -> bool {
    let start = cmp::max(first_range.0, second_range.0);
    let end = cmp::min(first_range.1, second_range.1);

    return (start, end) == first_range || (start, end) == second_range;
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

fn get_number_of_overlaps(file_path: &str) -> i32 {
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

    return ranges
        .iter()
        .map(|range| {
            if are_ranges_fully_overlapping(range.0, range.1) {
                return 1;
            }
            return 0;
        })
        .sum();
}

fn main() {
    let number_of_overlaps = get_number_of_overlaps("./test.txt");
    println!("{}", number_of_overlaps);
}

#[cfg(test)]
mod tests {
    use crate::get_number_of_overlaps;

    #[test]
    fn it_returns_expected_number_of_overlaps_for_test_file() {
        let number_of_overlaps = get_number_of_overlaps("./test.txt");
        assert_eq!(number_of_overlaps, 2);
    }

    #[test]
    fn it_returns_expected_number_of_overlaps_for_input_file() {
        let number_of_overlaps = get_number_of_overlaps("./input.txt");
        assert_eq!(number_of_overlaps, 500);
    }
}
