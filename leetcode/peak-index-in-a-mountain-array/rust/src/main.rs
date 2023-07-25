// https://leetcode.com/problems/peak-index-in-a-mountain-array/
pub struct Solution {}
impl Solution {
    pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
        let peak = Solution::binary_search(&arr);
        peak as i32
    }

    fn binary_search(arr: &Vec<i32>) -> usize {
        let mut low = 0_usize;
        let mut high = arr.len() - 1;

        while low < high {
            let middle = (low + high) / 2;
            if middle == 0 {
                if arr[middle] > arr[middle + 1] {
                    return middle;
                } else {
                    low = middle + 1;
                    continue;
                }
            }
            if middle == arr.len() - 1 {
                if arr[middle] > arr[middle - 1] {
                    return middle;
                } else {
                    high = middle;
                    continue;
                }
            }

            if arr[middle - 1] < arr[middle] && arr[middle] > arr[middle + 1] {
                return middle;
            } else {
                if arr[middle - 1] > arr[middle] {
                    high = middle;
                } else {
                    low = middle + 1;
                }
            }
        }

        low
    }
}
fn main() {
    println!("Hello, world!");
}
