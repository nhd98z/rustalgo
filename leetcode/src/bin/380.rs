#![allow(dead_code)]

use rand::Rng;

struct RandomizedSet {
    data: Vec<i32>,
    map: std::collections::HashMap<i32, usize>,
    rng: rand::rngs::ThreadRng,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {
    fn new() -> Self {
        RandomizedSet {
            data: Vec::new(),
            map: std::collections::HashMap::new(),
            rng: rand::thread_rng(),
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        if self.map.contains_key(&val) {
            return false;
        }
        self.data.push(val);
        self.map.insert(val, self.data.len() - 1);
        true
    }

    fn remove(&mut self, val: i32) -> bool {
        if let Some(&idx) = self.map.get(&val) {
            let last = self.data.len() - 1;
            let last_val = self.data[last];
            self.data[idx] = last_val;
            self.map.insert(last_val, idx);
            self.data.pop();
            self.map.remove(&val);
            true
        } else {
            false
        }
    }

    fn get_random(&mut self) -> i32 {
        let idx = self.rng.gen_range(0..self.data.len());
        self.data[idx]
    }
}

struct Solution;

fn main() {
    let mut obj = RandomizedSet::new();
    assert_eq!(obj.insert(1), true);
    assert_eq!(obj.remove(2), false);
    assert_eq!(obj.insert(2), true);
    assert_eq!(obj.get_random(), 1);
    assert_eq!(obj.remove(1), true);
    assert_eq!(obj.insert(2), false);
    assert_eq!(obj.get_random(), 2);
}
