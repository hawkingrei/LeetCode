pub fn max_product(nums: Vec<i32>) -> i32 {
    let mut max = 1;
    let mut min = 1;
    let mut fmax = i32::min_value();
    for j in 0..nums.len() {
        if nums[j] < 0 {
            let tmp = max;
            max = min;
            min = tmp
        }
        max = i32::max(max * nums[j], nums[j]);
        min = i32::min(min * nums[j], nums[j]);
        fmax = i32::max(max, fmax);
    }
    fmax
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
