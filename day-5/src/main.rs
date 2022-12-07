use std::fs;
use substring::Substring;

fn get_numbers_from_command(command: &str, index: usize) -> usize {
    let preceding_char = command.chars().nth(index - 1);
    let number = command.chars().nth(index);
    let succeeding_char = command.chars().nth(index + 1);

    if preceding_char.is_none() && number.is_none() && succeeding_char.is_none() {
        panic!("No number found");
    }

    if !preceding_char.is_none() && !number.is_none() && !succeeding_char.is_none() {
        let first_number_as_string = preceding_char.unwrap().to_string();
        let second_number_as_string = number.unwrap().to_string();
        let third_number_as_string = succeeding_char.unwrap().to_string();
        let full_string = format!(
            "{}{}{}",
            first_number_as_string, second_number_as_string, third_number_as_string
        );

        return full_string.replace(" ", "").parse::<usize>().unwrap();
    }

    return number.unwrap().to_digit(10).unwrap() as usize;
}

fn apply_command(stacks: &mut Vec<Vec<char>>, command: &str) {
    let quantity_to_move = get_numbers_from_command(command, 5);
    let mut index_for_original_stack_number = 12;
    let mut index_for_new_stack_number = 17;
    if quantity_to_move > 9 {
        index_for_original_stack_number = 13;
        index_for_new_stack_number = 18;
    }
    let original_stack_number_index =
        get_numbers_from_command(command, index_for_original_stack_number) - 1;
    let new_stack_number_index = get_numbers_from_command(command, index_for_new_stack_number) - 1;

    let stacks_to_move: Vec<char> = stacks[original_stack_number_index][0..quantity_to_move]
        .iter()
        // .rev()
        .cloned()
        .collect();
    stacks[new_stack_number_index] = [
        stacks_to_move.as_slice(),
        stacks[new_stack_number_index].as_slice(),
    ]
    .concat();
    stacks[original_stack_number_index].drain(0..quantity_to_move);
}

fn remove_whitespace(string: &str) -> String {
    string
        .chars()
        .filter(|char| !char.is_whitespace())
        .collect()
}

fn get_top_layer_of_stacks(stacks: &mut Vec<Vec<char>>) -> String {
    return stacks
        .iter()
        .map(|stack| stack.first().unwrap())
        .into_iter()
        .collect();
}

fn get_stacks_of_crates(file_path: &str) -> String {
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

    let stacks: &mut Vec<Vec<char>> = &mut Vec::new();

    for _ in 0..number_of_stacks {
        stacks.push(vec![]);
    }

    for (index, stack_line) in file_contents.lines().enumerate() {
        let line_number = index + 1;
        if line_number == stack_numbering_line as usize {
            break;
        }

        let mut string_stack_index: (usize, usize) = (0, 0);

        while string_stack_index.0 < number_of_stacks as usize {
            let crate_to_add =
                stack_line.substring(string_stack_index.1 + 1, string_stack_index.1 + 2);

            if remove_whitespace(crate_to_add).len() > 0 {
                stacks[string_stack_index.0].push(crate_to_add.chars().nth(0).unwrap());
            }
            string_stack_index.0 += 1;
            string_stack_index.1 += 4;
        }
    }

    for line in file_contents
        .lines()
        .skip((stack_numbering_line + 1) as usize)
    {
        apply_command(stacks, line);
    }

    return get_top_layer_of_stacks(stacks);
}

fn main() {
    get_stacks_of_crates("./input.txt");
}

#[cfg(test)]
mod tests {
    use crate::get_stacks_of_crates;

    #[test]
    fn it_returns_expected_message_for_test_file() {
        let number_of_overlaps = get_stacks_of_crates("./test.txt");
        assert_eq!(number_of_overlaps, "CMZ");
    }

    #[test]
    fn it_returns_expected_message_for_input_file() {
        let number_of_overlaps = get_stacks_of_crates("./input.txt");
        assert_eq!(number_of_overlaps, "RFFFWBPNS");
    }
}
