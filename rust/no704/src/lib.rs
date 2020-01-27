pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut lo = 0i32;
    let mut hi = (nums.len() as i32) - 1;
    while lo <= hi {
        let mid = lo + (hi - lo) / 2;
        match nums[mid as usize].cmp(&target) {
            Ordering::Less => {
                lo = mid + 1;
            }
            Ordering::Greater => {
                hi = mid - 1;
            }
            Ordering::Equal => {
                return mid;
            }
        }
    }
    -1
}

pub fn search2(nums: Vec<i32>, target: i32) -> i32 {
    if nums.len() == 1 {
        if *nums.get(0).unwrap() == target {
            return 0;
        }
        return -1;
    }
    if nums.len() == 2 {
        if *nums.get(0).unwrap() == target {
            return 0;
        }
        if *nums.get(1).unwrap() == target {
            return 1;
        }
        return -1;
    }

    let mut max_index = nums.len() - 1;
    let mut min_index = 0;
    let mut middle_index = if nums.len() > 2 { nums.len() / 2 } else { 0 };
    loop {
        let match_nums = nums.get(middle_index).unwrap();
        if *match_nums == target {
            return middle_index as i32;
        }
        if *match_nums < target {
            if max_index - middle_index < 1 {
                break;
            }
            if max_index - middle_index == 1 {
                if middle_index == min_index {
                    break;
                }
                min_index = middle_index;
                middle_index =  middle_index + 1;
            } else {
                min_index = middle_index;
                middle_index = middle_index + (max_index - middle_index) / 2;
            }
            
        } else {
            if middle_index - min_index < 1 {
                break;
            }
            if middle_index - min_index == 1 {
                if middle_index == max_index {
                    break;
                }
                max_index = middle_index;
                middle_index = middle_index  -1;
            } else {
                max_index = middle_index;
                middle_index = min_index + (middle_index - min_index) / 2
            }
            
        }

    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_792() {
        assert_eq!(search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
        assert_eq!(search(vec![-1, 0, 3, 5, 9, 12], 2), -1);
        assert_eq!(search(vec![1], 1), 0);
        assert_eq!(search(vec![5], -5), -1);
        assert_eq!(search(vec![5], 6), -1);
        assert_eq!(search(vec![1, 2], 0), -1);
        assert_eq!(search(vec![1, 2], 1), 0);
        assert_eq!(search(vec![1, 2], 2), 1);
        assert_eq!(search(vec![1, 2], 3), -1);
        assert_eq!(search(vec![-1,0,3,5,9,12],3),2);
    }
}
