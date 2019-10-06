pub fn my_atoi(str: String) -> i32 {
    let mut start = false;
    let mut result: i32 = 0;
    let mut neg = false;
    let mut close = false;
    let mut has_neg = false;
    for c in str.chars() {
        match c {
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                if !start {
                    start = true;
                }
                if close {
                    continue;
                }
                let is_over = || {
                    if neg {
                        return Some(i32::min_value());
                    } else {
                        return Some(i32::max_value());
                    }
                };
                result = match result.checked_mul(10) {
                    Some(m) => match c {
                        '0' => m,
                        '1' => m.checked_add(1).or_else(is_over).unwrap(),
                        '2' => m.checked_add(2).or_else(is_over).unwrap(),
                        '3' => m.checked_add(3).or_else(is_over).unwrap(),
                        '4' => m.checked_add(4).or_else(is_over).unwrap(),
                        '5' => m.checked_add(5).or_else(is_over).unwrap(),
                        '6' => m.checked_add(6).or_else(is_over).unwrap(),
                        '7' => m.checked_add(7).or_else(is_over).unwrap(),
                        '8' => m.checked_add(8).or_else(is_over).unwrap(),
                        '9' => m.checked_add(9).or_else(is_over).unwrap(),
                        _ => m,
                    },
                    None => {
                        if neg {
                            return i32::min_value();
                        } else {
                            return i32::max_value();
                        }
                    }
                }
            }
            '-' => {
                if !start && has_neg {
                    return 0;
                }
                has_neg = true;
                if start {
                    close = true;
                    start = true;
                    continue
                }
                start = true;
                neg = true;
            }
            '+' => {
                if !start && has_neg {
                    return 0;
                }
                has_neg = true;
                if start {
                    close = true;
                }
                start = true;
            }
            '.' => {
                close = true;
            }
            ' ' => {
                if start {
                    close = true;
                }
            }
            _ => {
                if !start {
                    result = 0;
                    break;
                }
                close = true;
            }
        }
    }
    if neg {
        return -1 * result;
    }
    return result;
}
#[cfg(test)]
mod tests {
    use crate::my_atoi;
    #[test]
    fn it_works() {
        assert_eq!(my_atoi("4193 with words".to_string()), 4193);
        assert_eq!(my_atoi("w4193 with words".to_string()), 0);
        assert_eq!(my_atoi("   -42".to_string()), -42);
        assert_eq!(my_atoi("-91283472332".to_string()), -2147483648);
        assert_eq!(my_atoi("2147483648".to_string()), 2147483647);
        assert_eq!(my_atoi("-5-".to_string()), -5);
        assert_eq!(my_atoi("-13+8".to_string()), -13);
        assert_eq!(my_atoi("123-".to_string()), 123);
        assert_eq!(my_atoi("123+".to_string()), 123);
    }
}
