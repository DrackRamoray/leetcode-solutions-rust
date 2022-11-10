pub struct Solution;

impl Solution {
    pub fn last_remaining(n: i32) -> i32 {
        if n == 1 {
            return 1;
        }

        2 * ( 1 + n / 2 - Self::last_remaining(n / 2))
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::last_remaining(9), 6);
    assert_eq!(Solution::last_remaining(1), 1);
}
