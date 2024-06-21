// https://leetcode.com/problems/grumpy-bookstore-owner/description/
pub struct Solution;

impl Solution {
    pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32) -> i32 {
        let minutes = minutes as usize;

        let mut unhappy_customers = customers
            .iter()
            .zip(grumpy.iter())
            .take(minutes)
            .filter_map(
                |(customers, &grumpy)| {
                    if grumpy == 1 {
                        Some(customers)
                    } else {
                        None
                    }
                },
            )
            .sum::<i32>();
        let mut max_unhappy_customers = unhappy_customers;

        for i in minutes..customers.len() {
            if grumpy[i - minutes] == 1 {
                unhappy_customers -= customers[i - minutes];
            }
            if grumpy[i] == 1 {
                unhappy_customers += customers[i];
            }

            max_unhappy_customers = max_unhappy_customers.max(unhappy_customers);
        }

        customers
            .iter()
            .zip(grumpy)
            .filter_map(
                |(customers, grumpy)| {
                    if grumpy == 1 {
                        None
                    } else {
                        Some(customers)
                    }
                },
            )
            .sum::<i32>()
            + max_unhappy_customers
    }
}

fn main() {
    let test_cases = [(
        vec![1, 0, 1, 2, 1, 1, 7, 5],
        vec![0, 1, 0, 1, 0, 1, 0, 1],
        3,
    )];

    for (customers, grumpy, minutes) in test_cases {
        println!("{}", Solution::max_satisfied(customers, grumpy, minutes));
    }
}
