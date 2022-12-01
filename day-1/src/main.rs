use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let file_contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let file_contents_by_line = file_contents.lines();

    let mut calories_per_elf = Vec::new();

    let mut count_for_elf: i128 = 0;

    for line in file_contents_by_line {

        if line.is_empty() {
            calories_per_elf.push(count_for_elf);
            count_for_elf = 0;
        } else {
            let value = line.parse::<i128>().unwrap();
            count_for_elf += value;
        }
    }

    calories_per_elf.sort_by(|a, b| b.cmp(a));

    // part 1

    println!("Highest number of calories: {}", calories_per_elf[0]);

    // part 2

    println!("Three highest calories combined: {}", calories_per_elf[0] + calories_per_elf[1] + calories_per_elf[2]);
}
