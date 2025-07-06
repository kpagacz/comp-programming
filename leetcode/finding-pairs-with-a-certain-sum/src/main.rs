// https://leetcode.com/problems/finding-pairs-with-a-certain-sum/description/?envType=daily-question&envId=2025-07-06

use std::collections::BTreeMap;
type Array = Vec<i32>;
type Frequency = BTreeMap<i32, usize>;
pub struct FindSumPairs {
    nums1: Array,
    freq1: Frequency,
    nums2: Array,
    freq2: Frequency,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FindSumPairs {
    fn new(nums1: Vec<i32>, nums2: Vec<i32>) -> Self {
        let freq1 = nums1.iter().fold(Frequency::new(), |mut freq, num| {
            freq.entry(*num).and_modify(|f| *f += 1).or_insert(1);
            freq
        });
        let freq2 = nums2.iter().fold(Frequency::new(), |mut freq, num| {
            freq.entry(*num).and_modify(|f| *f += 1).or_insert(1);
            freq
        });
        Self {
            nums1,
            freq1,
            nums2,
            freq2,
        }
    }

    fn add(&mut self, index: i32, val: i32) {
        let index = index as usize;
        self.freq2.entry(self.nums2[index]).and_modify(|f| *f -= 1);
        self.nums2[index] += val;
        self.freq2
            .entry(self.nums2[index])
            .and_modify(|f| *f += 1)
            .or_insert(1);
    }

    fn count(&self, tot: i32) -> i32 {
        let mut answer = 0;
        for (key, f) in &self.freq1 {
            let target = tot - key;
            let mut found = self.freq2.range(target..);
            if let Some((other_key, other_f)) = found.next() {
                if other_key + key == tot && *other_f > 0 && *f > 0 {
                    answer += other_f * f;
                }
            }
        }
        answer as _
    }
}

/**
 * Your FindSumPairs object will be instantiated and called as such:
 * let obj = FindSumPairs::new(nums1, nums2);
 * obj.add(index, val);
 * let ret_2: i32 = obj.count(tot);
 */

fn main() {
    println!("Hello, world!");
}
