// https://leetcode.com/problems/minimum-number-of-people-to-teach/description/?envType=daily-question&envId=2025-09-10
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn minimum_teachings(n: i32, languages: Vec<Vec<i32>>, friendships: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashMap;
        use std::collections::HashSet;

        let known_languages: HashMap<usize, HashSet<i32>> =
            languages
                .into_iter()
                .enumerate()
                .fold(HashMap::new(), |mut map, (id, languages)| {
                    map.insert(id, HashSet::from_iter(languages));
                    map
                });
        let friendship_languages: Vec<bool> = friendships
            .iter()
            .map(|friendship| {
                let (first, second) = (friendship[0] as usize - 1, friendship[1] as usize - 1);
                match (known_languages.get(&first), known_languages.get(&second)) {
                    (Some(first_ls), Some(second_ls)) => {
                        first_ls.intersection(second_ls).count() > 0
                    }
                    _ => false,
                }
            })
            .collect();

        let mut needed_to_teach = vec![0; n as usize + 1];

        for language in 1..=n {
            let mut need_to_teach = [false; 501];
            for (id, relation) in friendships.iter().enumerate() {
                if !friendship_languages[id] {
                    let (first, second) = (relation[0] as usize - 1, relation[1] as usize - 1);
                    if !known_languages[&first].contains(&language) {
                        need_to_teach[first] = true;
                    }
                    if !known_languages[&second].contains(&language) {
                        need_to_teach[second] = true;
                    }
                }
            }
            needed_to_teach[language as usize] = need_to_teach
                .into_iter()
                .filter(|is_needed| *is_needed)
                .count() as _;
        }

        needed_to_teach.into_iter().skip(1).min().unwrap()
    }
}

fn main() {
    let test_cases = [(
        2,
        vec![vec![1], vec![2], vec![1, 2]],
        vec![vec![1, 2], vec![1, 3], vec![2, 3]],
        1,
    )];

    for (n, languages, friendships, exp) in test_cases {
        println!(
            "{}  exp: {exp}",
            Solution::minimum_teachings(n, languages, friendships)
        );
    }
}
