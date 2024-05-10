// https://leetcode.com/problems/student-attendance-record-i/description/
pub struct Solution;

impl Solution {
    pub fn check_record(s: String) -> bool {
        let (absences, max_late, _) = s.chars().fold(
            (0, 0, 0),
            |(absences, max_late, current_late), day| match day {
                'A' => (absences + 1, max_late, 0),
                'L' => (absences, max_late.max(current_late + 1), current_late + 1),
                _ => (absences, max_late, 0),
            },
        );
        absences < 2 && max_late < 3
    }
}

fn main() {
    println!("Hello, world!");
}
