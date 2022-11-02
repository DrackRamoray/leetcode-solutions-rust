struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut num = x;
        let mut ans = 0;

        while num != 0 {
            let rest = num % 10;

            if ans > i32::MAX / 10 || (ans == i32::MAX / 10 && rest > 7) {
                return 0;
            }

            if ans < i32::MIN / 10 || (ans == i32::MIN / 10 && rest < -8) {
                return 0;
            }

            ans = ans * 10 + rest;
            num = num / 10;
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::reverse(123), 321);
        assert_eq!(Solution::reverse(-123), -321);
        assert_eq!(Solution::reverse(120), 21);
        assert_eq!(Solution::reverse(0), 0);
    }
}
