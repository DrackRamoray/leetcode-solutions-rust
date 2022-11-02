use std::collections::HashMap;

pub struct Solution;

impl Solution {
    fn is_scramble(s1: String, s2: String) -> bool {
        let n = s1.len();
        let mut mp = HashMap::new();

        Self::dfs(&s1, &s2, &mut mp, 0, n, 0, n)
    }

    fn dfs(s1: &str, s2: &str, mp: &mut HashMap<(usize, usize, usize, usize), bool>, u: usize, v: usize, x: usize, y: usize) -> bool {
        if let Some(&ans) = mp.get(&(u, v, x, y)) {
            return ans;
        }

        let ans = if s1[u..v] == s2[x..y] {
            true
        } else {
            let mut ans = false;
            let n = v - u;

            for i in 1..n {
                if Self::dfs(s1, s2, mp, u, u + i, x, x + i) && Self::dfs(s1, s2, mp, u + i, v, x + i, y) || Self::dfs(s1, s2, mp, u, u + i, y - i, y) && Self::dfs(s1, s2, mp, u + i, v, x, y - i) {
                    ans = true;
                    break;
                }
            }

            ans
        };

        mp.insert((u, v, x, y), ans);

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::is_scramble("great".into(), "rgeat".into()), true);
        assert_eq!(Solution::is_scramble("abcde".into(), "caebd".into()), false);
        assert_eq!(Solution::is_scramble("a".into(), "a".into()), true);
    }
}
