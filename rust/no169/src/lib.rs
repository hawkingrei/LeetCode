pub fn majority_element(nums: Vec<i32>) -> i32 {
    let (mut gnum, mut gcount) = (0, 0);
    for num in &nums {
        if gnum == *num {
            gcount += 1;
            continue;
        }

        if gcount == 0 {
            gnum = *num;
            gcount = 1;
            continue;
        }
        gcount -= 1;
    }
    gcount = 0;
    for num in &nums {
        if *num == gnum {
            gcount += 1;
            continue;
        }
    }
    return gnum;
}

#[cfg(test)]
mod tests {
    use crate::majority_element;
    #[test]
    fn it_works() {
        assert_eq!(
            majority_element(vec![1, 2, 1, 1, 1, 3, 3, 4, 3, 3, 3, 4, 4, 4]),
            vec![3]
        );
    }
}
