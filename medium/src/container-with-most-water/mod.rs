pub struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let n = height.len();
        let mut lo = 0;
        let mut hi = n - 1;
        let mut ans = 0;

        while lo < hi && hi < n {
            ans = ans.max(height[lo].min(height[hi]) * (hi as i32 - lo as i32));

            if height[lo] < height[hi] {
                lo += 1;
            } else {
                hi -= 1;
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
        assert_eq!(Solution::max_area(vec![1,8,6,2,5,4,8,3,7]), 49);
        assert_eq!(Solution::max_area(vec![1,1]), 1);
    }
}
