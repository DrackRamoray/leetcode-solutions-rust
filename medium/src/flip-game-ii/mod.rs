use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn can_win(current_state: String) -> bool {
        let n = current_state.len();
        let mut s = current_state.bytes().collect::<Vec<u8>>();

        Self::dfs(&mut s, &mut HashMap::new(), n)
    }

    fn dfs(s: &mut Vec<u8>, mp: &mut HashMap<Vec<u8>, bool>, n: usize) -> bool {
        if let Some(&b) = mp.get(s) {
            return b;
        }

        let mut ans = false;

        for i in 0..n - 1 {
            if ans {
                break;
            }

            if s[i] == b'+' && s[i + 1] == b'+' {
                s[i] = b'-';
                s[i + 1] = b'-';
                ans |= !Self::dfs(s, mp, n);
                s[i] = b'+';
                s[i + 1] = b'+';
            }
        }

        mp.insert(s.to_vec(), ans);

        ans
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::can_win("++++".into()), true);
    assert_eq!(Solution::can_win("+".into()), false);
}
