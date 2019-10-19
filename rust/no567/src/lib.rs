use std::collections::HashMap;

pub fn check_inclusion(s1: String, s2: String) -> bool {
    let mut windows = Vec::new();
    let mut checkmap: HashMap<char, i32> = HashMap::new();
    for chr in s1.chars() {
        if checkmap.contains_key(&chr) {
            if let Some(val) = checkmap.get_mut(&chr) {
                *val += 1;
            }
        } else {
            checkmap.insert(chr, 1);
        }
    }
    let mut windowmap: HashMap<char, i32> = HashMap::new();
    for chr in s2.chars() {
        if s1.contains(chr) {
            windows.push(chr);
            if windowmap.contains_key(&chr) {
                if let Some(val) = windowmap.get_mut(&chr) {
                    *val += 1;
                }
            } else {
                windowmap.insert(chr, 1);
            }
            if windows.len() == s1.len() {
                if windowmap == checkmap {
                    return true;
                } else {
                    let remove_item = windows.remove(0);
                    if let Some(val) = windowmap.get_mut(&remove_item) {
                        if *val > 1 {
                            *val -= 1;
                        } else {
                            windowmap.remove(&remove_item);
                        }
                    }
                }
            }
        } else {
            windows.clear();
            windowmap.clear();
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use crate::check_inclusion;
    #[test]
    fn it_works() {
        assert_eq!(check_inclusion("adc".to_string(), "dcda".to_string()), true);
        assert_eq!(
            check_inclusion("ab".to_string(), "eidbaooo".to_string()),
            true
        );
    }
}
