pub struct Solution;

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let values = vec![1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
        let romans = vec!["M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I"];
        let mut ans = Vec::new();
        let mut i = 0;
        let mut n = num;

        while i < values.len() && n >= 0 {
            while values[i] <= n {
                n -= values[i];
                ans.push(romans[i]);
            }
            i += 1;
        }

        ans.into_iter().collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::int_to_roman(3), "III".to_owned());
        assert_eq!(Solution::int_to_roman(4), "IV".to_owned());
        assert_eq!(Solution::int_to_roman(9), "IX".to_owned());
        assert_eq!(Solution::int_to_roman(58), "LVIII".to_owned());
        assert_eq!(Solution::int_to_roman(1994), "MCMXCIV".to_owned());
    }
}
