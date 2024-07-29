// https://leetcode.com/problems/count-number-of-teams/description/
pub struct Solution;

struct Fenwick {
    tree: Vec<i32>,
}

impl Fenwick {
    fn new(n: usize) -> Self {
        Self {
            tree: vec![0; n + 1],
        }
    }

    fn query(&self, mut index: i32) -> i32 {
        let mut sum = 0;

        while index > 0 {
            sum += self.tree[index as usize - 1];
            index &= index - 1;
        }
        sum
    }

    fn update(&mut self, mut index: i32, value: i32) {
        while (index as usize) < self.tree.len() {
            self.tree[index as usize] += value;
            index |= index + 1;
        }
    }
}

impl Solution {
    pub fn num_teams(rating: Vec<i32>) -> i32 {
        const MAX: usize = 100_000 + 1;
        let mut forward_tree = Fenwick::new(MAX);
        let mut backward_tree = Fenwick::new(MAX);

        rating
            .iter()
            .for_each(|&rating| backward_tree.update(rating, 1));

        let mut answer = 0;

        for i in 0..rating.len() {
            backward_tree.update(rating[i], -1);

            let lower_before = forward_tree.query(rating[i]);
            let greater_after =
                backward_tree.query(MAX as i32) - backward_tree.query(rating[i] + 1);
            answer += lower_before * greater_after;

            let greater_before = forward_tree.query(MAX as i32) - forward_tree.query(rating[i] + 1);
            let lower_after = backward_tree.query(rating[i]);
            answer += greater_before * lower_after;

            forward_tree.update(rating[i], 1);
        }

        answer
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::Fenwick;

    #[test]
    fn test_fewick() {
        let mut ftree = Fenwick::new(10);

        ftree.update(6, 1);
        assert_eq!(ftree.query(7), 1);
        ftree.update(1, 1);
        assert_eq!(ftree.query(1), 0);
        assert_eq!(ftree.query(2), 1);
        assert_eq!(ftree.query(7), 2);
    }
}
