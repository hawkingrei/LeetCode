use std::convert::TryInto;

pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
    let len = words.len() - 1;
    let mut result: Vec<String> = Vec::new();
    let mut index = 0;
    let mut tmp_vec: Vec<String> = Vec::new();
    let mut tmp_count = 0;
    let mut do_it = false;
    loop {
        if do_it || (index > len && !tmp_vec.is_empty()) {
            let mut first_space = true;
            let mut final_word = "".to_string();
            let mut words_num = tmp_vec.len();
            let mut space_num = if words_num == 1 { 1 } else { words_num - 1 };
            let mut space_additional = 0;
            let mut space_avg = 0;
            if (index > len && !tmp_vec.is_empty()) {
                space_additional = 0;
                space_avg = 1;
                space_num = if words_num == 1 { 1 } else { words_num };
            } else {
                space_additional = (max_width - tmp_count) % space_num as i32;
                space_avg = (max_width - tmp_count - space_additional) / space_num as i32;
            }

            while words_num != 0 || space_num != 0 {
                if words_num != 0 {
                    final_word.push_str(&tmp_vec.get(tmp_vec.len() - words_num).unwrap());
                    words_num -= 1;
                }
                if space_num != 0 {
                    space_num -= 1;
                    if first_space {
                        final_word.push_str(
                            &(0..space_avg + space_additional)
                                .map(|_| " ")
                                .collect::<String>(),
                        );
                        first_space = false;
                        continue;
                    }
                    final_word.push_str(&(0..space_avg).map(|_| " ").collect::<String>());
                }
            }
            result.push(final_word);
            tmp_vec.clear();
            tmp_count = 0;
            do_it = false;
        }
        if index > len {
            break;
        }
        let word = words.get(index).unwrap();
        if tmp_count + word.len() as i32 <= (max_width - tmp_vec.len() as i32).try_into().unwrap() {
            tmp_count += word.len() as i32;
            tmp_vec.push(word.to_string());
            index += 1;
        } else {
            do_it = true;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::full_justify;
    #[test]
    fn it_works() {
        let words: Vec<String> = vec![
            "This".to_string(),
            "is".to_string(),
            "an".to_string(),
            "example".to_string(),
            "of".to_string(),
            "text".to_string(),
            "justification.".to_string(),
        ];
        let maxWidth = 16;
        println!("{:?}", full_justify(words, maxWidth));
    }
}
