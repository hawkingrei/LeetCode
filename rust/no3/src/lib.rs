use std::collections::HashSet;
use std::convert::TryInto;
pub fn length_of_longest_substring(s: String) -> i32 {
    if s.len() == 0 {
        return 0;
    }
    let mut vec_char = vec![];
    let mut max = 0;
    for chr in s.chars() {
        if vec_char.len() == 0 || !vec_char.contains(&chr) {
            vec_char.push(chr);
        } else {
            if vec_char.len() > max {
                max = vec_char.len()
            }
            while vec_char.contains(&chr) {
                vec_char.remove(0);
            }
            vec_char.push(chr);
        }
    }
    if vec_char.len() > max {
        max = vec_char.len();
    }
    max.try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::length_of_longest_substring;
    #[test]
    fn it_works() {
        assert_eq!(length_of_longest_substring("abcabcbb".to_string()), 3);
    }
}
