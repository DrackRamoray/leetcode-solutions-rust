pub struct Solution;

impl Solution {
    pub fn nth_super_ugly_number(n: i32, primes: Vec<i32>) -> i32 {
        let mut dp = vec![1];
        let mut pointers = vec![0;primes.len()];

        for _ in 1..n {
            let mut tmp = primes[0] as i64 * dp[pointers[0]];

            for i in 1..pointers.len() {
                tmp = tmp.min(primes[i] as i64 * dp[pointers[i]]);
            }

            dp.push(tmp);

            for i in 0..pointers.len() {
                if primes[i] as i64 * dp[pointers[i]] == tmp {
                    pointers[i] += 1;
                }
            }
        }

        dp[n as usize - 1] as i32
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::nth_super_ugly_number(12, vec![2,7,13,19]), 32);
    assert_eq!(Solution::nth_super_ugly_number(1, vec![2,3,5]), 1);
}
