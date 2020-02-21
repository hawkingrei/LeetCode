use std::collections::HashMap;

pub fn letter_combinations(digits: String) -> Vec<String> {
    if digits.len() == 0 {
        return vec![];
    }
    let mut table: HashMap<String, Vec<String>> = HashMap::new();
    table.insert(
        '2'.to_string(),
        vec!["a".to_string(), "b".to_string(), "c".to_string()],
    );
    table.insert(
        '3'.to_string(),
        vec!["d".to_string(), "e".to_string(), "f".to_string()],
    );
    table.insert(
        '4'.to_string(),
        vec!["g".to_string(), "h".to_string(), "i".to_string()],
    );
    table.insert(
        '5'.to_string(),
        vec!["j".to_string(), "k".to_string(), "l".to_string()],
    );
    table.insert(
        '6'.to_string(),
        vec!["m".to_string(), "n".to_string(), "o".to_string()],
    );
    table.insert(
        '7'.to_string(),
        vec![
            "p".to_string(),
            "q".to_string(),
            "r".to_string(),
            "s".to_string(),
        ],
    );
    table.insert(
        '8'.to_string(),
        vec!["t".to_string(), "u".to_string(), "v".to_string()],
    );
    table.insert(
        '9'.to_string(),
        vec!["w".to_string(), "x".to_string(), "y".to_string(), "z".to_string()],
    );
    let mut result: Vec<String> = vec!["".to_string()];
    for digit in digits.chars() {
        let maptable = table.get(&digit.to_string()).unwrap();
        let mut tmp :Vec<String> = Vec::new();
        for ch in maptable {
            for rs in &result {
                tmp.push(format!("{}{}",ch,rs));
            }
        }
        result = tmp;
    }
    return result;
}
