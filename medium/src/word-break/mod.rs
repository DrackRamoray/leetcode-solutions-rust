pub struct Solution;

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let dict = word_dict.iter().collect::<std::collections::HashSet<_>>();
        let n = s.len();
        let mut dp = vec![false;n+1];
        dp[0] = true;

        for i in 0..=n {
            for j in 0..i {
                if dp[j] && dict.contains(&s[j..i].to_string()) {
                    dp[i] = true;
                    break;
                }
            }
        }

        dp[n]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::word_break("leetcode".into(), vec!["leet".to_string(), "code".to_string()]), true);
        assert_eq!(Solution::word_break("applepenapple".into(), vec!["apple".to_string(), "pen".to_string()]), true);
        assert_eq!(Solution::word_break("catsandog".into(), vec!["cats".to_string(), "dog".to_string(), "sand".to_string(), "and".to_string(), "cat".to_string()]), false);
    }
}
