use std::collections::HashMap;
use std::convert::TryInto;

use rand::{thread_rng, Rng};

struct RandomizedSet {
    index: HashMap<i32, usize>,
    data: Vec<Option<i32>>,
    empty: Vec<usize>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Self {
            index: HashMap::new(),
            data: Vec::new(),
            empty: Vec::new(),
        }
    }

    /** Inserts a value to the set. Returns true if the set did not already contain the specified element. */
    fn insert(&mut self, val: i32) -> bool {
        if self.index.contains_key(&val) {
            return false;
        }
        if self.empty.len() == 0 {
            self.data.push(Some(val));
            self.index.insert(val, self.data.len() - 1);
        } else {
            let data_index = self.empty.pop().unwrap();
            self.data[data_index] = Some(val);
            self.index.insert(val, data_index);
        }
        return true;
    }

    /** Removes a value from the set. Returns true if the set contained the specified element. */
    fn remove(&mut self, val: i32) -> bool {
        let index = self.index.get(&val);
        if index.is_none() {
            return false;
        }
        if let Some(data_index) = index {
            self.empty.push(*data_index);
            if let Some(elem) = self.data.get_mut(*data_index) {
                *elem = None;
            }
            self.index.remove(&val);
            
        }
        true
    }

    /** Get a random element from the set. */
    fn get_random(&self) -> i32 {
        loop {
            let mut rng = thread_rng();
            let n: usize = rng.gen_range(0, self.data.len()).try_into().unwrap();
            if let Some(data) = self.data.get(n) {
                if !data.is_none() {
                    return data.unwrap();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
