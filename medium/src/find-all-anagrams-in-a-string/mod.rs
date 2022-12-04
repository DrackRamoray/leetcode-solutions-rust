pub struct Solution;

impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let n = s.len();
        let m = p.len();
        let ss = s.as_bytes();
        let pp = p.as_bytes();
        let mut cnt_s = vec![0; 26];
        let mut cnt_p = vec![0; 26];
        let mut ans = vec![];

        for i in 0..m {
            cnt_p[pp[i] as usize - 97] += 1;
        }

        for i in 0..n {
            cnt_s[ss[i] as usize - 97] += 1;

            if i >= m {
                cnt_s[ss[i - m] as usize - 97] -= 1;
            }

            if i >= m - 1 && cnt_s == cnt_p {
                ans.push(i as i32 - m as i32 + 1);
            }
        }

        ans
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::find_anagrams("cbaebabacd".to_string(), "abc".to_string()), vec![0,6]);
    assert_eq!(Solution::find_anagrams("abab".to_string(), "ab".to_string()), vec![0,1,2]);
}
