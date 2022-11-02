pub struct Solution;

impl Solution {
    pub fn hamming_weight (mut n: u32) -> i32 {
        let mut ans = 0;

        while n != 0 {
            ans += 1;
            n = n & (n - 1);
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::hamming_weight(0b00000000000000000000000000001011), 3);
        assert_eq!(Solution::hamming_weight(0b00000000000000000000000010000000), 1);
        assert_eq!(Solution::hamming_weight(0b11111111111111111111111111111101), 31);
    }
}
