mod iterator_digit_finder;
mod two_pointer_digit_finder;

pub trait DigitFinder {
    fn find_digits(&self, line: &str) -> u32;

    fn sum_calibration_numbers(&self, file_content: &str) -> u32 {
        file_content
            .lines()
            .map(|line| self.find_digits(line))
            .sum()
    }
}

enum DigitFinderStrategy {
    Iterator(iterator_digit_finder::IteratorDigitFinder),
    TwoPointers(two_pointer_digit_finder::TwoPointersDigitFinder),
}

impl DigitFinder for DigitFinderStrategy {
    fn find_digits(&self, line: &str) -> u32 {
        match self {
            DigitFinderStrategy::Iterator(finder) => finder.find_digits(line),
            DigitFinderStrategy::TwoPointers(finder) => finder.find_digits(line),
        }
    }
}

fn main() {
    let strategies = vec![
        DigitFinderStrategy::Iterator(iterator_digit_finder::IteratorDigitFinder {}),
        DigitFinderStrategy::TwoPointers(two_pointer_digit_finder::TwoPointersDigitFinder {}),
    ];

    for strategy in strategies {
        let sample_answer = strategy.sum_calibration_numbers(include_str!("./test_data/sample.txt"));
        let input_answer = strategy.sum_calibration_numbers(include_str!("./test_data/input.txt"));

        match strategy {
            DigitFinderStrategy::Iterator(_) => println!("[Iterator]"),
            DigitFinderStrategy::TwoPointers(_) => println!("[Two pointers]"),
        }
        println!("Sample answer should be 142: {}", sample_answer);
        println!("Input Answer: {}", input_answer);
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
