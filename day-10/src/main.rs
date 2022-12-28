fn get_register_value(file_path: &str) -> i32 {
    0
}

fn get_sum_of_signal_strength_at_intervals(file_path: &str) -> u32 {
    0
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::get_register_value;
    use crate::get_sum_of_signal_strength_at_intervals;

    #[test]
    fn it_returns_expected_x_register_value_for_sample_file() {
        let register_value = get_register_value("./sample.txt");
        assert_eq!(register_value, -1);
    }

    #[test]
    fn it_returns_expected_sum_of_signal_strengths_at_intervals_with_test_file() {
        let signal = get_sum_of_signal_strength_at_intervals("./test.txt");
        assert_eq!(signal, 13140);
    }

    #[test]
    fn it_returns_expected_sum_of_signal_strengths_at_intervals_with_input_file() {
        let signal = get_sum_of_signal_strength_at_intervals("./input.txt");
        assert_eq!(signal, 6464);
    }
}
