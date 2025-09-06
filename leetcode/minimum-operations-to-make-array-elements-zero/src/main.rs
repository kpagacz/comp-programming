// https://leetcode.com/problems/minimum-operations-to-make-array-elements-zero/description/?envType=daily-question&envId=2025-09-06

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_operations(queries: Vec<Vec<i32>>) -> i64 {
        // The tricky bit of this problem is calculating how many
        // numbers have x bits. Because the observation is that dividing
        // by four is just shifting the number by two bits to the right.
        // And the question is how many times do we have to shift
        // to make the number 0.
        // For 1 and 2 bit number -> 1 (numbers 1..4)
        // For 3 and 4 bit numbers -> 2 (numbers 4..16)
        // For 5 and 6 bits numbers -> 3 (numbers 16..64)

        let mut answer = 0;
        for query in queries {
            let (left, right) = (query[0], query[1]);

            // We pair the operations and if the number
            // of total operations is odd we need another one.
            // Hence the +1
            answer += (-Solution::number_of_operations(left - 1)
                + Solution::number_of_operations(right)
                + 1)
                / 2;
        }
        answer
    }

    fn number_of_operations(to: i32) -> i64 {
        let to = to as i64;
        let mut base = 1i64;
        let mut ops_required = 1i64;
        let mut ops_count = 0i64;
        // This <= is important, otherwise
        // we stop before to is counted as well
        while base <= to {
            let next_base = base << 2; // * 4
            let number_of_nums = (next_base - 1).min(to) - base + 1;
            ops_count += number_of_nums * ops_required;
            ops_required += 1;
            base = next_base;
        }

        ops_count
    }
}

fn main() {
    let test_cases = [(vec![vec![4, 16]], 14)];

    for (queries, exp) in test_cases {
        println!("{}   exp: {exp}", Solution::min_operations(queries));
    }
}
