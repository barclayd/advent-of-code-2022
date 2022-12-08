use std::fs;

fn get_marker_from_unique_characters(line: &str, unique_character_marker: usize) -> usize {
    let mut unique_characters: Vec<char> = vec![];
    let mut marker = 0;

    for (index, char) in line.chars().enumerate() {
        if unique_characters.contains(&char) {
            let index_of_unique_char = unique_characters
                .iter()
                .position(|&unique_char| unique_char == char)
                .unwrap();
            unique_characters.drain(0..=index_of_unique_char);
        }

        unique_characters.push(char);

        if unique_characters.len() == unique_character_marker {
            marker = index + 1;
            break;
        }
    }

    return marker;
}

fn get_marker_characters(file_path: &str, unique_character_marker: usize) -> Vec<usize> {
    let file_contents =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    return file_contents
        .lines()
        .map(|line| get_marker_from_unique_characters(line, unique_character_marker))
        .collect();
}

fn main() {
    get_marker_characters("./test.txt", 14);
}

#[cfg(test)]
mod tests {
    use crate::get_marker_characters;

    #[test]
    fn it_returns_expected_marker_characters_for_test_file() {
        let marker_characters = get_marker_characters("./test.txt", 4);
        assert_eq!(marker_characters, [7, 5, 6, 10, 11]);
    }

    #[test]
    fn it_returns_expected_marker_characters_for_test_file_2() {
        let marker_characters = get_marker_characters("./test2.txt", 14);
        assert_eq!(marker_characters, [19, 23, 23, 29, 26]);
    }

    #[test]
    fn it_returns_expected_marker_characters_for_input_file_part_1() {
        let marker_characters = get_marker_characters("./input.txt", 4);
        assert_eq!(marker_characters, [1794]);
    }

    #[test]
    fn it_returns_expected_marker_characters_for_input_file_part_2() {
        let marker_characters = get_marker_characters("./input.txt", 14);
        assert_eq!(marker_characters, [2851]);
    }
}
