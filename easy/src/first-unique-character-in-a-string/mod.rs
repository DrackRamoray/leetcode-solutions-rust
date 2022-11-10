pub struct Solution;

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let ss = s.as_bytes();
        let cnt = ss.iter().fold(vec![0;26], |mut acc, &b| {
            acc[b as usize - b'a' as usize] += 1;
            acc
        });

        for i in 0..s.len() {
            if cnt[ss[i] as usize - b'a' as usize] == 1 {
                return i as i32;
            }
        }

        -1
    }
}


#[test]
fn it_works() {
    assert_eq!(Solution::first_uniq_char("leetcode".to_string()), 0);
    assert_eq!(Solution::first_uniq_char("loveleetcode".to_string()), 2);
    assert_eq!(Solution::first_uniq_char("aabb".to_string()), -1);
}
