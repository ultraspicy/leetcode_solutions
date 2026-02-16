use itertools::Itertools;
use super::Solution;

impl Solution {
    pub fn validate_coupons(code: Vec<String>, business_line: Vec<String>, is_active: Vec<bool>) -> Vec<String> {
        // nested tuple
        // sort_by()
        code.into_iter().zip(business_line.into_iter())
            .zip(is_active.into_iter())
            .filter_map(|((c, bl), is_active)|{
                if Self::is_valid(&c, &bl, is_active) {
                    Some((c, bl))
                } else {
                    None
                }
            })
            .sorted_by(|a, b| {
                Self::get_business_priority(&a.1)
                    .cmp(&Self::get_business_priority(&b.1))
                    .then_with(|| a.0.cmp(&b.0))
            })
            .map(|tuple| tuple.0)
            .collect::<Vec<_>>()
    }

    fn is_valid (code: &str, business_line: &str, is_active: bool) -> bool {
        if !is_active {
            return false;
        }

        if code.len() == 0 || code.chars().any(|ch| !ch.is_ascii_alphanumeric() && ch != '_') {
            return false
        }

        // matches macro
        matches!(business_line, "electronics" | "grocery" | "pharmacy" | "restaurant" )
    }

    fn get_business_priority (business_line: &str) -> usize {
        match business_line {
            "electronics" => 0,
            "grocery" => 1,
            "pharmacy" => 2,
            "restaurant" => 3,
            _ => 255
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_test() {
        let code = vec![
            "SAVE20".to_string(),
            "".to_string(),
            "PHARMA5".to_string(),
            "SAVE@20".to_string(),
        ];
        let business_line = vec![
            "restaurant".to_string(),
            "grocery".to_string(),
            "pharmacy".to_string(),
            "restaurant".to_string(),
        ];
        let is_active = vec![true, true, true, true];

        let result = Solution::validate_coupons(code, business_line, is_active);
        assert_eq!(result, vec!["PHARMA5", "SAVE20"])
    }
}
