use std::collections::HashMap;
use std::convert::TryInto;

use rand::{thread_rng, Rng};

struct RandomizedCollection {
    index: HashMap<i32, Vec<usize>>,
    data: Vec<Option<i32>>,
    empty: Vec<usize>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedCollection {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Self {
            index: HashMap::new(),
            data: Vec::new(),
            empty: Vec::new(),
        }
    }

    /** Inserts a value to the collection. Returns true if the collection did not already contain the specified element. */
    fn insert(&mut self, val: i32) -> bool {
        if self.empty.len() == 0 {
            self.data.push(Some(val));
            if self.index.contains_key(&val) {
                self.index.get_mut(&val).unwrap().push(self.data.len()-1);
            } else {
                self.index.insert(val, vec![self.data.len() - 1]);
            }
            
        } else {
            let data_index = self.empty.pop().unwrap();
            self.data[data_index] = Some(val);
            if self.index.contains_key(&val) {
                self.index.get_mut(&val).unwrap().push(self.data.len()-1);
            } else {
                self.index.insert(val, vec![self.data.len() - 1]);
            }
        }
        return true;
    }

    /** Removes a value from the collection. Returns true if the collection contained the specified element. */
    fn remove(&mut self, val: i32) -> bool {
        let index = self.index.get_mut(&val);
        if index.is_none() {
            return false;
        }
        if let Some(mut index_vec) = index {
            let data_index = index_vec.pop().unwrap();
            self.empty.push(data_index);
            if let Some(elem) = self.data.get_mut(data_index) {
                *elem = None;
            }
            if index_vec.len() == 0 {
                self.index.remove(&val);
            }
        }
        true
    }

    /** Get a random element from the collection. */
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
