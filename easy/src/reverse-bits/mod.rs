struct Solution;

impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
        let mut ans = 0;

        for i in 0..32 {
            let bit = x & 1 << i;
            let n = (31 - i as i32) - i as i32;

            if n > 0 {
                ans |= bit << n;
            } else {
                ans |= bit >> -n;
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        let n = 0b00000010100101000001111010011100;
        let ans = 964176192;
        assert_eq!(Solution::reverse_bits(n), ans);
        let n = 0b11111111111111111111111111111101;
        let ans = 3221225471;
        assert_eq!(Solution::reverse_bits(n), ans);
    }
}
