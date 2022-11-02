struct Solution;

impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let n = s.len();
        let mut dp = vec![vec![true;n];n];

        for i in (0..n-1).rev() {
            for j in i+1..n {
                dp[i][j] = s[i..=i] == s[j..=j] && dp[i+1][j-1];
            }
        }

        let mut ans = vec![];

        Self::dfs(&mut ans, &mut vec![], &dp, &s, 0);

        ans
    }

    fn dfs(ans: &mut Vec<Vec<String>>, selected: &mut Vec<String>, dp: &Vec<Vec<bool>>, s: &str, begin: usize) {
        if begin == s.len() {
            ans.push(selected.to_vec());
            return;
        }

        for i in begin..s.len() {
            if dp[begin][i] {
                selected.push(s[begin..=i].to_string());
                Self::dfs(ans, selected, dp, s, i + 1);
                selected.pop();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::partition("aab".into()), vec![vec!["a".to_string(),"a".to_string(),"b".to_string()],vec!["aa".to_string(),"b".to_string()]]);
        assert_eq!(Solution::partition("a".into()), vec![vec!["a".to_string()]]);
    }
}
