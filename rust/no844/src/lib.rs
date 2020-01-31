use std::convert::TryInto;

pub fn backspace_compare(s: String, t: String) -> bool {
    let mut s_len: isize = s.len() as isize - 1;
    let mut t_len: isize = t.len() as isize - 1;
    let mut s_skip = 0;
    let mut t_skip = 0;
    while s_len >= 0 || t_len >= 0 {
        while s_len >= 0 {
            if s.chars().nth(s_len.try_into().unwrap()).unwrap() == '#' {
                s_skip += 1;
                s_len -= 1;
            } else if s_skip > 0 {
                s_skip -= 1;
                s_len -= 1;
            } else {
                break;
            }
        }

        while t_len >= 0 {
            if t.chars().nth(t_len.try_into().unwrap()).unwrap() == '#' {
                t_skip += 1;
                t_len -= 1;
            } else if t_skip > 0 {
                t_skip -= 1;
                t_len -= 1;
            } else {
                break;
            }
        }
        if t_len >= 0
            && s_len >= 0
            && t.chars().nth(t_len.try_into().unwrap()).unwrap()
                != s.chars().nth(s_len.try_into().unwrap()).unwrap()
        {
            return false;
        }
        if ((s_len >= 0) != (t_len >= 0)) {
            return false;
        }

        s_len -= 1;
        t_len -= 1;
    }
    return true;
}

#[cfg(test)]
mod tests {
    use crate::backspace_compare;
    #[test]
    fn it_works() {
        assert_eq!(
            backspace_compare("bbbextm".to_string(), "bbb#extm".to_string()),
            false
        );
    }
}
