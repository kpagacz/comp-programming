// https://leetcode.com/problems/k-th-smallest-prime-fraction/description/
pub struct Solution;

#[derive(PartialEq, Eq)]
struct Fraction(i32, i32);
impl Ord for Fraction {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let this = self.0 * other.1;
        let other = other.0 * self.1;

        this.cmp(&other)
    }
}
impl PartialOrd for Fraction {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    // O(n^2logn)
    // pub fn kth_smallest_prime_fraction(arr: Vec<i32>, k: i32) -> Vec<i32> {
    //     let mut fractions = vec![];
    //     let k = k as usize;
    //
    //     for i in 0..arr.len() {
    //         for j in i + 1..arr.len() {
    //             fractions.push((arr[i], arr[j]));
    //         }
    //     }
    //
    //     let sort_fractions = |first: &(i32, i32), second: &(i32, i32)| {
    //         let new_first = (first.0 * second.1, first.1 * second.1);
    //         let new_second = (second.0 * first.1, second.1 * first.1);
    //         new_first.0.cmp(&new_second.0)
    //     };
    //
    //     fractions.sort_unstable_by(sort_fractions);
    //
    //     vec![fractions[k - 1].0, fractions[k - 1].1]
    // }

    // O((n + k)logn)
    pub fn kth_smallest_prime_fraction(arr: Vec<i32>, mut k: i32) -> Vec<i32> {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        let mut pq = BinaryHeap::new();

        let last = *arr.last().unwrap();
        let n = arr.len();
        arr.iter().enumerate().for_each(|(pos, &num)| {
            pq.push(Reverse((Fraction(num, last), pos, n - 1)));
        });

        while k > 1 {
            let (_, nominator, denominator) = pq.pop().unwrap().0;

            if nominator + 1 < denominator {
                pq.push(Reverse((
                    Fraction(arr[nominator], arr[denominator - 1]),
                    nominator,
                    denominator - 1,
                )));
            }

            k -= 1;
        }

        let top = pq.pop().unwrap().0 .0;
        vec![top.0, top.1]
    }

    // What did I learn?
    // This is a clever way of using a binary search. Using binary search did
    // go through my head when thinking about solving this problem,
    // but I couldn't figure out a linear way of calculating the number
    // lower fractions.
    //
    // What do I have to take home here...?
    // The inner loop reminds me a bit of the questions regardnig the subarrays.
    //
    // I think this works because a fraction is an f(i, j) where f(i,j) > f(i, j + 1)
    // for all i and j and f(i,j) < f(i + 1, j) for all i and js.
    //
    // And also in this approach, we did not care about the order of the fractions
    // below k, we are just interested in their number.
    //
    // The other part is finding the k-th fraction: the best fraction.
    // The trick there was to notice that for each i in arr, the possible
    // candidates for the answers were the fractions that are <= mid, and specifically
    // the first one after the inner loop exits.
    //
    // Tbh, this was not an easy problem conceptually. It's one of these problems
    // that are quite easy to code, but are not that easy to figure out, contrary
    // to some of the dp problems labeled as hard, which are sometimes easy to
    // conceptualize but quite annoying to implement.
    pub fn kth_smallest_prime_fraction_another(arr: Vec<i32>, k: i32) -> Vec<i32> {
        let (mut left, mut right) = (0.0, 1.0);
        let k = k as usize;

        while left < right {
            let mid = (left + right) / 2.0;
            let mut best = [0, 1];
            let mut count = 0;
            let mut j = 1;

            for i in 0..arr.len() - 1 {
                while j < arr.len() && mid * (arr[j] as f64) < arr[i] as f64 {
                    j += 1;
                }

                count += arr.len() - j;
                if j < arr.len() && best[0] * arr[j] < best[1] * arr[i] {
                    best = [arr[i], arr[j]];
                }
            }

            match count.cmp(&k) {
                std::cmp::Ordering::Less => left = mid,
                std::cmp::Ordering::Equal => return best.to_vec(),
                std::cmp::Ordering::Greater => right = mid,
            }
        }
        vec![0, 1]
    }
}

fn main() {
    println!("Hello, world!");
}
