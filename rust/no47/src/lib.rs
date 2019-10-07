#![feature(vec_remove_item)]

use std::collections::HashSet;

pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();
    let tmp: Vec<i32> = Vec::new();
    dfs(nums, tmp, &mut result);
    return result;
}

fn dfs(input: Vec<i32>, tmp: Vec<i32>, result: &mut Vec<Vec<i32>>) {
    let mut set = HashSet::new();
    if input.len() == 0 {
        result.push(tmp);
        return;
    }
    for num in &input {
        if !set.contains(&num) {
            set.insert(num);
            let mut next_input = input.clone();
            let mut next_tmp = tmp.clone();
            let pos = next_input.iter().position(|x| *x == *num).unwrap();
            next_input.remove(pos);
            next_tmp.push(*num);
            dfs(next_input, next_tmp, result);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
