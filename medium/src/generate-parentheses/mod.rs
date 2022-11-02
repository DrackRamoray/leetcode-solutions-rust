pub struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut ans = vec![];

        Self::dfs(&mut ans, &mut String::new(), n, 0, 0);

        ans
    }

    fn dfs(ans: &mut Vec<String>, selected: &mut String, n: i32, left: i32, right: i32) {
        if selected.len() == (2 * n as usize) {
            ans.push(selected.clone());
            return;
        }

        if left < n {
            selected.push('(');
            Self::dfs(ans, selected, n , left + 1, right);
            selected.pop();
        }

        if right < left {
            selected.push(')');
            Self::dfs(ans, selected, n , left, right + 1);
            selected.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::generate_parenthesis(3), vec!["((()))".to_owned(),"(()())".to_owned(),"(())()".to_owned(),"()(())".to_owned(),"()()()".to_owned()]);
        assert_eq!(Solution::generate_parenthesis(1), vec!["()".to_owned()]);
    }
}
