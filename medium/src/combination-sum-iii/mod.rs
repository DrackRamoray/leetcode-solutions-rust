struct Solution;

impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        let mut selection = Vec::new();

        Self::helper(1, k, n, &mut selection, &mut ans);

        ans
    }

    fn helper (cur: i32, k: i32, n: i32, selection: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
        if k == 0 && n == 0 {
            ans.push(selection.clone());
            return;
        }

        if cur > 9 || k < 0 || n < 0 {
            return;
        }

        Self::helper(cur + 1, k, n, selection, ans);
        selection.push(cur);
        Self::helper(cur + 1, k - 1, n - cur, selection, ans);
        selection.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::combination_sum3(3, 7), vec![vec![1,2,4]]);
        assert_eq!(Solution::combination_sum3(3,9), vec![vec![2,3,4],vec![1,3,5],vec![1,2,6]]);
        assert_eq!(Solution::combination_sum3(4,1), vec![] as Vec<Vec<i32>>);
    }
}
