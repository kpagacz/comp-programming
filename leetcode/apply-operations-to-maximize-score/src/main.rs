// https://leetcode.com/problems/apply-operations-to-maximize-score/description/?envType=daily-question&envId=2025-03-29
pub struct Solution;

impl Solution {
    pub fn maximum_score(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let max_num = 1 + *nums.iter().max().unwrap() as usize;
        let mut prime_score = vec![0; max_num];

        for num in 2..prime_score.len() {
            if prime_score[num] == 0 {
                for v in (num..prime_score.len()).step_by(num) {
                    prime_score[v] += 1;
                }
            }
        }

        // Count the number of values on the left side of the given index;
        // that could be included in the range - monotonic stack
        let mut left_count = vec![0_usize; n];
        let mut stack = vec![];
        for id in 0..n {
            while !stack.is_empty()
                && prime_score[nums[*stack.last().unwrap()] as usize]
                    < prime_score[nums[id] as usize]
            {
                stack.pop();
            }

            if stack.is_empty() {
                left_count[id] += id;
            } else {
                left_count[id] += id - *stack.last().unwrap() - 1;
            }
            stack.push(id);
        }

        // Count the number of values on the right side of the given index;
        // that could be included in the range - monotonic stack
        let mut right_count = vec![0_usize; n];
        stack.clear();

        for id in (0..n).rev() {
            while !stack.is_empty()
                && prime_score[nums[*stack.last().unwrap()] as usize]
            // it's <= because the prime score of the range is
            // the maximum score of any number in the range
            // or the number with the lowest index in case of ties.
            // This "ties" rule is what is different between this case and
            // the above one.
                    <= prime_score[nums[id] as usize]
            {
                stack.pop();
            }
            if stack.is_empty() {
                right_count[id] += n - id - 1;
            } else {
                right_count[id] += *stack.last().unwrap() - id - 1;
            }
            stack.push(id);
        }

        // Sort numbers in descending order
        let mut sorted_num_ids: Vec<_> = (0..n).collect();
        sorted_num_ids.sort_unstable_by_key(|id| -nums[*id]);

        let mut answer = 1;
        let mut k = k as usize;
        const MOD: usize = 10_usize.pow(9) + 7;
        // println!("left count: {left_count:?} right_count {right_count:?} primes: {prime_score:?}");
        // println!("sorted: {sorted_num_ids:?}");

        for num_id in sorted_num_ids {
            if k == 0 {
                break;
            }

            let mut x = nums[num_id] as usize;
            let mut power = k.min((left_count[num_id] + 1) * (right_count[num_id] + 1));

            k -= power;

            while power > 0 {
                if power % 2 == 1 {
                    answer *= x;
                    answer %= MOD;
                }

                x *= x;
                x %= MOD;

                power /= 2;
            }
        }

        answer as _
    }

    pub fn maximum_score_another(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let m = 1 + nums.iter().max().unwrap().clone() as usize;

        // 1. Find primes scores of all numbers up to the max num in the nums
        //    (using the Sieve of Eratosthenes algorithm)

        let mut prime_scores = vec![0; m];

        for x in 2..m {
            if prime_scores[x] == 0 {
                for v in (x..m).step_by(x) {
                    prime_scores[v] += 1
                }
            }
        }

        // 2. Count the number of values on the left side of each given num
        //    that could be included in a subarray with this num (using Monotonic Stack)

        let mut left_count = vec![0_usize; n];
        let mut left_border = Vec::<usize>::new();

        for l in 0..n {
            while !left_border.is_empty() {
                if prime_scores[nums[left_border.last().unwrap().clone()] as usize]
                    >= prime_scores[nums[l] as usize]
                {
                    break;
                }

                left_border.pop();
            }

            if left_border.is_empty() {
                left_count[l] += l
            } else {
                left_count[l] += l - left_border.last().unwrap().clone() - 1
            }

            left_border.push(l);
        }

        // 3. Count the number of values on the right side of each given num
        //    that could be included in a subarray with this num (using Monotonic Stack)

        let mut right_count = vec![0_usize; n];
        let mut right_border = Vec::<usize>::new();

        for r in (0..n).rev() {
            while !right_border.is_empty() {
                if prime_scores[nums[r] as usize]
                    < prime_scores[nums[right_border.last().unwrap().clone()] as usize]
                {
                    break;
                }

                right_border.pop();
            }

            if right_border.is_empty() {
                right_count[r] += n - r - 1
            } else {
                right_count[r] += right_border.last().unwrap().clone() - r - 1
            }

            right_border.push(r);
        }

        // 4. Sort nums in descending order (the indexes of nums)
        //    and calculate the result

        let mut ii = (0..n).collect::<Vec<usize>>();

        ii.sort_unstable_by_key(|i| std::cmp::Reverse(nums[*i]));

        let mut remain = k as usize;
        let mut result = 1usize;

        const MOD: usize = (1e9 as usize) + 7;
        println!("left count: {left_count:?} right_count {right_count:?} primes: {prime_scores:?}");
        println!("sorted: {ii:?}");

        for i in ii {
            if remain == 0 {
                break;
            }

            let mut x = nums[i] as usize;
            let mut c = remain.min((left_count[i] + 1) * (right_count[i] + 1));

            remain -= c;

            // Binary Exponentiation
            while c > 0 {
                if c % 2 == 1 {
                    result *= x;
                    result %= MOD;
                }

                x *= x;
                x %= MOD;

                c /= 2;
            }
        }

        result as i32
    }
}

fn main() {
    let test_cases = [(vec![8, 3, 9, 3, 8], 2)];
    for (nums, k) in test_cases {
        println!("{}", Solution::maximum_score(nums.clone(), k));
        println!("{}", Solution::maximum_score_another(nums, k));
    }
}
