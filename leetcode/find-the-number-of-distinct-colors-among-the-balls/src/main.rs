// https://leetcode.com/problems/find-the-number-of-distinct-colors-among-the-balls/description/
pub struct Solution;

impl Solution {
    pub fn query_results(_limit: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        use std::collections::HashMap;
        let mut balls = HashMap::new();
        let mut colours: HashMap<i32, Vec<usize>> = HashMap::new();

        let mut ans = Vec::with_capacity(queries.len());
        for query in queries {
            let (ball, colour) = (query[0] as usize, query[1]);
            match balls.get(&ball) {
                None => {
                    balls.insert(ball, colour);
                    colours.entry(colour).or_default().push(ball);
                }
                Some(&old_colour) => {
                    balls.insert(ball, colour);
                    colours
                        .entry(old_colour)
                        .or_default()
                        .retain(|coloured_ball| *coloured_ball != ball);
                    if colours.entry(old_colour).or_default().is_empty() {
                        colours.remove(&old_colour);
                    }
                    colours.entry(colour).or_default().push(ball);
                }
            }
            ans.push(colours.keys().len() as _);
        }
        ans
    }
}

fn main() {
    println!("Hello, world!");
}
