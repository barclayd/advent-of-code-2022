use std::fs;

#[derive(Debug, Default, Clone)]
enum Operation {
    #[default]
    Noop,
    Multiply(i32),
    Add(i32),
    MultiplySelf,
    AddSelf,
}

#[derive(Debug, Default, Clone)]
struct Monkey {
    items: Vec<i32>,
    operation: Operation,
    test: i32,
    destination: (usize, usize),
}

fn get_monkey_business_level_after_rounds(file_path: &str, rounds: i32) -> i32 {
    let file_contents =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut monkeys: Vec<Monkey> = Vec::new();

    let mut monkey = Monkey::default();

    for line in file_contents.lines().filter(|line| line.trim().len() > 1) {
        println!("line: {line}");
        let words = line.trim().split(' ').collect::<Vec<&str>>();
        match words[0] {
            "Monkey" => monkey = Monkey::default(),
            "Starting" => {
                let (_, str_list) = line.split_once(": ").unwrap();
                monkey.items = str_list
                    .split(", ")
                    .map(|word| word.parse().unwrap())
                    .collect()
            }
            "Operation:" => {
                monkey.operation = if words[4] == "+" {
                    if words[5] == "old" {
                        Operation::AddSelf
                    } else {
                        Operation::Add(words[5].parse().unwrap())
                    }
                } else {
                    if words[5] == "old" {
                        Operation::MultiplySelf
                    } else {
                        Operation::Multiply(words[5].parse().unwrap())
                    }
                }
            }
            "Test:" => monkey.test = words[3].parse().unwrap(),
            "If" => {
                if words[1] == "true:" {
                    monkey.destination.0 = words[5].parse().unwrap();
                } else {
                    monkey.destination.1 = words[5].parse().unwrap();
                    monkeys.push(monkey.clone());
                    monkey = Monkey::default();
                }
            }
            _ => panic!("Not yet implemented for line {}", line),
        }
    }

    println!("Monkeys {:?}", monkeys);

    0
}

fn main() {
    let level = get_monkey_business_level_after_rounds("./input.txt", 20);
    println!("Monkey business level: {level}");
}

#[cfg(test)]
mod tests {
    use crate::get_monkey_business_level_after_rounds;

    #[test]
    fn it_returns_expected_x_register_value_for_test_file() {
        let level = get_monkey_business_level_after_rounds("./test.txt", 20);
        assert_eq!(level, 10605);
    }

    #[test]
    fn it_returns_expected_x_register_value_for_input_file() {
        let level = get_monkey_business_level_after_rounds("./input.txt", 20);
        assert_eq!(level, 10605);
    }
}
