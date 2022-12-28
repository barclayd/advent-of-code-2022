use std::collections::HashMap;
use std::fs;

#[derive(PartialEq)]
enum Command {
    Noop,
    Addx,
}

struct Result {
    sum: i32,
    register_value: i32,
}

impl Command {
    fn parse(command: &str) -> Self {
        match command {
            "noop" => Self::Noop,
            "addx" => Self::Addx,
            _ => panic!("Unsupported command '${command}'"),
        }
    }
}

fn get_register_values(file_path: &str) -> Result {
    let file_contents =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut queue: HashMap<i32, i32> = HashMap::new();
    let mut cycle_counts = 1;
    let mut register_value = 1;

    const INTERESTING_SIGNAL_STRENGTHS_CYCLE_COUNTS: [i32; 6] = [20, 60, 100, 140, 180, 220];
    let mut interesting_signal_strengths: Vec<i32> = Vec::new();

    for line in file_contents.lines() {
        let command_arguments: Vec<&str> = line.split(" ").collect();
        let command = Command::parse(command_arguments[0]);

        match command {
            Command::Addx => {
                let register_modifier = command_arguments[1].parse::<i32>().unwrap();

                let execution_time = 2;
                let cycle_count_to_complete_on = cycle_counts + execution_time;

                queue.insert(cycle_count_to_complete_on, register_modifier);

                cycle_counts += execution_time;
            }
            Command::Noop => {
                cycle_counts += 1;
            }
        }
    }

    for cycle_count_index in 1..cycle_counts {
        let cycle_count_number = cycle_count_index + 1;

        if let Some(register_modifier) = queue.get(&cycle_count_number) {
            register_value += register_modifier;
        }
        if INTERESTING_SIGNAL_STRENGTHS_CYCLE_COUNTS.contains(&cycle_count_number) {
            interesting_signal_strengths.push(register_value * &cycle_count_number);
        }
    }

    return Result {
        register_value,
        sum: interesting_signal_strengths.into_iter().sum(),
    };
}

fn main() {
    get_register_values("./input.txt");
}

#[cfg(test)]
mod tests {
    use crate::get_register_values;

    #[test]
    fn it_returns_expected_x_register_value_for_sample_file() {
        let result = get_register_values("./sample.txt");
        assert_eq!(result.register_value, -1);
    }

    #[test]
    fn it_returns_expected_sum_of_signal_strengths_at_intervals_with_test_file() {
        let result = get_register_values("./test.txt");
        assert_eq!(result.sum, 13140);
    }

    #[test]
    fn it_returns_expected_sum_of_signal_strengths_at_intervals_with_input_file() {
        let result = get_register_values("./input.txt");
        assert_eq!(result.sum, 14820);
    }
}
