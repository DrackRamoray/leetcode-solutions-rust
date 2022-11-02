pub struct Solution;

impl Solution {
    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = vec![];

        nums.sort();

        Self::dfs(&nums, &mut ans, &mut vec![], 0, false);

        ans
    }

    fn dfs(nums: &Vec<i32>, ans: &mut Vec<Vec<i32>>, tmp: &mut Vec<i32>, cur: usize, choose_pre: bool) {
        if cur == nums.len() {
            ans.push(tmp.to_vec());
            return;
        }

        Self::dfs(nums, ans, tmp, cur + 1, false);

        if !choose_pre && cur > 0 && nums[cur-1] == nums[cur] {
            return;
        }

        tmp.push(nums[cur]);

        Self::dfs(nums, ans, tmp, cur + 1, true);

        tmp.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::subsets_with_dup(vec![1,2,2]), vec![vec![],vec![2],vec![2,2],vec![1],vec![1,2],vec![1,2,2]]);
        assert_eq!(Solution::subsets_with_dup(vec![0]), vec![vec![], vec![0]]);
    }
}
