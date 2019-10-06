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
                if is_palindrome(s[i..i + j +1 ].to_string()) {
                    dp[i][i + j] = true;
                    if j + 1 > max {
                        max = j + 1;
                        result = s[i..i + j+ 1].to_string();
                    }
                } else {
                    dp[i][i + j] = false;
                }
            } else {
                if i + 1 >= 0 && i + j - 1 < s.len() - 1 {
                    if dp[i + 1][i + j - 1] {
                        if s.chars().nth(i).unwrap() == s.chars().nth(i + j).unwrap() {
                            dp[i][i + j] = true;
                            if j + 1 > max {
                                max = j + 1;
                                result = s[i..i + j+1].to_string();
                            }
                        } else {
                            dp[i][i + j] = false
                        }
                    } else {
                        dp[i][i + j] = false
                    }
                } else {
                    dp[i][i + j] = is_palindrome(s[i..i + j+1].to_string())
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
