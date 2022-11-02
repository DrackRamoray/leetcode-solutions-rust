struct Solution;

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut s = n;
        let mut f = Self::count_sum(n);

        while f != 1 && s != f {
            s = Self::count_sum(s);
            f = Self::count_sum(Self::count_sum(f));
        }

        f == 1
    }

    fn count_sum(mut n: i32) -> i32 {
        let mut sum = 0;

        while n > 0 {
            sum += (n % 10).pow(2);
            n = n / 10;
        }

        sum
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::is_happy(19), true);
        assert_eq!(Solution::is_happy(2), false);
    }
}
