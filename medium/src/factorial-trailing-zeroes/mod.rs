struct Solution;

impl Solution {
    pub fn trailing_zeroes(n: i32) -> i32 {
        let mut res = 0;
        let mut n = n;

        while n != 0 {
            n = n / 5;
            res += n;
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::trailing_zeroes(3), 0);
        assert_eq!(Solution::trailing_zeroes(5), 1);
        assert_eq!(Solution::trailing_zeroes(0), 0);
    }
}
