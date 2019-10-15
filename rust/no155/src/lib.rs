struct MinStack {
    stack :Vec<i32>,
    min: Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {

    /** initialize your data structure here. */
    fn new() -> Self {
        Self{
            stack: Vec::new(),
            min: Vec::new(),
        }
    }
    
    fn push(&mut self, x: i32) {
        self.stack.push(x);
        if self.min.len() == 0 {
            self.min.push(x);
        } else {
            let last = self.min.last().unwrap();
            if last > &x {
                self.min.push(x);
            } else {
                self.min.push(*last);
            }
        }
    }
    
    fn pop(&mut self) {
        self.stack.pop();
        self.min.pop();
    }
    
    fn top(&self) -> i32 {
        *self.stack.last().unwrap()
    }
    
    fn get_min(&self) -> i32 {
        *self.min.last().unwrap()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
