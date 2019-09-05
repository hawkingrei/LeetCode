pub fn str_str(haystack: String, needle: String) -> i32 {
    if needle.len() == 0 {
        return 0;
    }
    if haystack.len() < needle.len() {
        return -1;
    }
    if haystack == needle {
        return 0;
    }

    let h_ptr = haystack.as_bytes();
    let n_ptr = needle.as_bytes();
    let mut h_offset = 0;

    while h_offset <= h_ptr.len() - 1 {
        let mut n_offset = 0;
        let mut tmp_h_offset = h_offset;
        loop {
            if h_ptr[tmp_h_offset] == n_ptr[n_offset] {
                n_offset += 1;
                if n_offset > n_ptr.len() - 1 {
                    return (tmp_h_offset - n_ptr.len() + 1) as i32;
                }
                tmp_h_offset += 1;
                if tmp_h_offset > h_ptr.len() - 1 {
                    return -1;
                }
            } else {
                break;
            }
        }
        h_offset += 1;
    }
    return -1;
}

#[cfg(test)]
mod tests {
    use crate::str_str;
    #[test]
    fn it_works() {
        assert_eq!(str_str("hello".to_string(), "ll".to_string()), 2);
        assert_eq!(str_str("mississippi".to_string(), "issip".to_string()), 4);
        assert_eq!(str_str("mississippi".to_string(), "sippia".to_string()), -1);
    }
}
