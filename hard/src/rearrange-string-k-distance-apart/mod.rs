use std::collections::BinaryHeap;
use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn rearrange_string(s: String, k: i32) -> String {
        if k == 0 {
            return s;
        }

        let mut cnt = vec![0;26];
        let mut queue = BinaryHeap::new();
        let mut remain = VecDeque::new();

        for &u in s.as_bytes() {
            cnt[u as usize - 'a' as usize] += 1;
        }

        for i in 0..cnt.len() {
            if cnt[i] > 0 {
                queue.push((cnt[i], (i as u8 + b'a')));
            }
        }

        let mut ans = String::new();

        while let Some((v, u)) = queue.pop() {
            ans.push(u as char);
            remain.push_back((v-1, u));

            if remain.len() == k as usize {
                if let Some(&(t, k)) = remain.front() {
                    if t > 0 {
                        queue.push((t, k));
                    }
                }
                remain.pop_front();
            }
        }

        if ans.len() < s.len() {
            "".to_string()
        } else {
            ans
        }
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::rearrange_string("aabbcc".to_string(), 3), "cbacba".to_string());
    assert_eq!(Solution::rearrange_string("aaabc".to_string(), 3), "".to_string());
    assert_eq!(Solution::rearrange_string("aaadbbcc".to_string(), 2), "acbadcba".to_string());
}
