extern crate core;

use itertools::Itertools;
use std::fs;

const ASCII_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

fn get_shared_letter_from_codes(codes: &(String, String)) -> char {
    let mut shared_letter: Option<char> = None;

    for letter_in_code_one in codes.0.chars() {
        if codes.1.contains(letter_in_code_one) {
            shared_letter = Option::from(letter_in_code_one);
        }

        if let Some(_char) = shared_letter {
            break;
        }
    }

    return shared_letter.unwrap();
}

fn get_common_letter_in_string(strings: &Vec<&str>) -> char {
    let first_code = strings[0];
    let second_code = strings[1];
    let third_code = strings[2];

    for letter in first_code.chars() {
        if second_code.contains(letter) && third_code.contains(letter) {
            return letter;
        }
    }

    panic!("No common letters found!");
}

fn to_chunks(string: &str, chunk_size: usize) -> (String, String) {
    let mut sections: (String, String) = ("".to_string(), "".to_string());

    let mut index = 0;
    for chunk in &string.chars().chunks(chunk_size) {
        let string_from_chunk = String::from_iter(chunk);
        if index % 2 == 0 {
            sections.0 = string_from_chunk
        } else {
            sections.1 = string_from_chunk
        }
        index += 1;
    }

    return sections;
}

fn get_priority_for_char(char: &char) -> i16 {
    let mut base_priority: i16 = 0;

    if char.is_uppercase() {
        base_priority = 26;
    }

    let letter_to_find: String = char.to_lowercase().to_string();

    let index = ASCII_LOWER
        .iter()
        .position(|letter| letter.to_string() == letter_to_find)
        .unwrap() as i16;

    return index + 1 + base_priority;
}

fn get_sum_of_priorities(file_path: &str) -> i16 {
    let file_contents =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    let file_contents_by_line = file_contents.lines();

    let mut codes: Vec<(String, String)> = Vec::new();

    for line in file_contents_by_line {
        codes.push(to_chunks(line, line.len() / 2));
    }

    let shared_letters = codes
        .iter()
        .map(|codes| get_shared_letter_from_codes(codes))
        .collect::<Vec<char>>();

    return shared_letters
        .iter()
        .map(|letter| get_priority_for_char(letter))
        .sum::<i16>();
}

fn get_sum_of_priorities_by_group(file_path: &str) -> i16 {
    let file_contents =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    let file_contents_by_line = file_contents.lines().chunks(3);

    let group_codes = file_contents_by_line
        .into_iter()
        .map(|line| Vec::from_iter(line))
        .collect::<Vec<Vec<&str>>>();

    let common_letters = group_codes
        .iter()
        .map(|group_code| get_common_letter_in_string(group_code))
        .collect::<Vec<char>>();

    return common_letters
        .iter()
        .map(|letter| get_priority_for_char(letter))
        .sum::<i16>();
}

fn main() {
    let sum_of_priorities = get_sum_of_priorities("./input.txt");
    println!("Part 1: {}", sum_of_priorities);
    let sum_of_priorities_by_group = get_sum_of_priorities_by_group("./input.txt");
    println!("Part 2: {}", sum_of_priorities_by_group);
}

#[cfg(test)]
mod tests {
    use crate::get_sum_of_priorities;
    use crate::get_sum_of_priorities_by_group;

    #[test]
    fn it_returns_expected_result_for_test_file() {
        let sum_of_priorities = get_sum_of_priorities("./test.txt");
        assert_eq!(sum_of_priorities, 157);
    }

    #[test]
    fn it_returns_expected_result_for_input_file() {
        let sum_of_priorities = get_sum_of_priorities("./input.txt");
        assert_eq!(sum_of_priorities, 7811);
    }
    #[test]
    fn get_sum_of_priorities_by_group_for_test_file() {
        let sum_of_priorities = get_sum_of_priorities_by_group("./test.txt");
        assert_eq!(sum_of_priorities, 70);
    }

    #[test]
    fn get_sum_of_priorities_by_group_for_input_file() {
        let sum_of_priorities = get_sum_of_priorities_by_group("./input.txt");
        assert_eq!(sum_of_priorities, 2639);
    }
}
