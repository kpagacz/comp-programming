// https://leetcode.com/problems/image-smoother/
pub struct Solution {}

const SMOOTHER: [(i32, i32); 9] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (0, 0),
    (1, -1),
    (1, 0),
    (1, 1),
];
impl Solution {
    pub fn image_smoother(img: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut answer = vec![vec![0; img.len()]; img[0].len()];
        for i in 0..img.len() as i32 {
            for j in 0..img[0].len() as i32 {
                let values = SMOOTHER
                    .iter()
                    .map(|(delta_x, delta_y)| (i + delta_x, j + delta_y))
                    .filter(|&(x, y)| {
                        x >= 0 && y >= 0 && x < img.len() as i32 && y < img[0].len() as i32
                    })
                    .map(|(x, y)| img[x as usize][y as usize])
                    .collect::<Vec<_>>();
                answer[i as usize][j as usize] = values.iter().sum::<i32>() / values.len() as i32;
            }
        }
        answer
    }
}

fn main() {
    println!("Hello, world!");
}
