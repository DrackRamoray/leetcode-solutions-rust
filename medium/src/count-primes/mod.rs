struct Solution;

impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        let mut count = 0;

        for i in 2..n {
            count += Solution::is_prime(i);
        }

        count
    }

    fn is_prime(n: i32) -> i32 {
        let mut i = 2;

        while i * i <= n {
            if n % i == 0 {
                return 0;
            }

            i += 1;
        }

        1
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::count_primes(10), 4);
        assert_eq!(Solution::count_primes(0), 0);
        assert_eq!(Solution::count_primes(1), 0);
    }
}
