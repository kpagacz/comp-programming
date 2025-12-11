// https://leetcode.com/problems/count-covered-buildings/description/?envType=daily-question&envId=2025-12-11
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_covered_buildings(n: i32, buildings: Vec<Vec<i32>>) -> i32 {
        let mut rows = vec![(usize::MAX, usize::MIN); n as usize + 1]; // holds min and max column
        let mut cols = vec![(usize::MAX, usize::MIN); n as usize + 1]; // holds min and max row

        for building in &buildings {
            let (row, col) = (building[0] as usize, building[1] as usize);
            rows[row].0 = rows[row].0.min(col);
            rows[row].1 = rows[row].1.max(col);
            cols[col].0 = cols[col].0.min(row);
            cols[col].1 = cols[col].1.max(row);
        }

        let mut covered = 0;
        for building in &buildings {
            let (row, col) = (building[0] as usize, building[1] as usize);
            if rows[row].0 < col && col < rows[row].1 && cols[col].0 < row && row < cols[col].1 {
                covered += 1;
            }
        }
        covered
    }
}

fn main() {
    let test_cases = vec![
        (
            3,
            vec![vec![1, 2], vec![2, 2], vec![3, 2], vec![2, 1], vec![2, 3]],
            1,
        ),
        (3, vec![vec![1, 1], vec![1, 2], vec![2, 1], vec![2, 2]], 0),
        (
            5,
            vec![vec![1, 3], vec![3, 2], vec![3, 3], vec![3, 5], vec![5, 3]],
            1,
        ),
    ];
    for (n, buildings, exp) in test_cases {
        println!(
            "{} exp: {exp}",
            Solution::count_covered_buildings(n, buildings)
        );
    }
}
