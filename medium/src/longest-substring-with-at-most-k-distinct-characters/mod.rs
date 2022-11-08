pub struct Solution;

impl Solution {
    pub fn length_of_longest_substring_k_distinct(s: String, k: i32) -> i32 {
        let k = k as usize;
        let n = s.len();

        if n == 0 || k == 0 {
            return 0;
        }

        let s = s.as_bytes();
        let mut i = 0;
        let mut j = 0;
        let mut m = std::collections::HashMap::new();
        let mut ans = 1;

        while j < s.len() {
            m.insert(s[j], j);
            j += 1;

            if m.len() == k + 1 {
                if let Some(&idx) = m.values().min() {
                    m.remove(&s[idx]);
                    i = idx + 1;
                }
            }

            ans = ans.max(j - i);
        }

        ans as i32
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::length_of_longest_substring_k_distinct("eceba".to_string(), 2), 3);
    assert_eq!(Solution::length_of_longest_substring_k_distinct("aa".to_string(), 1), 2);
}
