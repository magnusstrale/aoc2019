
fn is_valid(password: usize, has_double: bool, prev_digit: usize) -> bool {
    if password == 0 { return has_double; }
    let digit = password % 10;
    if digit > prev_digit { return false; }
    is_valid(password / 10, has_double || digit == prev_digit, digit)
}

fn count_valid(password: usize) -> usize {
    is_valid(password, false, 99) as usize
}

pub fn valid_in_range(from: usize, to: usize) -> usize {
    (from..=to).map(|password| count_valid(password)).sum()
}

fn strict_is_valid(password: usize, prev_digit: usize, repeat_count: usize, has_double: bool) -> bool {
    if password == 0 { return has_double || repeat_count == 1; }
    let digit = password % 10;
    if digit > prev_digit { return false; }
    if digit == prev_digit { 
        strict_is_valid(password / 10, digit, repeat_count + 1, has_double) 
    } else {
        strict_is_valid(password / 10, digit, 0, has_double || repeat_count == 1)
    }
}

fn strict_count_valid(password: usize) -> usize {
    strict_is_valid(password, 99, 0, false) as usize
}

pub fn strict_valid_in_range(from: usize, to: usize) -> usize {
    (from..=to).map(|password| strict_count_valid(password)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn password_111111_is_valid() {
        assert_eq!(count_valid(111111), 1);
    }
    
    #[test]
    fn password_122345_is_valid() {
        assert_eq!(count_valid(122345), 1);
    }
    
    #[test]
    fn password_111123_is_valid() {
        assert_eq!(count_valid(111123), 1);
    }
    
    #[test]
    fn password_112345_is_valid() {
        assert_eq!(count_valid(112345), 1);
    }
    
    #[test]
    fn password_223450_is_invalid_because_decreasing_digit() {
        assert_eq!(count_valid(223450), 0);
    }
    
    #[test]
    fn password_123789_is_invalid_because_no_double() {
        assert_eq!(count_valid(123789), 0);
    }
    
    #[test]
    fn password_112233_is_valid() {
        assert_eq!(strict_count_valid(112233), 1);
    }
    
    #[test]
    fn password_123444_is_invalid_because_three_digits_in_group() {
        assert_eq!(strict_count_valid(123444), 0);
    }
    
    #[test]
    fn password_111122_is_valid_because_double_two_digits_in_group() {
        assert_eq!(strict_count_valid(111122), 1);
    }
    
    #[test]
    fn password_1111_is_invalid_because_no_double_digits_in_group() {
        assert_eq!(strict_count_valid(1111), 0);
    }
    
    #[test]
    fn password_1123_is_valid() {
        assert_eq!(strict_count_valid(1123), 1);
    }
}