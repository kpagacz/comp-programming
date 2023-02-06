impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let mut answer = Vec::new();
        answer.reserve(nums.count);

        for i in 0..n as usize {
            answer.push(nums[i]);
            answer.push(nums[n + i]);
        }

        answer
    }
}
