struct TwoSum {
    ctx :Vec<i32>,

}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TwoSum {

    /** Initialize your data structure here. */
    fn new() -> Self {
        Self {
            ctx: Vec::new()
        }
    }
    
    /** Add the number to an internal data structure.. */
    fn add(&mut self, number: i32) {
        self.ctx.push(number);
    }
    
    /** Find if there exists any pair of numbers which sum is equal to the value. */
    fn find(&mut self, value: i32) -> bool {
        if self.ctx.len() == 0 {
            return false;
        }
        let mut low = 0;
        self.ctx.sort()
        let mut high = self.ctx.len() - 1;
        while (low != high) {
            let result = self.ctx[low] + self.ctx[high] - value;
            if result > 0 {
                high = high -1;
                continue;
            }
            if result == 0 {
                return true;
            }
            if result < 0 {
                low = low + 1;
                continue;
            }

        }
        return false;
    }
}
