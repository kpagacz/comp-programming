// https://leetcode.com/problems/strong-password-checker-ii/description/
pub struct Solution;

impl Solution {
    pub fn strong_password_checker_ii(password: String) -> bool {
        let password = password.as_bytes();

        use std::ops::Deref;
        fn is_ascii_special_char<T: Deref<Target = u8>>(b: T) -> bool {
            let special_chars = "!@#$%^&*()-+";
            let special_chars = special_chars.as_bytes();
            special_chars.contains(&*b)
        }
        password.len() >= 8
            && password.iter().any(|b| b.is_ascii_lowercase())
            && password.iter().any(|b| b.is_ascii_uppercase())
            && password.iter().any(|b| b.is_ascii_digit())
            && password.iter().any(is_ascii_special_char)
            && password.windows(2).all(|window| window[0] != window[1])
    }
}

fn main() {
    println!("Hello, world!");
}
