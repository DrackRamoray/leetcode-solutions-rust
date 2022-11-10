pub struct Solution;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        if ransom_note.len() > magazine.len() {
            return false;
        }

        let mut cnt = vec![0;26];

        for ch in magazine.chars() {
            cnt[ch as usize - 97] += 1;
        }

        for ch in ransom_note.chars() {
            cnt[ch as usize - 97] -= 1;

            if cnt[ch as usize - 97] < 0 {
                return false;
            }
        }

        true
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::can_construct("a".to_string(), "b".to_string()), false);
    assert_eq!(Solution::can_construct("aa".to_string(), "ab".to_string()), false);
    assert_eq!(Solution::can_construct("aa".to_string(), "aab".to_string()), true);
}
