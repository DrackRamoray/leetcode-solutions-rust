pub struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut p = 1;
        let mut ans = vec![1;n];

        for i in 0..n {
            ans[i] = p;
            p = p * nums[i];
        }

        let mut s = 1;

        for i in (1..n).rev() {
            s = s * nums[i];
            ans[i-1] = ans[i-1] * s;
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::product_except_self(vec![1,2,3,4]), vec![24,12,8,6]);
        assert_eq!(Solution::product_except_self(vec![-1,1,0,-3,3]), vec![0,0,9,0,0]);
    }
}
