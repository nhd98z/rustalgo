use std::collections::HashMap;

struct FindSumPairs {
    nums1: Vec<i32>,
    nums2: Vec<i32>,
    freq2: HashMap<i32, i32>,
}

impl FindSumPairs {
    fn new(nums1: Vec<i32>, nums2: Vec<i32>) -> Self {
        let mut freq2 = HashMap::new();
        for &num in &nums2 {
            *freq2.entry(num).or_insert(0) += 1;
        }
        Self { nums1, nums2, freq2 }
    }

    fn add(&mut self, index: i32, val: i32) {
        let old_val = self.nums2[index as usize];
        let new_val = old_val + val;

        // Update frequency map
        *self.freq2.entry(old_val).or_insert(1) -= 1;
        if self.freq2[&old_val] == 0 {
            self.freq2.remove(&old_val);
        }
        *self.freq2.entry(new_val).or_insert(0) += 1;

        // Update nums2
        self.nums2[index as usize] = new_val;
    }

    fn count(&self, tot: i32) -> i32 {
        let mut count = 0;
        for &num1 in &self.nums1 {
            let target = tot - num1;
            if let Some(&freq) = self.freq2.get(&target) {
                count += freq;
            }
        }
        count
    }
}

struct Solution;

fn main() {
    let nums1 = vec![1, 1, 2, 2, 2, 3];
    let nums2 = vec![1, 4, 5, 2, 5, 4];
    let index = 0;
    let val = 1;
    let tot = 7;
    let mut obj = FindSumPairs::new(nums1, nums2);
    obj.add(index, val);
    let ret_2: i32 = obj.count(tot);
    println!("{}", ret_2);
}
