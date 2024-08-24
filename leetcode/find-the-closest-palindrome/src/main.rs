// https://leetcode.com/problems/find-the-closest-palindrome/description/
pub struct Solution;

impl Solution {
    pub fn nearest_palindromic(n: String) -> String {
        let l = n.len();
        let bytes = n.as_bytes();

        if l == 1 {
            return std::str::from_utf8(&[bytes[0] - 1]).unwrap().to_string();
        }

        let mut candidates = vec![];

        candidates.push(format!("{}{}{}", "1", "0".repeat(l - 1), "1"));
        candidates.push("9".repeat(l - 1).to_string());

        let prefix = bytes[0..l / 2].to_vec();
        // No change in the middle digit
        let mut no_change = prefix.clone();
        if l % 2 == 1 {
            no_change.push(bytes[l / 2]);
        }
        no_change.extend(prefix.iter().rev());
        if no_change != bytes {
            candidates.push(std::str::from_utf8(&no_change).unwrap().to_owned());
        }
        // Increase by one
        let mut increased = prefix.clone();
        let middle_digit = bytes[(l - 1) / 2];
        if l % 2 == 1 && middle_digit < b'9' {
            increased.push(middle_digit + 1);
            increased.extend(prefix.iter().rev());
            candidates.push(std::str::from_utf8(&increased).unwrap().to_string());
        } else {
            let increased = std::str::from_utf8(&prefix)
                .unwrap()
                .parse::<u64>()
                .unwrap()
                + 1;
            let increased = format!("{increased}");
            let mut increased = increased.as_bytes().to_vec();
            increased.extend(increased.clone().iter().rev());
            candidates.push(std::str::from_utf8(&increased).unwrap().to_string());
        }

        // Decrease by one
        let mut decreased = prefix.clone();
        if l % 2 == 1 && middle_digit > b'0' {
            decreased.push(middle_digit - 1);
            decreased.extend(prefix.iter().rev());
            candidates.push(std::str::from_utf8(&decreased).unwrap().to_string());
        } else {
            let decreased = std::str::from_utf8(&prefix)
                .unwrap()
                .parse::<u64>()
                .unwrap()
                - 1;
            let decreased = format!("{decreased}");
            let mut decreased = decreased.as_bytes().to_vec();
            decreased.extend(decreased.clone().iter().rev());
            candidates.push(std::str::from_utf8(&decreased).unwrap().to_string());
        }

        println!("{candidates:?}");

        let original_number = n.parse::<u64>().unwrap();
        candidates.sort_by(|first, second| {
            let first = first.parse::<u64>().unwrap();
            let second = second.parse::<u64>().unwrap();
            (first
                .abs_diff(original_number)
                .cmp(&(second.abs_diff(original_number))))
            .then_with(|| first.cmp(&second))
        });

        candidates.into_iter().next().unwrap()
    }
}

fn main() {
    let test_cases = ["123", "1", "137401923651236", "88", "9009"];

    for n in test_cases {
        println!("{}", Solution::nearest_palindromic(n.to_string()));
    }
}
