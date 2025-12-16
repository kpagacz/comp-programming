// https://leetcode.com/problems/maximum-profit-from-trading-stocks-with-discounts/description/?envType=daily-question&envId=2025-12-16
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn max_profit(
        n: i32,
        present: Vec<i32>,
        future: Vec<i32>,
        hierarchy: Vec<Vec<i32>>,
        budget: i32,
    ) -> i32 {
        let n = n as usize;
        let mut bosses = vec![0; n + 1];
        let mut subordinates = vec![vec![]; n + 1];
        for relationship in hierarchy {
            let (boss, subordinate) = (relationship[0] as usize - 1, relationship[1] as usize - 1);
            bosses[subordinate] = boss;
            subordinates[boss].push(subordinate);
        }

        // returns max profit
        fn rec(
            node: usize,
            present: &[i32],
            future: &[i32],
            subordinates: &[Vec<usize>],
            remaining_budget: usize,
        ) -> (Vec<i32>, Vec<i32>, usize) {
            let cost = present[node] as usize;
            let discounted_cost = cost / 2;
            let mut dp_parent_bought = vec![0; remaining_budget + 1];
            let mut dp_parent_not_bought = vec![0; remaining_budget + 1];
            let mut children_parent_bought = vec![0; remaining_budget + 1];
            let mut children_parent_not_bought = vec![0; remaining_budget + 1];
            let mut subtree_weight = cost;

            for &child in &subordinates[node] {
                let (child_dp_parent_bought, child_dp_parent_not_bought, child_subtree_weight) =
                    rec(child, present, future, subordinates, remaining_budget);
                subtree_weight += child_subtree_weight;

                for remaining_budget in (0..=remaining_budget).rev() {
                    for available_funds in 0..=child_subtree_weight.min(remaining_budget) {
                        if remaining_budget >= available_funds {
                            children_parent_bought[remaining_budget] =
                                children_parent_bought[remaining_budget].max(
                                    children_parent_bought[remaining_budget - available_funds]
                                        + child_dp_parent_bought[available_funds],
                                );
                            children_parent_not_bought[remaining_budget] =
                                children_parent_not_bought[remaining_budget].max(
                                    children_parent_not_bought[remaining_budget - available_funds]
                                        + child_dp_parent_not_bought[available_funds],
                                );
                        }
                    }
                }
            }

            for i in 0..=remaining_budget {
                dp_parent_bought[i] = children_parent_not_bought[i];
                dp_parent_not_bought[i] = children_parent_not_bought[i];

                if i >= discounted_cost {
                    dp_parent_bought[i] = dp_parent_bought[i].max(
                        children_parent_bought[i - discounted_cost] + future[node]
                            - discounted_cost as i32,
                    );
                }
                if i >= cost {
                    dp_parent_not_bought[i] = dp_parent_not_bought[i]
                        .max(children_parent_bought[i - cost] + future[node] - cost as i32);
                }
            }

            (dp_parent_bought, dp_parent_not_bought, subtree_weight)
        }

        let (_, dp, _) = rec(0, &present, &future, &subordinates, budget as usize);
        dp[budget as usize]
    }
}

fn main() {
    let test_cases = [
        (2, vec![1, 2], vec![4, 3], vec![vec![1, 2]], 3, 5),
        (2, vec![3, 4], vec![5, 8], vec![vec![1, 2]], 4, 4),
    ];
    for (n, present, future, hierarchy, budget, exp) in test_cases {
        println!(
            "{} exp: {exp}",
            Solution::max_profit(n, present, future, hierarchy, budget)
        );
    }
}
