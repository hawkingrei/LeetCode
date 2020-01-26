use std::cmp;
use std::i32;

pub fn min_moves(nums: Vec<i32>) -> i32 {
    let mut total = 0;
    let mut min_mun = i32::max_value();
    let mut nums_len = 0;
    for num in &nums {
        total = total + num;
        min_mun = cmp::min(min_mun, *num);
        nums_len = nums_len + 1;
    }
    return total - min_mun * nums_len;
}
