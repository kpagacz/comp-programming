// https://leetcode.com/problems/minimum-amount-of-time-to-collect-garbage/description/
pub struct Solution {}

impl Solution {
    pub fn garbage_collection(garbage: Vec<String>, travel: Vec<i32>) -> i32 {
        let mut letters = vec![0; 3]; // M, P, G
        let (mut last_m, mut last_p, mut last_g) = (0, 0, 0);

        for (id, g) in garbage.iter().enumerate() {
            for letter in g.as_bytes() {
                match letter {
                    b'M' => {
                        letters[0] += 1;
                        last_m = id;
                    }
                    b'P' => {
                        letters[1] += 1;
                        last_p = id;
                    }
                    b'G' => {
                        letters[2] += 1;
                        last_g = id;
                    }
                    _ => unreachable!(),
                }
            }
        }
        let mut answer = letters.into_iter().sum();

        let travel: Vec<_> = travel.into_iter().scan(0, |prefix, time| {
            *prefix += time;
            Some(*prefix)
        }).collect();

        for last_id in [last_m, last_p, last_g] {
            if last_id != 0 {
                answer += travel[last_id - 1];
            }
        }
        answer
    }
}

fn main() {
    println!("Hello, world!");
}
