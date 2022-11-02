struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.len() < 2 {
            return s;
        }

        let len = s.len();
        let mut left = 0;
        let mut dp = vec![vec![false;len];len];
        let mut num = 1;

        for i in 0 ..len {
            dp[i][i] = true;
        }

        let sv = s.as_bytes();

        for k in 2..=len {
            for i in 0..len {
                let j = k + i - 1;

                if j >= len {
                    break;
                }

                if sv[i] != sv[j] {
                    dp[i][j] = false;
                } else {
                    if j - i < 3 {
                        dp[i][j] = true;
                    } else {
                        dp[i][j] = dp[i+1][j-1];
                    }
                }

                if dp[i][j] && j - i + 1 > num {
                    num = j - i + 1;
                    left = i
                }
            }
        }

        s[left..left+num].into()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::longest_palindrome("babad".to_owned()), "bab".to_owned());
        assert_eq!(Solution::longest_palindrome("cbbd".to_owned()), "bb".to_owned());
    }
}
