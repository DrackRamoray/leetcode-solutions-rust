pub struct Solution;

impl Solution {
    pub fn is_power_of_four(n: i32) -> bool {
        n.is_positive() && (n & (n - 1) == 0) && (n & 0x2aaaaaaa == 0)
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::is_power_of_four(16), true);
    assert_eq!(Solution::is_power_of_four(5), false);
    assert_eq!(Solution::is_power_of_four(1), true);
}
