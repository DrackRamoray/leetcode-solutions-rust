pub struct Solution;

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let mut cnt = vec![0;26];
        let ss = s.as_bytes();
        let n = ss.len();
        let mut left = 0;
        let mut right = 0;
        let mut ans = 0;

        while right < n {
            let index = ss[right] as usize - b'A' as usize;
            cnt[index] += 1;
            ans = ans.max(cnt[index]);

            if right as i32 - left as i32 + 1 - ans > k {
                cnt[ss[left] as usize - b'A' as usize] -= 1;
                left += 1;
            }

            right += 1;
        }

        (right - left) as i32
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::character_replacement("ABAB".to_string(), 2), 4);
    assert_eq!(Solution::character_replacement("AABABBA".to_string(), 1), 4);
}
