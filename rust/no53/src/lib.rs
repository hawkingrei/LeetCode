pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut max = i32::min_value();
    let mut curr = 0;
    for j in 0..nums.len() {
        curr = curr + nums[j];
        max = i32::max(curr, max);
        if curr < 0 {
            curr = 0
        }
    }
    max
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
