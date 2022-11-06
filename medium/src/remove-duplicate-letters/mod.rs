pub struct Solution;

impl Solution {
    pub fn remove_duplicate_letters(s: String) -> String {
        let mut stack = vec![];
        let mut mp = vec![0;26];

        for b in s.bytes() {
            mp[(b - b'a') as usize] += 1;
        }

        let mut vis = vec![false;26];

        for b in s.bytes() {
            let u = (b - b'a') as usize;
            mp[u] -= 1;

            if !vis[u] {
                vis[u] = true;

                while let Some(&v) = stack.last() {
                    let u = (v - b'a') as usize;
                    if v > b && mp[u] > 0 {
                        vis[u] = false;
                        stack.pop();
                    } else {
                        break;
                    }
                }

                stack.push(b);
            }
        }

        stack.into_iter().map(|u| u as char).collect()
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::remove_duplicate_letters("bcabc".to_string()), "abc".to_string());
    assert_eq!(Solution::remove_duplicate_letters("cbacdcbc".to_string()), "acdb".to_string());
}
