pub struct Solution;

impl Solution {
    pub fn count_numbers_with_unique_digits(n: i32) -> i32 {
        match n {
            0 => 1,
            _ => Self::count_numbers_with_unique_digits(n-1) + 9 * (11-n..10).product::<i32>(),
        }
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::count_numbers_with_unique_digits(2), 91);
    assert_eq!(Solution::count_numbers_with_unique_digits(0), 1);
}
