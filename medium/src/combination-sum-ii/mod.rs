pub struct Solution;

impl Solution {
    fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut ans = vec![];

        let n = candidates.len();
        candidates.sort();

        Self::dfs(&candidates, &mut Vec::new(), &mut ans, target, 0, n);

        ans
    }

    fn dfs(candidates: &[i32], selected: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>, target: i32, cur: usize, n: usize) {
        if target == 0 {
            ans.push(selected.to_vec());
            return;
        }

        if target > 0 {
            for i in cur..n {
                if i > cur && candidates[i] == candidates[i - 1] {
                    continue;
                }

                selected.push(candidates[i]);
                Self::dfs(candidates, selected, ans, target - candidates[i], i + 1, n);
                selected.pop();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::combination_sum2(vec![10,1,2,7,6,1,5], 8), vec![vec![1,1,6], vec![1,2,5], vec![1,7], vec![2,6]]);
        assert_eq!(Solution::combination_sum2(vec![2,5,2,1,2], 5), vec![vec![1,2,2], vec![5]]);
    }
}
