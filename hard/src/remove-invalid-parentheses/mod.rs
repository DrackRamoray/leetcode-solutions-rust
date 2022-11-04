use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn remove_invalid_parentheses(s: String) -> Vec<String> {
        let n = s.len();
        let mut left = 0;
        let mut right = 0;
        let mut cnt_left = 0;
        let mut cnt_right = 0;

        for &u in s.as_bytes() {
            if u == b'(' {
                left += 1;
                cnt_left += 1;
            } else if u== b')' {
                if left != 0 {
                    left -= 1;
                } else {
                    right += 1;
                }
                cnt_right += 1;
            }
        }

        let target = n - left - right;
        let max = cnt_left.min(cnt_right);
        let mut ans = HashSet::new();

        Self::dfs(&s, &mut ans, "".to_string(), target, 0, max, left as i32, right as i32, 0);

        ans.into_iter().collect::<Vec<String>>()
    }

    fn dfs(s: &str, ans: &mut HashSet<String>, selected: String, target: usize, cur: usize, max: i32, left: i32, right: i32, num: i32) {
        if left < 0 || right < 0 || num < 0 || num > max {
            return;
        }

        if left == 0 && right == 0 {
            if selected.len() == target {
                ans.insert(selected.clone());
            }
        }

        if cur == s.len() {
            return;
        }

        if &s[cur..=cur] == "(" {
            Self::dfs(s, ans, selected.clone() + &s[cur..=cur], target, cur + 1, max, left, right, num + 1);
            Self::dfs(s, ans, selected, target, cur + 1, max, left - 1, right, num);
        } else if &s[cur..=cur] == ")" {
            Self::dfs(s, ans, selected.clone() + &s[cur..=cur], target, cur + 1, max, left, right, num - 1);
            Self::dfs(s, ans, selected, target, cur + 1, max, left, right - 1, num);
        } else {
            Self::dfs(s, ans, selected + &s[cur..=cur], target, cur + 1, max, left, right, num);
        }
    }
}

#[test]
fn it_works() {
    use assist::vec_stringify;
    let s = "()())()".to_string();
    let mut res = vec_stringify!(["()()()", "(())()"]);
    let mut ans = Solution::remove_invalid_parentheses(s);
    ans.sort();
    res.sort();
    assert_eq!(ans, res);
    let s = "(a)())()".to_string();
    let mut res = vec_stringify!(["(a)()()", "(a())()"]);
    let mut ans = Solution::remove_invalid_parentheses(s);
    ans.sort();
    res.sort();
    assert_eq!(ans, res);
    let s = ")(".to_string();
    let mut res = vec_stringify!([""]);
    let mut ans = Solution::remove_invalid_parentheses(s);
    ans.sort();
    res.sort();
    assert_eq!(ans, res);
    let s = ")(f".to_string();
    let mut res = vec_stringify!(["f"]);
    let mut ans = Solution::remove_invalid_parentheses(s);
    ans.sort();
    res.sort();
    assert_eq!(ans, res);
}
