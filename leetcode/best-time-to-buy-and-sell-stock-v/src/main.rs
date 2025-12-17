// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-v/description/?envType=daily-question&envId=2025-12-17
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn maximum_profit(prices: Vec<i32>, k: i32) -> i64 {
        let k = k as usize;
        let n = prices.len();

        // dfs(i, j, transaction) - max profit at after ith day, j transactions started,
        // transaction type
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        enum Transaction {
            None,
            Long,
            Short,
        }
        use std::collections::HashMap;
        fn dfs(
            day: usize,
            txs: usize,
            tx_type: Transaction,
            prices: &[i32],
            mem: &mut HashMap<(usize, usize, Transaction), i64>,
        ) -> i64 {
            if txs == 0 {
                return 0;
            }
            if day == 0 {
                return match tx_type {
                    Transaction::None => 0,
                    Transaction::Long => -prices[0] as i64,
                    Transaction::Short => prices[0] as i64,
                };
            }
            if let Some(cached) = mem.get(&(day, txs, tx_type)) {
                return *cached;
            }

            let res = match tx_type {
                Transaction::None => dfs(day - 1, txs, Transaction::None, prices, mem)
                    .max(dfs(day - 1, txs, Transaction::Long, prices, mem) + prices[day] as i64)
                    .max(dfs(day - 1, txs, Transaction::Short, prices, mem) - prices[day] as i64),
                Transaction::Long => dfs(day - 1, txs, Transaction::Long, prices, mem).max(
                    dfs(day - 1, txs - 1, Transaction::None, prices, mem) - prices[day] as i64,
                ),
                Transaction::Short => dfs(day - 1, txs, Transaction::Short, prices, mem).max(
                    dfs(day - 1, txs - 1, Transaction::None, prices, mem) + prices[day] as i64,
                ),
            };
            mem.insert((day, txs, tx_type), res);
            res
        }

        let mut mem = HashMap::new();
        dfs(n - 1, k, Transaction::None, &prices, &mut mem)
    }
}

fn main() {
    println!("Hello, world!");
}
