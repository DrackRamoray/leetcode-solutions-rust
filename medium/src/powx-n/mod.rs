pub struct Solution;

impl Solution {
    pub fn my_pow(mut x: f64, n: i32) -> f64 {
        if n == 0 {
            return 1_f64;
        }

        let mut exp = n.abs() as u32;
        let mut ans = 1_f64;

        while exp > 0 {
            if exp % 2 == 1 {
                ans *= x as f64;
            }

            x *= x;
            exp /= 2;
        }

        if n < 0 {
            return 1_f64 / ans;
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(super::Solution::my_pow(2.00000, 10), 1024.00000);
        assert_eq!(super::Solution::my_pow(2.10000, 3), 9.261000000000001);
        assert_eq!(super::Solution::my_pow(2.00000, -2), 0.25000);
    }
}
