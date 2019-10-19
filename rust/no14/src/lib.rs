pub fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.len() == 0 {
        return "".to_string();
    }
    if strs.len() == 1 {
        return strs[0].clone();
    }
    let min = strs
        .clone()
        .into_iter()
        .map(|s| s.len())
        .fold(std::usize::MAX, |mut acc, x| {
            if acc > x {
                acc = x;
            }
            acc
        });
    if min == 0 {
        return "".to_string();
    }
    let mut index = 0;
    let strs_len = strs.len();
    while index <= min - 1 {
        for vec_index in 1..strs_len {
            if strs[vec_index].chars().nth(index).unwrap()
                != strs[vec_index - 1].chars().nth(index).unwrap()
            {
                return strs[0].get(..index).unwrap().to_string();
            }
        }
        index = index + 1;
    }
    return strs[0].get(..index).unwrap().to_string();
}

#[cfg(test)]
mod tests {
    use crate::longest_common_prefix;
    #[test]
    fn it_works() {
        
        assert_eq!(
            longest_common_prefix(vec![
                "flower".to_string(),
                "flow".to_string(),
                "flight".to_string()
            ]),
            "fl"
        );
        assert_eq!(
            longest_common_prefix(vec!["fl".to_string(), "fl".to_string(), "fl".to_string()]),
            "fl"
        );        
        assert_eq!(
            longest_common_prefix(vec!["a".to_string(), "b".to_string(), "fl".to_string()]),
            ""
        );
        assert_eq!(
            longest_common_prefix(vec!["a".to_string(), "a".to_string()]),
            "a"
        );
    }
}
