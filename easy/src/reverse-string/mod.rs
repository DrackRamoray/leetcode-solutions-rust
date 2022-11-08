pub struct Solution;

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        if s.len() == 0 {
            return;
        }

        let mut l: usize = 0;
        let mut r: usize = s.len() - 1;

        while l < r {
            s.swap(l, r);
            l += 1;
            r -= 1;
        }
    }
}

#[test]
fn it_works() {
    let mut s = vec!['h','e','l','l','o'];
    let ans = ['o','l','l','e','h'];
    Solution::reverse_string(&mut s);
    assert_eq!(s, ans);

    let mut s = vec!['H','a','n','n','a','h'];
    let ans = ['h','a','n','n','a','H'];
    Solution::reverse_string(&mut s);
    assert_eq!(s, ans);
}
