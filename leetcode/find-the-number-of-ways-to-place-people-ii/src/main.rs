// https://leetcode.com/problems/find-the-number-of-ways-to-place-people-ii/description/
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn number_of_pairs(mut points: Vec<Vec<i32>>) -> i32 {
        let mut answer = 0;
        points.sort_unstable_by(|first, second| {
            first[0].cmp(&second[0]).then(second[1].cmp(&first[1]))
        });

        println!("{:?}", &points);
        for i in 0..points.len() {
            let mut max_y = i32::MIN;

            for second in points.iter().skip(i + 1) {
                if second[1] > max_y && second[1] <= points[i][1] {
                    answer += 1;
                    max_y = second[1];
                }
            }
        }

        answer
    }
}

fn main() {
    let test_cases = [
        (vec![vec![3, 1], vec![1, 3], vec![1, 1]], 2),
        (vec![vec![0, 3], vec![5, 4], vec![6, 2]], 2),
    ];
    for (points, exp) in test_cases {
        println!("{} exp: {exp}", Solution::number_of_pairs(points));
    }
}
