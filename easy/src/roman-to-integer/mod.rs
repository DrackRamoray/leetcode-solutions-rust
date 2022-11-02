struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut ans = 0;
        let sr = s.as_bytes();
        let mut prev = Solution::get_value(sr[0]);

        for i in 1..sr.len() {
            let cur = Solution::get_value(sr[i]);

            if cur > prev {
                ans -= prev;
            } else {
                ans += prev;
            }

            prev = cur;
        }

        ans += prev;

        ans
    }

    fn get_value(ch: u8) -> i32 {
        match ch {
            b'I' => 1,
            b'V' => 5,
            b'X' => 10,
            b'L' => 50,
            b'C' => 100,
            b'D' => 500,
            b'M' => 1000,
            _ => 0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::roman_to_int("III".to_string()), 3);
        assert_eq!(Solution::roman_to_int("IV".to_string()), 4);
        assert_eq!(Solution::roman_to_int("IX".to_string()), 9);
        assert_eq!(Solution::roman_to_int("LVIII".to_string()), 58);
        assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994);
    }
}
