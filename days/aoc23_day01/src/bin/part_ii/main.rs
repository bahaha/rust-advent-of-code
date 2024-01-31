fn main() {
    let input = include_str!("./test_data/input.txt");
    println!("Sum of calibration numbers: {}", sum_numeric_calibration_nums(input));
}

fn sum_numeric_calibration_nums(message: &str) -> u32 {
    message.lines().map(|line| find_digit(line)).sum()
}

fn find_numeric_digit(text: &str, forward: bool) -> Option<u32> {
    let mut potential_digit = None;
    let len = text.len();
    let char_at = |i: usize| if forward { text.chars().nth(i) } else { text.chars().nth(len - 1 - i) }; 

    for i in 0..len {
        let c = char_at(i)?;
        potential_digit = c.to_digit(10);
        for digits in 3..=5 {
            if potential_digit.is_none() && i + digits < len {
                let start = if forward { i } else { len - (i + digits) };
                let num_candidate = &text[start..start+digits];
                potential_digit = num_candidate.to_digit();
            }
        }
        if potential_digit.is_some() {
            break;
        }
    }
    potential_digit
}

fn find_digit(line: &str) -> u32 {
    let ten_digit = find_numeric_digit(line, true);
    let unit_digit = find_numeric_digit(line, false);
    ten_digit.unwrap_or(0) * 10 + unit_digit.unwrap_or(0)
}

trait Numeric {
    fn to_digit(&self) -> Option<u32>;
}

impl Numeric for &str {
    fn to_digit(&self) -> Option<u32>{
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
            _ => None
        }
    }
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
