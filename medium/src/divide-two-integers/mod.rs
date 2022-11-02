pub struct Solution;

impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        let is_neg = (dividend > 0) != (divisor > 0);
        let mut dividend = (dividend as i64).abs();
        let mut divisor = (divisor as i64).abs();
        let mut ans = 0;
        let mut cnt = 1;

        while (divisor << 1) <= dividend {
            cnt <<= 1;
            divisor <<= 1;
        }

        while cnt > 0 {
            if dividend >= divisor {
                ans += cnt;
                dividend -= divisor;
            }

            divisor >>= 1;
            cnt >>= 1;
        }

        if is_neg {
            -ans as i32
        } else {
            ans.min(i32::MAX as i64) as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::divide(10, 3), 3);
        assert_eq!(Solution::divide(7, -3), -2);
    }
}
