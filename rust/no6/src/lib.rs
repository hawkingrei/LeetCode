pub fn convert(s: String, num_rows: i32) -> String {
    if num_rows == 1 {
        return s;
    }
    let mut inter: Vec<Vec<String>> = vec![Vec::new(); num_rows as usize];
    let mut up = false;
    let mut index: usize = 0;
    let mut result: String = "".to_string();
    for c in s.chars() {
        if index == 0 || index % (num_rows as usize - 1) == 0 {
            up = !up;
        }
        inter[index].push(c.to_string());
        if up {
            index += 1;
        } else {
            index -= 1;
        }
    }
    for big in inter {
        for small in big {
            result.push_str(&small);
        }
    }
    return result;
}

#[cfg(test)]
mod tests {

    use crate::convert;
    #[test]
    fn it_works() {
        assert_eq!(
            convert("LEETCODEISHIRING".to_string(), 3),
            "LCIRETOESIIGEDHN".to_string()
        );
        assert_eq!(convert("AB".to_string(), 1), "AB".to_string());
    }
}
