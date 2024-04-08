// https://leetcode.com/problems/number-of-students-unable-to-eat-lunch/description/
pub struct Solution;

impl Solution {
    pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
        let mut eaten = 0;
        let mut new_students = vec![];

        for &student in &students {
            if student == sandwiches[eaten] {
                eaten += 1;
            } else {
                new_students.push(student);
            }
        }

        if new_students.is_empty() {
            students.len() as i32 - eaten as i32
        } else if eaten == 0 {
            students.len() as _
        } else {
            students.len() as i32
                - eaten as i32
                - (new_students.len() as i32
                    - Solution::count_students(
                        new_students,
                        sandwiches.into_iter().skip(eaten).collect(),
                    ))
        }
    }
}

fn main() {
    let test_cases = [
        (vec![1, 1, 0, 0], vec![0, 1, 0, 1]),
        (vec![1, 1, 1, 0, 0, 1], vec![1, 0, 0, 0, 1, 1]),
    ];

    for (students, sandwiches) in test_cases {
        println!("{}", Solution::count_students(students, sandwiches));
    }
}
