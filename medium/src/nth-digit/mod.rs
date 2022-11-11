pub struct Solution;

impl Solution {
    pub fn find_nth_digit(n: i32) -> i32 {
        let mut start = 1;
        let mut digit = 1;
        let mut count = 9;
        let mut mn = n as i64;

        while mn > count {
            mn = mn - count;
            digit =  digit + 1;
            start = start * 10;
            count = 9 * start * digit;
        }

        let num = start + (mn - 1) / digit;
        let index = (mn - 1) % digit;
        let s = num.to_string().chars().nth(index as usize).unwrap();

        return s.to_digit(10).unwrap() as i32;
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::find_nth_digit(3), 3);
    assert_eq!(Solution::find_nth_digit(11), 0);
}
