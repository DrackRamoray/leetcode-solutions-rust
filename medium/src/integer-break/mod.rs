pub struct Solution;

impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        if n <= 3 {
            return n - 1;
        }

        let q = n as u32 / 3;
        let r = n % 3;

        if r == 0 {
            return 3_i32.pow(q);
        }

        if r == 1 {
            return 3_i32.pow(q - 1) * 4;
        }

        return 3_i32.pow(q) * 2
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::integer_break(2), 1);
    assert_eq!(Solution::integer_break(10), 36);
}
