pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();
    let tmp: Vec<i32> = Vec::new();
    dfs(nums, tmp, &mut result);
    return result;
}

fn dfs(input: Vec<i32>, tmp: Vec<i32>, result: &mut Vec<Vec<i32>>) {
    if input.len() == 0 {
        result.push(tmp);
        return;
    }
    for num in &input {
        let mut next_input = input.clone();
        let mut next_tmp = tmp.clone();
        let pos = next_input.iter().position(|x| *x == *num).unwrap();
        next_input.remove(pos);
        next_tmp.push(*num);
        dfs(next_input, next_tmp, result);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
