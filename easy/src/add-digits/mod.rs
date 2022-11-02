pub struct Solution;

impl Solution {
    pub fn add_digits(num: i32) -> i32 {
        let mut sum = num;

        while sum >= 10 {
            let n1 = sum / 10;
            let n2 = sum % 10;
            sum = n1 + n2
        }

        sum
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::add_digits(38), 2);
    assert_eq!(Solution::add_digits(0), 0);
}
