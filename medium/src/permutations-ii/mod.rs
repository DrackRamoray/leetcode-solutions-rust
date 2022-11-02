use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = vec![];

        Self::dfs(&mut nums, &mut ans, 0);

        ans
    }

    fn dfs(nums: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>, begin: usize) {
        if begin == nums.len() {
            ans.push(nums.to_vec());
            return;
        }

        let mut dup = HashSet::new();

        for i in begin..nums.len() {
            if dup.contains(&nums[i]) {
                continue;
            }

            dup.insert(nums[i]);

            nums.swap(i, begin);

            Self::dfs(nums, ans, begin + 1);

            nums.swap(i, begin);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(super::Solution::permute_unique(vec![1,1,2]), vec![vec![1,1,2], vec![1,2,1], vec![2,1,1]]);
        assert_eq!(super::Solution::permute_unique(vec![1,2,3]), vec![vec![1,2,3],vec![1,3,2],vec![2,1,3],vec![2,3,1],vec![3,2,1],vec![3,1,2]]);
    }
}
