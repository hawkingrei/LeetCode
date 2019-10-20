use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Eq)]
struct Node {
    internal: (String, i32),
}

impl Node {
    fn key(&self) -> String {
        self.internal.0.clone()
    }

    fn val(&self) -> i32 {
        self.internal.1
    }

    fn inc(&mut self) {
        self.internal.1 += 1;
    }

    fn dec(&mut self) {
        self.internal.1 -= 1;
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.internal.1.cmp(&other.internal.1))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.internal.1 == other.internal.1
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.internal.1.cmp(&other.internal.1)
    }
}

struct List {
    list: Vec<Node>,
}

impl List {
    fn new() -> Self {
        Self { list: Vec::new() }
    }

    fn insert(&mut self, key: String) {
        let context: Node = Node { internal: (key, 1) };
        let idx = self.list.binary_search(&context).unwrap_or_else(|x| x);
        self.list.insert(idx, context);
    }

    fn inc(&mut self, key: String) {
        for context in self.list.iter() {
            if context.key() == key {
                let new_context = Node {
                    internal: (context.key(), context.val() + 1),
                };
                let idx = self.list.binary_search(&new_context).unwrap_or_else(|x| x);
                self.list.insert(idx, new_context);
                return;
            }
        }
    }

    fn dec(&mut self, key: String) {
        for context in self.list.iter() {
            if context.key() == key {
                let new_context = Node {
                    internal: (context.key(), context.val() - 1),
                };
                let idx = self.list.binary_search(&new_context).unwrap_or_else(|x| x);
                self.list.insert(idx, new_context);
                return;
            }
        }
    }

    fn remove(&mut self) {
        self.list.pop();
    }

    fn max(&self) -> String {
        self.list.last().unwrap().key()
    }

    fn min(&self) -> String {
        self.list.first().unwrap().key()
    }
}

struct AllOne {
    map: HashMap<String, i32>,
    list: List,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl AllOne {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Self {
            map: HashMap::new(),
            list: List::new(),
        }
    }

    /** Inserts a new key <Key> with value 1. Or increments an existing key by 1. */
    fn inc(&mut self, key: String) {
        if !self.map.contains_key(&key) {
            self.map.insert(key.clone(), 1);
            self.list.insert(key);
        } else {
            if let Some(val) = self.map.get_mut(&key) {
                *val += 1;
                self.list.inc(key);
            }
        }
    }

    /** Decrements an existing key by 1. If Key's value is 1, remove it from the data structure. */
    fn dec(&mut self, key: String) {
        if self.map.contains_key(&key) {
            if let Some(val) = self.map.get_mut(&key) {
                self.list.dec(key.clone());
                if *val == 1 {
                    self.map.remove(&key.clone());
                    self.list.remove();
                } else {
                    *val -= 1;
                }
            }
        }
    }

    /** Returns one of the keys with maximal value. */
    fn get_max_key(&self) -> String {
        if self.map.len() == 0 {
            return "".to_string();
        }
        self.list.min()
    }

    /** Returns one of the keys with Minimal value. */
    fn get_min_key(&self) -> String {
        if self.map.len() == 0 {
            return "".to_string();
        }
        self.list.max()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
