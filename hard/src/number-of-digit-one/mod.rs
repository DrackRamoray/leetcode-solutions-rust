pub struct Solution;

impl Solution {
    pub fn count_digit_one(n: i32) -> i32 {
        let mut count = 0;
        let mut high = n / 10;
        let mut cur = n % 10;
        let mut low = 0;
        let mut digit = 1;

        while high != 0 || cur != 0 {
            match cur {
                0 => count += high * digit,
                1 => count += high * digit + low + 1,
                _ => count += (high + 1) * digit
            };

            low += cur * digit;
            cur = high % 10;
            high /= 10;
            digit *= 10;
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::count_digit_one(13), 6);
        assert_eq!(Solution::count_digit_one(0), 0);
    }
}
