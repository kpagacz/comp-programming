// https://leetcode.com/problems/find-the-largest-area-of-square-inside-two-rectangles/description/?envType=daily-question&envId=2026-01-17
struct Solution;

struct Rectangle {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}
impl Rectangle {
    fn new(x1: i32, y1: i32, x2: i32, y2: i32) -> Self {
        Self { x1, y1, x2, y2 }
    }
    fn lower_x(&self) -> i32 {
        self.x1
    }

    fn lower_y(&self) -> i32 {
        self.y1
    }

    fn upper_x(&self) -> i32 {
        self.x2
    }

    fn upper_y(&self) -> i32 {
        self.y2
    }

    fn side_lengths(&self) -> (i32, i32) {
        ((self.x2 - self.x1).abs(), (self.y2 - self.y1).abs())
    }
}
#[allow(dead_code)]
impl Solution {
    pub fn largest_square_area(bottom_left: Vec<Vec<i32>>, top_right: Vec<Vec<i32>>) -> i64 {
        let mut max_area = 0;
        for i in 0..bottom_left.len() {
            for j in i + 1..bottom_left.len() {
                let first = Rectangle::new(
                    bottom_left[i][0],
                    bottom_left[i][1],
                    top_right[i][0],
                    top_right[i][1],
                );
                let second = Rectangle::new(
                    bottom_left[j][0],
                    bottom_left[j][1],
                    top_right[j][0],
                    top_right[j][1],
                );
                let intersection = Self::intersection(first, second);
                max_area = max_area.max(Self::area_of_square_that_fits_rectangle(intersection));
            }
        }
        max_area
    }

    fn intersection(first: Rectangle, second: Rectangle) -> Rectangle {
        // Rectangle are listed as bottom left and top right corners,
        // so we don't need to deal with other scenarios
        if first.lower_x() <= second.upper_x()
            && second.lower_x() <= first.upper_x()
            && first.lower_y() <= second.upper_y()
            && second.lower_y() <= first.upper_y()
        {
            Rectangle::new(
                first.lower_x().max(second.lower_x()),
                first.lower_y().max(second.lower_y()),
                first.upper_x().min(second.upper_x()),
                first.upper_y().min(second.upper_y()),
            )
        } else {
            Rectangle::new(0, 0, 0, 0)
        }
    }

    fn area_of_square_that_fits_rectangle(rectangle: Rectangle) -> i64 {
        let (first, second) = rectangle.side_lengths();
        let square_side = first.min(second);
        (square_side as i64) * (square_side as i64)
    }
}

fn main() {
    println!("Hello, world!");
}
