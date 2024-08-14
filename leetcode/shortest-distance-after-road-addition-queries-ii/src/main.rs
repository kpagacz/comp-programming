// https://leetcode.com/problems/shortest-distance-after-road-addition-queries-ii/description/
pub struct Solution;

impl Solution {
    pub fn shortest_distance_after_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let mut is_part_of_road = vec![true; n];
        let mut next_city = (1..=n).collect::<Vec<_>>();

        let mut answer = Vec::with_capacity(queries.len());
        let mut cities_in_road = n - 1;
        for query in queries {
            let (from, to) = (query[0] as usize, query[1] as usize);

            if !is_part_of_road[from] || !is_part_of_road[to] {
                answer.push(cities_in_road as i32);
                continue;
            }

            let mut next_stop = next_city[from];

            while next_stop != to {
                is_part_of_road[next_stop] = false;
                cities_in_road -= 1;
                next_stop = next_city[next_stop];
            }
            next_city[from] = to;

            answer.push(cities_in_road as i32);
        }

        answer
    }
}

fn main() {
    println!("Hello, world!");
}
