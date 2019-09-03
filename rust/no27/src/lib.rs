pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let len = nums.len();
    if len == 0 {
        return 0;
    }

    let ptr = nums.as_mut_ptr();
    let mut next_read: usize = 0;
    let mut next_write: usize = 0;
    unsafe {
        while next_read < len {
            let ptr_read = ptr.add(next_read);
            let ptr_write = ptr.add(next_write);
            if *ptr_read != val {
                if next_read != next_write {
                    std::mem::swap(&mut *ptr_read, &mut *ptr_write);
                }
                next_write += 1
            }
            next_read += 1;
        }
    }
    nums.truncate(next_write);
    return next_write as i32;
}

#[cfg(test)]
mod tests {

    use crate::remove_element;
    #[test]
    fn it_works() {
        let mut test_data = vec![3,4,4,3,4];
        assert_eq!(remove_element(&mut test_data, 4),2);
        assert_eq!(test_data, vec![3,3]);
    }
}
