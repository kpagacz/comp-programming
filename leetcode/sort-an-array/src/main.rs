// https://leetcode.com/problems/sort-an-array/description/
pub struct Solution;

impl Solution {
    pub fn sort_array(mut nums: Vec<i32>) -> Vec<i32> {
        Self::quick_sort(&mut nums[..]);
        nums
    }

    fn quick_sort(arr: &mut [i32]) {
        if arr.is_empty() || arr.len() == 1 {
            return;
        }
        let pivot = arr[arr.len() / 2];
        let [mut lt, mut eq, mut gt] = [0, 0, arr.len() - 1];

        while eq <= gt {
            match pivot.cmp(&arr[eq]) {
                std::cmp::Ordering::Less => {
                    arr.swap(gt, eq);
                    gt -= 1;
                }
                std::cmp::Ordering::Equal => eq += 1,
                std::cmp::Ordering::Greater => {
                    arr.swap(eq, lt);
                    lt += 1;
                    eq += 1;
                }
            }
        }

        Self::quick_sort(&mut arr[..lt]);
        Self::quick_sort(&mut arr[gt + 1..]);
    }
}

fn main() {
    let test_cases = [vec![5, 2, 3, 1], vec![5, 1, 1, 2, 0, 0]];
    for nums in test_cases {
        println!("{:?}", Solution::sort_array(nums));
    }
}
