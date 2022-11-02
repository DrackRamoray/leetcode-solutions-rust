struct Solution;

impl Solution {
    pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = vec![];

        Self::dfs(&mut nums, &mut ans, 0);

        ans
    }

    fn dfs(nums: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>, begin: usize) {
        if begin == nums.len() {
            ans.push(nums.to_vec());
            return;
        }

        for i in begin..nums.len() {
            nums.swap(i, begin);
            Self::dfs(nums, ans, begin + 1);
            nums.swap(i, begin);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::permute(vec![1,2,3]), vec![vec![1,2,3],vec![1,3,2],vec![2,1,3],vec![2,3,1],vec![3,2,1],vec![3,1,2]]);
        assert_eq!(Solution::permute(vec![0,1]), vec![vec![0,1], vec![1,0]]);
        assert_eq!(Solution::permute(vec![1]), vec![vec![1]]);
    }
}
