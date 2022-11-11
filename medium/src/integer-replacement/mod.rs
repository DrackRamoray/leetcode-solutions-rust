use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn integer_replacement(n: i32) -> i32 {
        Self::dfs(n, &mut HashMap::new())
    }

    fn dfs(n: i32, memo: &mut HashMap<i32, i32>) -> i32 {
        if n == 1 {
            return 0;
        }

        if let Some(&v) = memo.get(&n) {
            return v;
        }

        let times = if n % 2 == 0 {
            1 + Self::dfs(n / 2, memo)
        } else {
            2 + Self::dfs(n / 2, memo).min(Self::dfs(n / 2 + 1, memo))
        };

        memo.insert(n, times);

        times
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::integer_replacement(8), 3);
    assert_eq!(Solution::integer_replacement(7), 4);
    assert_eq!(Solution::integer_replacement(4), 2);
}
