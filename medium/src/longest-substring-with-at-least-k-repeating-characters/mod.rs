pub struct Solution;

impl Solution {
    pub fn longest_substring(s: String, k: i32) -> i32 {
        Self::helper(s.as_bytes(), 0, s.len(), k as usize)
    }

    fn helper(s: &[u8], begin: usize, end: usize, k: usize) -> i32 {
        if end < k {
            return 0;
        }

        let mut cnt = vec![0;26];

        for i in begin..end {
            cnt[s[i] as usize - 'a' as usize] += 1;
        }

        for i in begin..end {
            if cnt[s[i] as usize - 'a' as usize] >= k {
                continue;
            }

            let mut j = i + 1;

            while j < end && cnt[s[j] as usize - 'a' as usize] < k {
                j += 1;
            }

            return (Self::helper(s, begin, i, k)).max(Self::helper(s, j, end, k));
        }

        end as i32 - begin as i32
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::longest_substring("aaabb".to_string(), 3), 3);
    assert_eq!(Solution::longest_substring("ababbc".to_string(), 2), 5);
}
