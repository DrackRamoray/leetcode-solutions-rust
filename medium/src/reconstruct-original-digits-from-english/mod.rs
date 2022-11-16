pub struct Solution;

impl Solution {
    pub fn original_digits(s: String) -> String {
        let mut mp = std::collections::HashMap::new();

        for c in s.chars() {
            *mp.entry(c).or_insert(0) += 1;
        }

        let mut cnt = [0;10];
        cnt[0] = *mp.get(&'z').unwrap_or(&0);
        cnt[2] = *mp.get(&'w').unwrap_or(&0);
        cnt[4] = *mp.get(&'u').unwrap_or(&0);
        cnt[6] = *mp.get(&'x').unwrap_or(&0);
        cnt[8] = *mp.get(&'g').unwrap_or(&0);
        cnt[3] = *mp.get(&'h').unwrap_or(&0) - cnt[8];
        cnt[5] = *mp.get(&'f').unwrap_or(&0) - cnt[4];
        cnt[7] = *mp.get(&'s').unwrap_or(&0) - cnt[6];
        cnt[1] = *mp.get(&'o').unwrap_or(&0) - cnt[0] - cnt[2] - cnt[4];
        cnt[9] = *mp.get(&'i').unwrap_or(&0) - cnt[5] - cnt[6] - cnt[8];

        let mut ans = String::new();

        for i in 0..10 {
            for _ in 0..cnt[i] {
                ans.push((i as u8 + '0' as u8) as char);
            }
        }

        ans
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::original_digits("owoztneoer".to_string()), "012".to_string());
    assert_eq!(Solution::original_digits("fviefuro".to_string()), "45".to_string());
}
