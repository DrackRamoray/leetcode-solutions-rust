struct Solution;

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut ans = vec![];

        Self::dfs(&candidates, &mut ans, &mut vec![], target, 0);

        ans
    }

    fn dfs(candidates: &Vec<i32>, ans: &mut Vec<Vec<i32>>, selected: &mut Vec<i32>, target: i32, cur: usize) {
        if target == 0 {
            ans.push(selected.to_vec());
            return;
        }

        if cur == candidates.len() {
            return;
        }

        Self::dfs(candidates, ans, selected, target, cur + 1);

        if target >= candidates[cur] {
            selected.push(candidates[cur]);
            Self::dfs(candidates, ans, selected, target - candidates[cur], cur);
            selected.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::combination_sum(vec![2,3,6,7], 7), vec![vec![7], vec![2,2,3]]);
        assert_eq!(Solution::combination_sum(vec![2,3,5], 8), vec![vec![3,5], vec![2,3,3], vec![2,2,2,2]]);
        assert_eq!(Solution::combination_sum(vec![2], 1), vec![] as Vec<Vec<i32>>);
    }
}
