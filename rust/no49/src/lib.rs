use std::collections::HashMap;

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut map = HashMap::new();
    for s in strs {
        let mut key = [0; 26];
        for a in s.chars() {
            key[(a as usize - 'a' as usize)] += 1;
        }
        map.entry(key).or_insert(Vec::new()).push(s);
    }
    map.into_iter().map(|(_, v)| v).collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
