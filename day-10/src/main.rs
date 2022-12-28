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
    crt_screen: Vec<String>,
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
    let mut cycle_count = 0;
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
                let cycle_count_to_complete_on = cycle_count + execution_time;

                queue.insert(cycle_count_to_complete_on, register_modifier);

                cycle_count += execution_time;
            }
            Command::Noop => {
                cycle_count += 1;
            }
        }
    }

    let mut crt_screen: Vec<String> = Vec::new();

    let mut crt_line = "".to_string();

    for cycle_count_index in 1..=cycle_count {
        let cycle_count_number = cycle_count_index + 1;
        let previous_cycle_count_number = cycle_count_index - 1;

        if previous_cycle_count_number % 40 == 0 && !crt_line.is_empty() {
            crt_screen.push(crt_line);
            crt_line = "".to_string();
        }

        if previous_cycle_count_number % 40 < register_value - 1
            || previous_cycle_count_number % 40 > register_value + 1
        {
            crt_line.push('.');
        } else {
            crt_line.push('#');
        }

        if let Some(register_modifier) = queue.get(&cycle_count_index) {
            register_value += register_modifier;
        }
        if INTERESTING_SIGNAL_STRENGTHS_CYCLE_COUNTS.contains(&cycle_count_number) {
            interesting_signal_strengths.push(register_value * &cycle_count_number);
        }
    }

    crt_screen.push(crt_line);

    return Result {
        crt_screen,
        register_value,
        sum: interesting_signal_strengths.into_iter().sum(),
    };
}

fn main() {
    let result = get_register_values("./input.txt");
    for crt_line in result.crt_screen {
        println!("{}", crt_line);
    }
}

#[cfg(test)]
mod tests {
    use crate::get_register_values;
    use std::fs;

    #[test]
    fn it_returns_expected_x_register_value_for_sample_file() {
        let result = get_register_values("./sample.txt");
        assert_eq!(result.register_value, -1);
    }

    #[test]
    fn it_returns_expected_sum_of_signal_strengths_at_intervals_for_test_file() {
        let result = get_register_values("./test.txt");
        assert_eq!(result.sum, 13140);
    }

    #[test]
    fn it_returns_expected_sum_of_signal_strengths_at_intervals_for_input_file() {
        let result = get_register_values("./input.txt");
        assert_eq!(result.sum, 14820);
    }

    #[test]
    fn it_returns_expected_crt_output_for_test_file() {
        let result = get_register_values("./test.txt");

        let file_contents =
            fs::read_to_string("./test-crt.txt").expect("Should have been able to read the file");

        for (index, test_crt_line) in file_contents.lines().enumerate() {
            let crt_line = &result.crt_screen[index];
            assert_eq!(crt_line, test_crt_line);
        }
    }

    #[test]
    fn it_returns_expected_crt_output_for_input_file() {
        let result = get_register_values("./input.txt");

        let file_contents =
            fs::read_to_string("./input-crt.txt").expect("Should have been able to read the file");

        for (index, test_crt_line) in file_contents.lines().enumerate() {
            let crt_line = &result.crt_screen[index];
            assert_eq!(crt_line, test_crt_line);
        }
    }
}
