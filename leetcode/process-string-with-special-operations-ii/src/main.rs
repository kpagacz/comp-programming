// https://leetcode.com/problems/process-string-with-special-operations-ii/description/?envType=daily-question&envId=2026-06-17
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn process_str(s: String, mut k: i64) -> char {
        let mut l = 0i64;
        for c in s.as_bytes() {
            match c {
                b'*' => {
                    l -= 1;
                    l = l.max(0);
                }
                b'#' => l *= 2,
                b'%' => {}
                _ => l += 1,
            }
        }
        if k >= l {
            return '.';
        }

        for c in s.chars().rev() {
            match c {
                '*' => l += 1,
                '#' => {
                    l /= 2;
                    if k >= l {
                        k -= l;
                    }
                }
                '%' => {
                    k = l - 1 - k;
                }
                _ => {
                    l -= 1;
                    if k > l {
                        return '.';
                    }
                    if l == k {
                        return c;
                    }
                }
            }
        }
        unreachable!()
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let s = "a#b%*".to_string();
        let k = 1;
        assert_eq!(Solution::process_str(s, k), 'a');
    }

    #[test]
    fn test2() {
        let s = "cd%#*#".to_string();
        let k = 3;
        assert_eq!(Solution::process_str(s, k), 'd');
    }

    #[test]
    fn test3() {
        let s = "z*#".to_string();
        let k = 0;
        assert_eq!(Solution::process_str(s, k), '.');
    }

    #[test]
    fn test4() {
        let s = "x%#d%".to_string();
        let k = 649453232458258;
        assert_eq!(Solution::process_str(s, k), '.');
    }

    #[test]
    fn test5() {
        let s = "%#*gm#xib".to_string();
        let k = 2;
        assert_eq!(Solution::process_str(s, k), 'g');
    }
}
