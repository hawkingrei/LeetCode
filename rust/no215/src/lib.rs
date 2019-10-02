impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        if nums.len() == 1 && k == 1 {
            return nums[0];
        }
        let mut sort_vec = nums.clone();
        sort_vec.sort();
        *(sort_vec.get(nums.len() - k as usize).unwrap())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
