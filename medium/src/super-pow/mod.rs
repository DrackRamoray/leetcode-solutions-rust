pub struct Solution;

impl Solution {
    pub fn super_pow(a: i32, b: Vec<i32>) -> i32 {
        let mut ans = 1;

        for v in b {
            ans = Self::power(ans, 10) * Self::power(a as i64, v as i64) % 1337;
        }

        ans as i32
    }

    fn power(mut x: i64, mut n: i64) -> i64 {
        let mut ans = 1;

        while n > 0 {
            if n % 2 == 1 {
                ans = ans * x % 1337;
            }

            x = x * x % 1337;
            n /= 2;
        }

        ans
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::super_pow(2, vec![3]), 8);
    assert_eq!(Solution::super_pow(2, vec![1,0]), 1024);
    assert_eq!(Solution::super_pow(1, vec![4,3,3,8,5,2]), 1);
    assert_eq!(Solution::super_pow(2147483647, vec![2,0,0]), 1198);
}
