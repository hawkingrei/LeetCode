pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
    if nums.len() <= 2 {
        if nums.len() == 2 && nums[0] == nums[1] {
            return vec![nums[0]];
        }
        return nums;
    }

    let (mut num1, mut num2, mut count1, mut count2) = if nums[0] != nums[1] {
        (nums[0], nums[1], 1, 1)
    } else {
        (nums[0], 0, 2, 0)
    };
    for (pos, num) in nums.iter().enumerate() {
        if pos == 0 || pos == 1 {
            continue;
        }
        if num1 == *num {
            count1 += 1;
            continue;
        }
        if num2 == *num {
            count2 += 1;
            continue;
        }
        if count1 == 0 {
            num1 = *num;
            count1 = 1;
            continue;
        }
        if count2 == 0 {
            num2 = *num;
            count2 = 1;
            continue;
        }
        count1 -= 1;
        count2 -= 1;
    }
    count1 = 0;
    count2 = 0;
    for num in &nums {
        if *num == num1 {
            count1 += 1;
            continue;
        }
        if *num == num2 {
            count2 += 1;
            continue;
        }
    }
    let mut result: Vec<i32> = Vec::new();
    if count1 > nums.len() / 3 {
        result.push(num1);
    }
    if count2 > nums.len() / 3 {
        result.push(num2);
    }
    return result;
}

#[cfg(test)]
mod tests {
    use crate::majority_element;
    #[test]
    fn it_works() {
        assert_eq!(majority_element(vec![1,2,1,1,1,3,3,4,3,3,3,4,4,4]), vec![3]);
    }
}
