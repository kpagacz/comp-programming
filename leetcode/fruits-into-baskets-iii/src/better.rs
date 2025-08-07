/// A segment tree for efficiently finding and updating basket capacities.
struct SegmentTree {
    size: usize,    // The number of leaf nodes (next power of 2)
    data: Vec<i32>, // The segment tree storing maximum basket capacities
}

struct Solution;

impl Solution {
    pub fn num_of_unplaced_fruits(fruits: Vec<i32>, baskets: Vec<i32>) -> i32 {
        let mut tree = SegmentTree::new(&baskets);
        let mut unplaced = 0;

        for fruit in fruits {
            if let Some(idx) = tree.find_first_ge(fruit) {
                tree.update(idx, 0); // Mark the basket as used (set capacity to 0)
            } else {
                unplaced += 1; // No suitable basket found
            }
        }

        unplaced
    }
}

impl SegmentTree {
    /// Constructs a segment tree from the given basket capacities.
    fn new(values: &[i32]) -> Self {
        let size = values.len().next_power_of_two();
        let mut data = vec![0; 2 * size];

        // Copy values into leaf nodes
        data[size..size + values.len()].copy_from_slice(values);

        // Build the segment tree by computing parent nodes
        for i in (1..size).rev() {
            data[i] = data[2 * i].max(data[2 * i + 1]);
        }

        Self { size, data }
    }

    /// Finds the leftmost basket that can hold at least `val` fruits.
    /// Returns the basket index if found, otherwise `None`.
    fn find_first_ge(&self, val: i32) -> Option<usize> {
        let mut pos = 1;
        while pos < self.size {
            pos *= 2;
            if self.data[pos] < val {
                pos += 1; // Move to the right child if the left one is too small
            }
        }

        if pos >= self.size && self.data[pos] >= val {
            Some(pos - self.size) // Convert tree index to original index
        } else {
            None
        }
    }

    /// Updates the basket at `idx` with the new capacity `val`,
    /// then propagates changes up the segment tree.
    fn update(&mut self, idx: usize, val: i32) {
        let mut pos = self.size + idx;
        self.data[pos] = val; // Update the leaf node

        // Propagate the update to parent nodes using bitwise XOR for sibling index calculation
        while pos > 1 {
            self.data[pos / 2] = self.data[pos].max(self.data[pos ^ 1]);
            pos /= 2;
        }
    }
}
