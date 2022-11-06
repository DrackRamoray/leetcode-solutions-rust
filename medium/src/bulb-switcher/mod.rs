pub struct Solution;

impl Solution {
    pub fn bulb_switch(n: i32) -> i32 {
        (n as f64).sqrt() as i32
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::bulb_switch(3), 1);
    assert_eq!(Solution::bulb_switch(1), 1);
    assert_eq!(Solution::bulb_switch(0), 0);
}
