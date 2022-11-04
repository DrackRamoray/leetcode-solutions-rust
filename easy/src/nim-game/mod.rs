pub struct Solution;

impl Solution {
    pub fn can_win_nim(n: i32) -> bool {
        n % 4 != 0
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::can_win_nim(4), false);
    assert_eq!(Solution::can_win_nim(1), true);
    assert_eq!(Solution::can_win_nim(2), true);
}
