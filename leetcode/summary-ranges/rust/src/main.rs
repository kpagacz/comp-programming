pub struct Solution {}

impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut ranges = vec![];

        if nums.len() == 0 {
            return ranges;
        }

        let mut last_num = nums[0];
        let mut range_start = last_num;

        nums.iter().skip(1).for_each(|&elem| {
            if elem != last_num + 1 {
                if last_num == range_start {
                    ranges.push(last_num.to_string());
                } else {
                    ranges.push(format!("{}->{}", range_start, last_num));
                }
                range_start = elem;
            }
            last_num = elem;
        });

        if last_num == range_start {
            ranges.push(last_num.to_string());
        } else {
            ranges.push(format!("{}->{}", range_start, last_num));
        }

        ranges
    }
}

fn main() {
    println!("Hello, world!");
}
