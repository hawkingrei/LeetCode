use std::convert::TryInto;

pub fn repeated_string_match(a: String, b: String) -> i32 {
    let mut data: String = "".to_string();
    let lena = a.len();
    let lenb = b.len();
    let mut i = 0;
    while i < (lenb / lena) + 2 {
        data.push_str(&a);
        if data.contains(&b) {
            return (i + 1).try_into().unwrap();
        }
        i = i + 1;
    }
    return -1;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
