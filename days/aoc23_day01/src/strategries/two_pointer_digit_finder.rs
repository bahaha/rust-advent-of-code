use super::DigitFinder;

pub struct TwoPointersDigitFinder;

impl DigitFinder for TwoPointersDigitFinder {
    fn name(&self) -> &'static str {
        "Two pointers"
    }

    fn find_digit(&self, line: &str) -> u32 {
        let bytes = line.as_bytes();
        let mut ten_digit = None;
        let mut unit_digit = None;

        for i in 0..bytes.len() {
            if ten_digit.is_some() && unit_digit.is_some() {
                break;
            }

            if ten_digit.is_none() && (bytes[i] as char).is_digit(10) {
                ten_digit = (bytes[i] as char).to_digit(10);
            }

            let tail_char = bytes[bytes.len() - i - 1] as char;
            if unit_digit.is_none() && tail_char.is_digit(10) {
                unit_digit = tail_char.to_digit(10);
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
        let finder = TwoPointersDigitFinder;
        assert_eq!(finder.find_digit("1abc2"), 12);
        assert_eq!(finder.find_digit("pqr3stu8vwx"), 38);
        assert_eq!(finder.find_digit("a1b2c3d4e5f"), 15);
        assert_eq!(finder.find_digit("treb7uchet"), 77);
    }
}
