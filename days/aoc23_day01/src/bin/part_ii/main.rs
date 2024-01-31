fn main() {
    let input = include_str!("./test_data/input.txt");
    println!(
        "Sum of calibration numbers: {}",
        sum_numeric_calibration_nums(input)
    );
}

trait Numeric {
    fn to_digit(&self) -> Option<u32>;
}

impl Numeric for &str {
    fn to_digit(&self) -> Option<u32> {
        match *self {
            "one" => Some(1),
            "two" => Some(2),
            "three" => Some(3),
            "four" => Some(4),
            "five" => Some(5),
            "six" => Some(6),
            "seven" => Some(7),
            "eight" => Some(8),
            "nine" => Some(9),
            _ => None,
        }
    }
}

fn find_digit(line: &str) -> u32 {
    let ten_digit = find_numeric_digit(line, true);
    let unit_digit = find_numeric_digit(line, false);
    ten_digit.unwrap_or(0) * 10 + unit_digit.unwrap_or(0)
}

fn sum_numeric_calibration_nums(message: &str) -> u32 {
    message.lines().map(|line| find_digit(line)).sum()
}

fn find_numeric_digit(text: &str, forward: bool) -> Option<u32> {
    let chars = text.chars().collect::<Vec<char>>();
    let len = chars.len();

    for i in 0..len {
        let c = if forward {
            chars[i]
        } else {
            chars[len - i - 1]
        };
        if let Some(digit) = c.to_digit(10) {
            return Some(digit);
        }
        for digits in 3..=5 {
            if i + digits < len {
                let start = if forward { i } else { len - (i + digits) };
                let numeric_text = &text[start..start + digits];
                if let Some(digit) = numeric_text.to_digit() {
                    return Some(digit);
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_digits() {
        assert_eq!(find_digit("two1nine"), 29);
        assert_eq!(find_digit("eightwothree"), 83);
        assert_eq!(find_digit("abcone2threexyz"), 13);
        assert_eq!(find_digit("xtwone3four"), 24);
        assert_eq!(find_digit("4nineeightseven2"), 42);
        assert_eq!(find_digit("zoneight234"), 14);
        assert_eq!(find_digit("7pqrstsixteen"), 76);
    }

    #[test]
    fn test_sum_numbers_per_line() {
        let sample = include_str!("./test_data/sample.txt");
        assert_eq!(sum_numeric_calibration_nums(sample), 281);

        let input = include_str!("./test_data/input.txt");
        assert_eq!(sum_numeric_calibration_nums(input), 52834);
    }
}
