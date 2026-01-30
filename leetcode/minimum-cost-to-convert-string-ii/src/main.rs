// https://leetcode.com/problems/minimum-cost-to-convert-string-ii/description/?envType=daily-question&envId=2026-01-30
struct Solution;

#[derive(Clone)]
struct TrieNode {
    end: bool,
    children: Vec<Option<TrieNode>>,
}

impl TrieNode {
    fn new() -> Self {
        Self {
            end: false,
            children: vec![Option::None; 26],
        }
    }

    fn append(&mut self, word: &str) {
        let mut it = self;
        for &c in word.as_bytes() {
            let offset = (c - b'a') as usize;
            if it.children[offset].is_none() {
                let new_node = TrieNode::new();
                it.children[offset] = Option::Some(new_node);
            }
            it = it.children[offset].as_mut().unwrap();
        }
        it.end = true;
    }

    fn next(&self, c: u8) -> Option<&TrieNode> {
        let offset = (c - b'a') as usize;
        self.children[offset].as_ref()
    }

    fn is_word(&self) -> bool {
        self.end
    }
}

#[allow(dead_code)]
impl Solution {
    pub fn minimum_cost(
        source: String,
        target: String,
        original: Vec<String>,
        changed: Vec<String>,
        cost: Vec<i32>,
    ) -> i64 {
        // Floyd Warshal original -> changed
        let mut triples = (0..original.len())
            .map(|id| (cost[id], &original[id], &changed[id]))
            .collect::<Vec<_>>();
        triples.sort_unstable();
        triples.dedup_by_key(|(_, o, c)| (o.clone(), c.clone()));

        let mut all = original
            .iter()
            .cloned()
            .chain(changed.iter().cloned())
            .collect::<Vec<_>>();
        all.sort_unstable();
        all.dedup();
        let mut fw = vec![vec![i64::MAX; all.len()]; all.len()];
        for i in 0..fw.len() {
            for j in 0..fw.len() {
                for k in 0..triples.len() {
                    if &all[i] == triples[k].1 && &all[j] == triples[k].2 {
                        fw[i][j] = fw[i][j].min(triples[k].0 as i64);
                    }
                }
            }
        }
        for k in 0..fw.len() {
            for i in 0..fw.len() {
                for j in 0..fw.len() {
                    if fw[i][k] != i64::MAX && fw[k][j] != i64::MAX {
                        fw[i][j] = fw[i][j].min(fw[i][k] + fw[k][j]);
                    }
                }
            }
        }
        use std::collections::HashMap;
        let mapping: HashMap<&str, usize> =
            all.iter()
                .enumerate()
                .fold(HashMap::new(), |mut map, (pos, s)| {
                    map.insert(s, pos);
                    map
                });

        let mut trie = TrieNode::new();
        for word in &all {
            trie.append(word);
        }

        // DP from the end, where dp[i] is the cost to
        // change source[i..] to target[i..]
        // dp[i] = -1 means it's impossible
        let mut dp = vec![i64::MAX; source.len() + 1];
        dp[source.len()] = 0; // It takes zero cost to change nothing

        // Iterate from the end
        // For each index iterate towards the end checking if a
        // substring source[i..j] can be transformed to target[i..j]
        // and dp[j] != -1, if so, update the cost
        for i in (0..source.len()).rev() {
            let mut trie_it = Option::Some(&trie);
            if source.as_bytes()[i] == target.as_bytes()[i] {
                dp[i] = dp[i + 1];
            }
            for j in i + 1..=source.len() {
                let next_char = source.as_bytes()[j - 1];
                trie_it = trie_it.and_then(|trie_it| trie_it.next(next_char));
                if trie_it.is_none() {
                    break;
                }
                if trie_it.is_some_and(|trie_it| trie_it.is_word())
                    && let Some(original_id) = mapping.get(&source[i..j])
                    && let Some(changed_id) = mapping.get(&target[i..j])
                    && dp[j] != i64::MAX
                    && fw[*original_id][*changed_id] != i64::MAX
                {
                    let cost = fw[*original_id][*changed_id];
                    dp[i] = dp[i].min(dp[j] + cost);
                }
            }
        }

        // Returns dp[0] = cost of changing source[0..] - the entire string
        if dp[0] == i64::MAX { -1 } else { dp[0] }
    }
}

fn main() {
    let test_cases = [
        (
            "abcd",
            "acbe",
            vec!["a", "b", "c", "c", "e", "d"],
            vec!["b", "c", "b", "e", "b", "e"],
            vec![2, 5, 5, 1, 2, 20],
            28,
        ),
        (
            "abcdefgh",
            "acdeeghh",
            vec!["bcd", "fgh", "thh"],
            vec!["cde", "thh", "ghh"],
            vec![1, 3, 5],
            9,
        ),
        (
            "abcdefgh",
            "addddddd",
            vec!["bcd", "defgh"],
            vec!["ddd", "ddddd"],
            vec![100, 1578],
            -1,
        ),
        (
            "aaaddcaaccbabaaccbabbaadcccadbaacbddbaccabddbdbaaddbbacbddddbbdbccaadcaccacdbcbddbacabadaaccbadbbdbc",
            "abaddcabcdbabcbadcaccaadabbadddbcacaaabdabbdcbbdbcbaaabbbcddcbddcbccadacddcbdcbacadbbadbdabcbadbbdac",
            vec![
                "ddddb",
                "dccbb",
                "dadac",
                "dbdbb",
                "ddbacabadaac",
                "bcbccdcadabd",
                "dacabcdaacca",
                "dcdadacacbbd",
                "dcccadbaacbddbacc",
                "dcdcbccdccdbaaaac",
                "bbbcccdbcdcadaabc",
                "bccaadcaccacdb",
                "bbcabcbcbaddbd",
                "dbadadaddcddad",
                "badaddbcddacca",
                "bc",
                "da",
                "cb",
                "ddbdbaaddbbac",
                "dbcadcdbabddd",
                "abdadacbbbcca",
                "adaaabcabdbcc",
                "caaccbabaaccbabba",
                "abaadddbaaccbbacc",
                "bbddaaadcbccccbac",
                "cdbdbddaadbbbdbdd",
                "bcbdaabaddbdcdcaa",
                "aa",
                "cb",
                "dd",
            ],
            vec![
                "dccbb",
                "dadac",
                "dbdbb",
                "bcddc",
                "bcbccdcadabd",
                "dacabcdaacca",
                "dcdadacacbbd",
                "acadbbadbdab",
                "dcdcbccdccdbaaaac",
                "bbbcccdbcdcadaabc",
                "dabbadddbcacaaabd",
                "bbcabcbcbaddbd",
                "dbadadaddcddad",
                "badaddbcddacca",
                "dcbccadacddcbd",
                "da",
                "cb",
                "ac",
                "dbcadcdbabddd",
                "abdadacbbbcca",
                "adaaabcabdbcc",
                "bdcbbdbcbaaab",
                "abaadddbaaccbbacc",
                "bbddaaadcbccccbac",
                "cdbdbddaadbbbdbdd",
                "bcbdaabaddbdcdcaa",
                "cabcdbabcbadcacca",
                "cb",
                "dd",
                "ba",
            ],
            vec![
                67, 56, 64, 83, 100, 73, 95, 97, 100, 98, 20, 92, 58, 70, 95, 77, 95, 93, 69, 92,
                77, 53, 96, 68, 83, 96, 93, 64, 81, 100,
            ],
            2405,
        ),
    ];
    for (source, target, original, changed, cost, exp) in test_cases {
        let source = source.to_string();
        let target = target.to_string();
        let original = original.into_iter().map(str::to_owned).collect();
        let changed = changed.into_iter().map(str::to_owned).collect();

        println!(
            "{} exp {exp}",
            Solution::minimum_cost(source, target, original, changed, cost)
        );
    }
}
