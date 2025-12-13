// https://leetcode.com/problems/coupon-code-validator/description/?envType=daily-question&envId=2025-12-13
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn validate_coupons(
        code: Vec<String>,
        business_line: Vec<String>,
        is_active: Vec<bool>,
    ) -> Vec<String> {
        #[derive(Eq, PartialEq, Ord, PartialOrd)]
        enum BusinessLine {
            Electronics,
            Grocery,
            Pharmacy,
            Restaurant,
        }

        let mut coupons = vec![];
        for i in 0..code.len() {
            let code = code[i].clone();
            let business_line = &business_line[i];
            let is_active = is_active[i];
            if is_active
                && code
                    .as_bytes()
                    .iter()
                    .all(|c| c.is_ascii_alphanumeric() || *c == b'_')
                && !code.is_empty()
            {
                let business_line = match business_line.as_str() {
                    "electronics" => BusinessLine::Electronics,
                    "grocery" => BusinessLine::Grocery,
                    "pharmacy" => BusinessLine::Pharmacy,
                    "restaurant" => BusinessLine::Restaurant,
                    _ => continue,
                };
                coupons.push((business_line, code));
            }
        }
        coupons.sort_unstable();
        coupons.into_iter().map(|coupon| coupon.1).collect()
    }
}

fn main() {
    let test_cases = [(
        vec!["GROCERY15", "ELECTRONICS_50", "DISCOUNT10"],
        vec!["grocery", "electronics", "invalid"],
        vec![false, true, true],
        vec!["ELECTRONICS_50"],
    )];

    for (code, business_line, is_active, exp) in test_cases {
        let code = code.into_iter().map(str::to_owned).collect();
        let business_line = business_line.into_iter().map(str::to_owned).collect();
        println!(
            "{:?} exp {exp:?}",
            Solution::validate_coupons(code, business_line, is_active)
        );
    }
}
