// https://leetcode.com/problems/maximum-candies-you-can-get-from-boxes/description/?envType=daily-question&envId=2025-06-03
pub struct Solution;

impl Solution {
    pub fn max_candies(
        status: Vec<i32>,
        candies: Vec<i32>,
        keys: Vec<Vec<i32>>,
        contained_boxes: Vec<Vec<i32>>,
        initial_boxes: Vec<i32>,
    ) -> i32 {
        let mut gathered_candy = 0;
        use std::collections::VecDeque;
        let mut open_boxes = VecDeque::new();
        let mut gathered_closed_boxes = vec![0; keys.len()];
        let mut gathered_keys = vec![false; keys.len()];
        for box_num in initial_boxes {
            let box_num = box_num as usize;
            if status[box_num] == 1 {
                open_boxes.push_back(box_num);
            } else {
                gathered_closed_boxes[box_num] = 1;
            }
        }

        while let Some(opened_box) = open_boxes.pop_front() {
            for &contained_box in &contained_boxes[opened_box] {
                let contained_box = contained_box as usize;
                if status[contained_box] == 1 || gathered_keys[contained_box] {
                    open_boxes.push_back(contained_box);
                } else {
                    gathered_closed_boxes[contained_box] = 1;
                }
            }
            for &key in &keys[opened_box] {
                let key = key as usize;
                if gathered_keys[key] {
                    continue;
                }
                gathered_keys[key] = true;
                if gathered_closed_boxes[key] == 1 {
                    open_boxes.push_back(key);
                }
            }
            gathered_candy += candies[opened_box];
        }

        gathered_candy
    }
}

fn main() {
    let test_cases = [(
        vec![
            1, 1, 0, 1, 1, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 1,
            0, 0, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 0, 0, 1, 0, 0,
        ],
        vec![
            732, 320, 543, 300, 814, 568, 947, 685, 142, 111, 805, 233, 813, 306, 55, 1, 290, 944,
            36, 592, 150, 596, 372, 299, 644, 445, 605, 202, 64, 807, 753, 731, 552, 766, 119, 862,
            453, 136, 43, 572, 801, 518, 936, 408, 515, 215, 492, 738, 154,
        ],
        vec![
            vec![42, 2, 24, 8, 39, 16, 46],
            vec![20, 39, 46, 21, 32, 31, 43, 16, 12, 23, 3],
            vec![21, 14, 30, 2, 11, 13, 27, 37, 4, 48],
            vec![16, 17, 15, 6],
            vec![31, 14, 3, 32, 35, 19, 42, 43, 44, 29, 25, 41],
            vec![7, 39, 2, 3, 40, 28, 37, 35, 43, 22, 6, 23, 48, 10, 21, 11],
            vec![27, 1, 37, 3, 45, 32, 30, 26, 16, 2, 35, 19, 31, 47, 5, 14],
            vec![28, 35, 23, 17, 6],
            vec![6, 39, 34, 22],
            vec![44, 29, 36, 31, 40, 22, 9, 11, 17, 25, 1, 14, 41],
            vec![39, 37, 11, 36, 17, 42, 13, 12, 7, 9, 43, 41],
            vec![23, 16, 32, 37],
            vec![36, 39, 21, 41],
            vec![15, 27, 5, 42],
            vec![11, 5, 18, 48, 25, 47, 17, 0, 41, 26, 9, 29],
            vec![18, 36, 40, 35, 12, 33, 11, 5, 44, 14, 46, 7],
            vec![48, 22, 11, 33, 14],
            vec![44, 12, 3, 31, 25, 15, 18, 28, 42, 43],
            vec![36, 9, 0, 42],
            vec![1, 22, 3, 24, 9, 11, 43, 8, 35, 5, 41, 29, 40],
            vec![15, 47, 32, 28, 33, 31, 4, 43],
            vec![1, 11, 6, 37, 28],
            vec![46, 20, 47, 32, 26, 15, 11, 40],
            vec![33, 45, 26, 40, 12, 3, 16, 18, 10, 28, 5],
            vec![14, 6, 4, 46, 34, 9, 33, 24, 30, 12, 37],
            vec![45, 24, 18, 31, 32, 39, 26, 27],
            vec![29, 0, 32, 15, 7, 48, 36, 26, 33, 31, 18, 39, 23, 34, 44],
            vec![25, 16, 42, 31, 41, 35, 26, 10, 3, 1, 4, 29],
            vec![8, 11, 5, 40, 9, 18, 10, 16, 26, 30, 19, 2, 14, 4],
            vec![],
            vec![0, 20, 17, 47, 41, 36, 23, 42, 15, 13, 27],
            vec![7, 15, 44, 38, 41, 42, 26, 19, 5, 47],
            vec![],
            vec![37, 22],
            vec![21, 24, 15, 48, 33, 6, 39, 11],
            vec![23, 7, 3, 29, 10, 40, 1, 16, 6, 8, 27],
            vec![27, 29, 25, 26, 46, 15, 16],
            vec![33, 40, 10, 38, 13, 19, 17, 23, 32, 39, 7],
            vec![35, 3, 39, 18],
            vec![47, 11, 27, 23, 35, 26, 43, 4, 22, 38, 44, 31, 1, 0],
            vec![],
            vec![18, 43, 46, 9, 15, 3, 42, 31, 13, 4, 12, 39, 22],
            vec![42, 45, 47, 18, 26, 41, 38, 9, 0, 35, 8, 16, 29, 36, 31],
            vec![3, 20, 29, 12, 46, 41, 23, 4, 9, 27],
            vec![19, 33],
            vec![32, 18],
            vec![17, 28, 7, 35, 6, 22, 4, 43],
            vec![41, 31, 20, 28, 35, 32, 24, 23, 0, 33, 18, 39, 29, 30, 16],
            vec![43, 47, 46],
        ],
        vec![
            vec![14],
            vec![],
            vec![26],
            vec![4, 47],
            vec![],
            vec![6],
            vec![39, 43, 46],
            vec![30],
            vec![],
            vec![],
            vec![0, 3],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![27],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![12],
            vec![],
            vec![],
            vec![41],
            vec![],
            vec![31],
            vec![20, 29],
            vec![13, 35],
            vec![18],
            vec![10, 40],
            vec![],
            vec![38],
            vec![],
            vec![],
            vec![19],
            vec![5],
            vec![],
            vec![],
            vec![11],
            vec![1],
            vec![15],
            vec![],
            vec![],
            vec![],
            vec![24],
            vec![],
            vec![],
            vec![],
            vec![],
        ],
        vec![
            2, 7, 8, 9, 16, 17, 21, 22, 23, 25, 28, 32, 33, 34, 36, 37, 42, 44, 45, 48,
        ],
    )];
    for (status, candies, keys, contained_boxes, initial_boxes) in test_cases {
        println!(
            "{}",
            Solution::max_candies(status, candies, keys, contained_boxes, initial_boxes)
        );
    }
}
