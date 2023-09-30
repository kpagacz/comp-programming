// https://leetcode.com/problems/132-pattern/description/
pub struct Solution {}

// What did I learn?
// I couldn't solve this on my own in a reasonable time, and I think
// it comes from my poor understanding of what exactly the monotonic stack does.
// What I have learned here is that monotonic stack is REALLY good at keeping
// track of the previous (in order) highest value. So for each element, we can
// easily tell what is the second highest value so far.
//
// This property combined with the fact that we can solve the problem in reverse
// (so looking for a 231 pattern instead of 132) makes it really easy to keep track
// of the 23 values for each num in the reversed array. And also ensures that we
// tracked the highest 23 values so far as well.
//
// I need to think more about what are the values discarded when traversing an array
// with the monotonic stack. I also need to realize that it is quite easy to find
// any values larger than the two lowest it finds so far or any values lower
// than two highest we find so far.
//
// This problem found me lacking :( I am pretty bummed about it because I instinctively
// thought about monotonic stack when approaching this problem but I couldn't find
// the reverse solution, and I was struggling to make it work when iterating forward.
// But iterating forward meant any value needed to be in range, and that's something
// a monotonic stack cannot do. This is what I learned today.
impl Solution {
    pub fn find132pattern(mut nums: Vec<i32>) -> bool {
        nums.reverse();
        let mut monotonic_stack = Vec::new();
        let mut previous_highest = i32::MIN;

        for num in nums {
            if num.cmp(&previous_highest) == std::cmp::Ordering::Less {
                return true;
            }

            while !monotonic_stack.is_empty() && &num > monotonic_stack.last().unwrap() {
                previous_highest = monotonic_stack.pop().unwrap();
            }

            monotonic_stack.push(num);
        }

        false
    }

    pub fn find132pattern_another(mut nums: Vec<i32>) -> bool {
        let mut monotonic_stack = Vec::new();
        let mut min_prefix = vec![i32::MAX; nums.len()];
        min_prefix[0] = nums[0];

        for i in 1..nums.len() {
            min_prefix[i] = min_prefix[i - 1].min(nums[i]);
        }

        for i in (0..nums.len()).rev() {
            if min_prefix[i] < nums[i] {
                while !monotonic_stack.is_empty()
                    && monotonic_stack.last().unwrap() <= &min_prefix[i]
                {
                    monotonic_stack.pop();
                }
                if !monotonic_stack.is_empty() && monotonic_stack.last().unwrap() < &nums[i] {
                    return true;
                }

                monotonic_stack.push(nums[i]);
            }
        }

        false
    }
}

fn main() {
    let test_cases = vec![vec![3, 1, 4, 2]];
    for test in test_cases {
        println!("{}", Solution::find132pattern(test));
    }
}
