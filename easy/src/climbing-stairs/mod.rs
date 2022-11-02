struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut a = 0;
        let mut b = 1;
        let mut ans = 0;

        for _ in 1..=n {
            ans = a + b;
            a = b;
            b = ans;
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(super::Solution::climb_stairs(2), 2);
        assert_eq!(super::Solution::climb_stairs(3), 3);
    }
}
