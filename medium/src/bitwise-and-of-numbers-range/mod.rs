pub struct Solution;

impl Solution {
    pub fn range_bitwise_and(left: i32, mut right: i32) -> i32 {
        while left < right {
            right = right & (right - 1);
        }

        right
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::range_bitwise_and(5, 7), 4);
        assert_eq!(Solution::range_bitwise_and(0, 0), 0);
        assert_eq!(Solution::range_bitwise_and(1, 2147483647), 0);
    }
}
