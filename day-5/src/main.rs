use regex::Regex;
use std::fs;
use substring::Substring;

const DIGITS_ONLY_REGEX: &str = r"^[0-9]*$";

fn remove_whitespace(string: &str) -> String {
    string
        .chars()
        .filter(|char| !char.is_whitespace())
        .collect()
}

fn get_crates_by_line(file_path: &str) {
    let file_contents =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut number_of_stacks: u32 = 0;
    let mut stack_numbering_line: u32 = 0;

    for (index, line) in file_contents.lines().enumerate() {
        let line_without_whitespace = remove_whitespace(line);

        let is_row_of_numbers = line_without_whitespace
            .chars()
            .all(|char| char::is_digit(char, 10))
            && line_without_whitespace.len() > 0;

        if is_row_of_numbers {
            number_of_stacks = line_without_whitespace
                .chars()
                .last()
                .unwrap()
                .to_digit(10)
                .unwrap();
            stack_numbering_line = (index + 1) as u32;
        }
    }

    let mut stacks: Vec<Vec<&str>> = Vec::new();

    for _ in 0..number_of_stacks {
        stacks.push(vec![]);
    }

    for (index, stack_line) in file_contents.lines().enumerate() {
        let line_number = index + 1;
        if line_number == stack_numbering_line as usize {
            println!("line number at break: {}", line_number);
            break;
        }

        let mut string_stack_index: (usize, usize) = (0, 0);

        while string_stack_index.0 < number_of_stacks as usize {
            let crate_to_add =
                stack_line.substring(string_stack_index.1 + 1, string_stack_index.1 + 2);

            if remove_whitespace(crate_to_add).len() > 0 {
                stacks[string_stack_index.0].push(crate_to_add);
            }
            string_stack_index.0 += 1;
            string_stack_index.1 += 4;
        }
    }

    println!("{:?}", stacks);
}

fn main() {
    get_crates_by_line("./test.txt")
}
