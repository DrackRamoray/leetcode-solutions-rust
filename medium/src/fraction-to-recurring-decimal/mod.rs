pub struct Solution;

impl Solution {
    pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
        if numerator == 0 {
            return "0".to_string();
        }

        let mut ans = String::new();

        if (numerator < 0) ^ (denominator < 0) {
            ans.push('-');
        }

        let n = (numerator as i64).abs();
        let d = (denominator as i64).abs();
        (n / d).to_string().chars().for_each(|c| ans.push(c));
        let mut r = n % d;

        if r == 0 {
            return ans;
        }

        let mut dup = std::collections::HashMap::new();

        ans.push('.');

        while r != 0 {
            if let Some(i) = dup.insert(r, ans.len()) {
                ans.insert(i, '(');
                ans.push(')');
                return ans;
            }

            r *= 10;
            (r / d).to_string().chars().for_each(|c| ans.push(c));
            r %= d;
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::fraction_to_decimal(1, 2), "0.5".to_string());
        assert_eq!(Solution::fraction_to_decimal(2, 1), "2".to_string());
        assert_eq!(Solution::fraction_to_decimal(4, 333), "0.(012)".to_string());
    }
}
