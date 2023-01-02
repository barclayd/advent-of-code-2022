use std::fs;

#[derive(Debug, Default, Clone)]
enum Operation {
    #[default]
    Noop,
    Multiply(i64),
    Add(i64),
    MultiplySelf,
    AddSelf,
}

impl Operation {
    fn calculate(&self, value: i64) -> i64 {
        match self {
            Self::AddSelf => value + value,
            Self::MultiplySelf => value * value,
            Self::Add(n) => value + *n,
            Self::Multiply(n) => value * *n,
            Self::Noop => panic!("Tried to process noop"),
        }
    }
}

#[derive(Debug, Default, Clone)]
struct Monkey {
    items: Vec<i64>,
    operation: Operation,
    test: i64,
    destination: (usize, usize),
    count: usize,
}

fn process_round(monkeys: &mut Vec<Monkey>, is_part_1: bool) {
    let mod_value: i64 = monkeys.iter().map(|monkey| monkey.test).product();
    println!("{}", mod_value);
    for i in 0..monkeys.len() {
        while let Some(item) = monkeys[i].items.pop() {
            let current_monkey = &mut monkeys[i];
            let worry = if is_part_1 {
                current_monkey.operation.calculate(item) / 3
            } else {
                current_monkey.operation.calculate(item) % mod_value
            };
            let destination = if worry % current_monkey.test == 0 {
                current_monkey.destination.0
            } else {
                current_monkey.destination.1
            };
            monkeys[destination].items.push(worry);
            monkeys[i].count += 1;
        }
    }
}

fn get_monkey_business_level_after_rounds(file_path: &str, rounds: i64, is_part_1: bool) -> usize {
    let file_contents =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut monkeys: Vec<Monkey> = Vec::new();

    let mut monkey = Monkey::default();

    for line in file_contents.lines().filter(|line| line.trim().len() > 1) {
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

    for _ in 0..rounds {
        process_round(&mut monkeys, is_part_1);
    }

    let mut monkey_business = monkeys
        .iter()
        .map(|monkey| monkey.count)
        .collect::<Vec<usize>>();

    monkey_business.sort();
    monkey_business.reverse();

    return monkey_business[0] * monkey_business[1];
}

fn main() {
    let part_1_monkey_business_level =
        get_monkey_business_level_after_rounds("./test.txt", 20, true);
    println!("Part 1: Monkey business level={part_1_monkey_business_level}");

    let part_2_monkey_business_level =
        get_monkey_business_level_after_rounds("./test.txt", 10000, false);
    println!("Part 2: Monkey business level={part_2_monkey_business_level}");
}

#[cfg(test)]
mod tests {
    use crate::get_monkey_business_level_after_rounds;

    #[test]
    fn it_returns_expected_levels_of_monkey_business_for_test_file_and_rounds() {
        let level = get_monkey_business_level_after_rounds("./test.txt", 20, true);
        assert_eq!(level, 10605);
    }

    #[test]
    fn it_returns_expected_levels_of_monkey_business_for_input_file_and_rounds() {
        let level = get_monkey_business_level_after_rounds("./input.txt", 20, true);
        assert_eq!(level, 55944);
    }

    #[test]
    fn it_returns_expected_levels_of_monkey_business_with_new_rules_for_test_file_and_rounds() {
        let level = get_monkey_business_level_after_rounds("./test.txt", 10000, false);
        assert_eq!(level, 2713310158);
    }

    #[test]
    fn it_returns_expected_levels_of_monkey_business_with_new_rules_for_input_file_and_rounds() {
        let level = get_monkey_business_level_after_rounds("./input.txt", 10000, false);
        assert_eq!(level, 15117269860);
    }
}
