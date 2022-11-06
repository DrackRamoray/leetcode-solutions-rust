pub struct Solution;

impl Solution {
    pub fn is_additive_number(num: String) -> bool {
        let n = num.len();
        let mut ans = false;

        Self::dfs(&num, &mut Vec::new(), &mut ans, n, 0);

        ans
    }

    fn dfs(num: &str, selected: &mut Vec<u64>, ans: &mut bool, n: usize, begin: usize) {
        if begin == n {
            if selected.len() >= 3 {
                *ans = true;
            }

            return;
        }

        for i in begin+1..=n {
            if &num[begin..=begin] == "0" && i > begin + 1 {
                return;
            }

            if let Ok(v) = num[begin..i].parse::<u64>() {
                if selected.len() < 2 {
                    selected.push(v);
                    Self::dfs(num, selected, ans, n, i);
                    selected.pop();
                } else {
                    if selected[selected.len() - 2] + selected[selected.len() - 1] == v {
                        selected.push(v);
                        Self::dfs(num, selected, ans, n, i);
                        selected.pop();
                    }
                }
            }
        }
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::is_additive_number("112358".to_string()), true);
    assert_eq!(Solution::is_additive_number("199100199".to_string()), true);
}
