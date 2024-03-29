pub mod iterator_digit_finder;
pub mod numeric_digit_finder;
pub mod two_pointer_digit_finder;

pub trait DigitFinder {
    fn find_digit(&self, line: &str) -> u32;

    fn sum_calibration_numbers(&self, file_content: &str) -> u32 {
        file_content.lines().map(|line| self.find_digit(line)).sum()
    }

    fn name(&self) -> &'static str;
}
