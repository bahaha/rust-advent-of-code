use aoc23_day01::strategries::{iterator_digit_finder, two_pointer_digit_finder, DigitFinder};

fn main() {
    let strategies: Vec<Box<dyn DigitFinder>> = vec![
        Box::new(iterator_digit_finder::IteratorDigitFinder {}),
        Box::new(two_pointer_digit_finder::TwoPointersDigitFinder {}),
    ];

    for strategy in strategies {
        let sample_answer = strategy.sum_calibration_numbers(include_str!("./test_data/sample.txt"));
        let input_answer = strategy.sum_calibration_numbers(include_str!("./test_data/input.txt"));

        println!("[{}] Sample answer should be 142: {}", strategy.name(), sample_answer);
        println!("[{}] Input Answer: {}", strategy.name(), input_answer);
        println!();
    }
}

#[cfg(test)]
mod digit_finder_strategy {
    use super::*;

    #[test]
    fn test_iterator_digit_finder() {
        let finder = iterator_digit_finder::IteratorDigitFinder;
        assert_eq!(finder.sum_calibration_numbers(include_str!("./test_data/sample.txt")), 142);
        assert_eq!(finder.sum_calibration_numbers(include_str!("./test_data/input.txt")), 53334);

    }

    #[test]
    fn test_two_pointers_digit_finder() {
        let finder = two_pointer_digit_finder::TwoPointersDigitFinder;
        assert_eq!(finder.sum_calibration_numbers(include_str!("./test_data/sample.txt")), 142);
        assert_eq!(finder.sum_calibration_numbers(include_str!("./test_data/input.txt")), 53334);

    }
}
