// https://leetcode.com/problems/destination-city/
pub struct Solution {}

impl Solution {
    pub fn dest_city(paths: Vec<Vec<String>>) -> String {
        use std::collections::BTreeSet;
        let mut destination_cities = BTreeSet::new();
        paths.iter().for_each(|path| {
            destination_cities.insert(path[1].clone());
        });
        paths.iter().for_each(|path| {
            destination_cities.remove(&path[0]);
        });
        destination_cities.into_iter().next().unwrap()
    }
}

fn main() {
    println!("Hello, world!");
}
