struct Solution;

impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        (0..n as i64).fold(1, |c, i| c * 2 * (2 * i + 1) / (i + 2)) as i32
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(super::Solution::num_trees(3), 5);
        assert_eq!(super::Solution::num_trees(1), 1);
    }
}
