// https://leetcode.com/problems/find-in-mountain-array/
pub struct Solution {}

/**
 * // This is the MountainArray's API interface.
 * // You should not implement it, or speculate about its implementation
 *  struct MountainArray;
 *  impl MountainArray {
 *     fn get(index:i32)->i32;
 *     fn length()->i32;
 * };
 */
pub struct MountainArray {
    arr: Vec<i32>,
}
impl MountainArray {
    fn new(arr: Vec<i32>) -> Self {
        Self { arr }
    }
    fn get(&self, index: i32) -> i32 {
        self.arr[index as usize]
    }
    fn length(&self) -> i32 {
        self.arr.len() as i32
    }
}

impl Solution {
    pub fn find_in_mountain_array(target: i32, mountainArr: &MountainArray) -> i32 {
        let size = mountainArr.length() as usize;

        let (mut left, mut right) = (0, size - 2);
        while left < right {
            let middle = left + (right - left) / 2;
            let (first, second) = (
                mountainArr.get(middle as i32),
                mountainArr.get((middle + 1) as i32),
            );
            if first < second {
                left = middle + 1;
            } else {
                right = middle;
            }
        }
        let mountain_top = left;
        println!("{mountain_top}");

        let before_mountain_top =
            Self::partition_point(mountainArr, 0, mountain_top, |el| el < target);
        let after_mountain_top =
            Self::partition_point(mountainArr, mountain_top, size - 1, |el| el > target);
        if mountainArr.get(before_mountain_top as i32) == target {
            return before_mountain_top as i32;
        }
        if after_mountain_top < size && mountainArr.get(after_mountain_top as i32) == target {
            return after_mountain_top as i32;
        }
        -1
    }

    fn partition_point<F>(
        mountain_array: &MountainArray,
        mut start: usize,
        mut end: usize,
        predicate: F,
    ) -> usize
    where
        F: Fn(i32) -> bool,
    {
        while start < end {
            let middle = start + (end - start) / 2;
            let val = mountain_array.get(middle as i32);
            if predicate(val) {
                start = middle + 1;
            } else {
                end = middle;
            }
        }
        start
    }
}

fn main() {
    let mountain_array = MountainArray::new(vec![0,1,2,4,2,1]);
    println!("{}", Solution::find_in_mountain_array(3, &mountain_array));
}
