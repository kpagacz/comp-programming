// https://leetcode.com/problems/separate-squares-ii/description/?envType=daily-question&envId=2026-01-14
struct Solution;

struct SegmentTree {
    count: Vec<i32>,
    covered: Vec<i32>,
    xs: Vec<i32>,
    n: usize,
}

impl SegmentTree {
    fn new(xs: Vec<i32>) -> Self {
        let n = xs.len() - 1;
        Self {
            count: vec![0; 4 * n],
            covered: vec![0; 4 * n],
            xs,
            n,
        }
    }

    fn query(&self) -> i32 {
        self.covered[0]
    }

    fn update(&mut self, qleft: i32, qright: i32, delta: i32) {
        self.modify(qleft, qright, delta, 0, self.n - 1, 0);
    }

    fn modify(
        &mut self,
        qleft: i32,
        qright: i32,
        qval: i32,
        left: usize,
        right: usize,
        pos: usize,
    ) {
        if self.xs[right + 1] <= qleft || self.xs[left] >= qright {
            return;
        }
        if qleft <= self.xs[left] && self.xs[right + 1] <= qright {
            self.count[pos] += qval;
        } else {
            let mid = (left + right) / 2;
            self.modify(qleft, qright, qval, left, mid, pos * 2 + 1);
            self.modify(qleft, qright, qval, mid + 1, right, pos * 2 + 2);
        }

        if self.count[pos] > 0 {
            self.covered[pos] = self.xs[right + 1] - self.xs[left];
        } else if left == right {
            self.covered[pos] = 0;
        } else {
            self.covered[pos] = self.covered[pos * 2 + 1] + self.covered[pos * 2 + 2];
        }
    }
}

#[allow(dead_code)]
impl Solution {
    pub fn separate_squares(squares: Vec<Vec<i32>>) -> f64 {
        use std::collections::BTreeSet;
        let mut events: Vec<_> = Vec::new();
        let mut xs = BTreeSet::new();

        for square in squares {
            let (x, xr, y, l) = (square[0], square[0] + square[2], square[1], square[2]);
            events.push((y, 1, x, xr));
            events.push((y + l, -1, x, xr));
            xs.insert(x);
            xs.insert(xr);
        }

        events.sort();
        let xs = xs.into_iter().collect::<Vec<_>>();
        let mut st = SegmentTree::new(xs);

        let mut prefix_sums = vec![];
        let mut prefix_widths = vec![];
        let mut total_area = 0;
        let mut previous_y = events[0].0;

        for &(y, delta, x, xr) in &events {
            let width = st.query();
            total_area += width as i64 * (y - previous_y) as i64;
            st.update(x, xr, delta);
            prefix_sums.push(total_area);
            prefix_widths.push(st.query());
            previous_y = y;
        }

        let target = ((total_area as f64 + 1.0) / 2.0).floor() as i64;
        let pos = prefix_sums
            .partition_point(|&prefix_area| prefix_area < target)
            .saturating_sub(1);

        let area = prefix_sums[pos];
        let width = prefix_widths[pos];
        let height = events[pos].0;
        println!("prefix sums: {prefix_sums:?}");
        println!("prefix widths: {prefix_widths:?}");
        println!("pos: {pos} area: {area} width:{width} height:{height}");

        height as f64 + (total_area as f64 - area as f64 * 2.0) / (width as f64 * 2.0)
    }
}

fn main() {
    let test_cases = [
        (vec![vec![0, 0, 1], vec![2, 2, 1]], 1.00000),
        (vec![vec![0, 0, 2], vec![1, 1, 1]], 1.00000),
    ];
    for (squares, exp) in test_cases {
        println!("{}, exp {exp}", Solution::separate_squares(squares));
    }
}
