use aoc23_day01::strategries::{numeric_digit_finder, DigitFinder};

fn main() {
    let finder = numeric_digit_finder::NumericDigitFinder {};
    let input = include_str!("./test_data/input.txt");
    println!(
        "Sum of calibration numbers: {}",
        finder.sum_calibration_numbers(input)
    );
}

#[cfg(test)]
mod tests {
    use aoc23_day01::strategries::{numeric_digit_finder, DigitFinder};

    #[test]
    fn test_sum_numbers_per_line() {
        let finder = numeric_digit_finder::NumericDigitFinder {};
        let sample = include_str!("./test_data/sample.txt");
        assert_eq!(finder.sum_calibration_numbers(sample), 281);

        let input = include_str!("./test_data/input.txt");
        assert_eq!(finder.sum_calibration_numbers(input), 52834);
    }
}
