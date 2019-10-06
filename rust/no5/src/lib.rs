pub fn longest_palindrome(s: String) -> String {
    if s.len() <= 1 {
        return s;
    }
    let mut dp = vec![vec![false; s.len()]; s.len()];
    let mut max = 1;
    let mut result = s[..1].to_string();

    // init 2
    for j in 1..s.len() {
        for i in 0..s.len() {
            if i + j > s.len() - 1 {
                continue;
            }
            if j <= 2 {
                if is_palindrome(s[i..i + j + 1].to_string()) {
                    dp[i][i + j] = true;
                    if j + 1 > max {
                        max = j + 1;
                        result = s[i..i + j + 1].to_string();
                    }
                } else {
                    dp[i][i + j] = false;
                }
            } else {
                if dp[i + 1][i + j - 1] {
                    if s.chars().nth(i).unwrap() == s.chars().nth(i + j).unwrap() {
                        dp[i][i + j] = true;
                        if j + 1 > max {
                            max = j + 1;
                            result = s[i..i + j + 1].to_string();
                        }
                    } else {
                        dp[i][i + j] = false
                    }
                } else {
                    dp[i][i + j] = false
                }
            }
        }
    }
    return result;
}

fn is_palindrome(s: String) -> bool {
    let nums: usize = s.len() / 2;
    for index in 0..nums + 1 {
        if index != s.len() - index {
            if s.chars().nth(index).unwrap() != s.chars().nth(s.len() - 1 - index).unwrap() {
                return false;
            }
        }
    }
    return true;
}

pub fn longest_palindrome2(s: String) -> String {
    let seq: Vec<char> = s.chars().collect();
    let len = seq.len();
    if len < 1 {
        return s;
    }
    let (mut idx, mut curr_len, mut curr_start, mut curr_end) = (0, 0, 0, 0);
    while idx < len {
        let (mut i, mut j) = (idx, idx);
        let ch = seq[idx];
        // handle same char
        while i > 0 && seq[i - 1] == ch {
            i -= 1
        }
        while j < len - 1 && seq[j + 1] == ch {
            j += 1
        }
        idx = j + 1;
        while i > 0 && j < len - 1 && seq[i - 1] == seq[j + 1] {
            i -= 1;
            j += 1;
        }
        let max_len = j - i + 1;
        if max_len > curr_len {
            curr_len = max_len;
            curr_start = i;
            curr_end = j;
        }
        if max_len >= len - 1 {
            break;
        }
    }

    s[curr_start..curr_end + 1].to_owned()
}

#[cfg(test)]
mod tests {
    use crate::is_palindrome;
    use crate::longest_palindrome;
    #[test]
    fn it_works() {
        assert_eq!(longest_palindrome("".to_string()), "".to_string());
        assert_eq!(longest_palindrome("aa".to_string()), "aa".to_string());
        assert_eq!(longest_palindrome("a".to_string()), "a".to_string());
        assert_eq!(longest_palindrome("babad".to_string()), "bab".to_string());
        assert_eq!(is_palindrome("bab".to_string()), true);
        assert_eq!(is_palindrome("baab".to_string()), true);
        assert_eq!(is_palindrome("babc".to_string()), false);
        assert_eq!(is_palindrome("bacab".to_string()), true);
    }
}
