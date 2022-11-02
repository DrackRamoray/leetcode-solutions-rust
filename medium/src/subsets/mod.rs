pub struct Solution;

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = vec![];

        Self::dfs(0, &mut vec![], &mut ans, &nums);

        ans
    }

    fn dfs(cur: usize, tmp: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>, nums: &Vec<i32>) {
        if cur == nums.len() {
            ans.push(tmp.to_vec());
            return;
        }

        tmp.push(nums[cur]);
        Self::dfs(cur + 1, tmp, ans, nums);
        tmp.pop();
        Self::dfs(cur + 1, tmp, ans, nums);
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::subsets(vec![1,2,3]), vec![vec![1,2,3],vec![1,2],vec![1,3],vec![1],vec![2,3],vec![2],vec![3],vec![]]);
        assert_eq!(Solution::subsets(vec![0]), vec![vec![0],vec![]]);
    }
}
