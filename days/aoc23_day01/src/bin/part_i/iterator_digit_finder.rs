use super::DigitFinder;

pub struct IteratorDigitFinder;

impl DigitFinder for IteratorDigitFinder {
    fn find_digits(&self, line: &str) -> u32 {
        let mut ten_digit = None;
        let mut unit_digit = None;

        for c in line.chars() {
            if ten_digit.is_some() {
                break;
            }
            if c.is_digit(10)  {
                ten_digit = c.to_digit(10);
            }
        }

        for c in line.chars().rev() {
            if unit_digit.is_some() {
                break;
            }
            if c.is_digit(10)  {
                unit_digit = c.to_digit(10);
            }
        }

        ten_digit.unwrap_or(0) * 10 + unit_digit.unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    
    #[test]
    fn test_find_digit_from_line() {
        let finder = IteratorDigitFinder;
        assert_eq!(finder.find_digits("1abc2"), 12);
        assert_eq!(finder.find_digits("pqr3stu8vwx"), 38);
        assert_eq!(finder.find_digits("a1b2c3d4e5f"), 15);
        assert_eq!(finder.find_digits("treb7uchet"), 77);
    }
}
