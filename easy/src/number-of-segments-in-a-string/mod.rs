pub struct Solution;

impl Solution {
    pub fn count_segments(s: String) -> i32 {
        s.split_whitespace().count() as i32
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::count_segments("Hello, my name is John".to_string()), 5);
}
