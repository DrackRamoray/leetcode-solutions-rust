pub struct Solution;

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let v = s.chars().collect::<Vec<char>>();
        let n = v.len();

        if v[0] == '0' {
            return 0;
        }

        let mut dp = vec![0;n];
        dp[0] = 1;

        for i in 1..n {
            if v[i] == '0' {
                if v[i-1] == '1' || v[i-1] == '2' {
                    dp[i] = match i > 1 {
                        true => dp[i-2],
                        false => 1
                    };
                } else {
                    return 0;
                }
            } else if v[i-1] == '1' || (v[i-1] == '2' && v[i] >= '1' && v[i] <= '6') {
                dp[i] = match i > 1 {
                    true => dp[i-1] + dp[i-2],
                    false => dp[i-1] + 1
                };
            } else {
                dp[i] = dp[i-1];
            }
        }

        dp[n-1]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::num_decodings("12".into()), 2);
        assert_eq!(Solution::num_decodings("226".into()), 3);
        assert_eq!(Solution::num_decodings("0".into()), 0);
    }
}
