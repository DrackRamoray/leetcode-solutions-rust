struct Solution;

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();

        Self::dfs(n as usize, k as usize, &mut ans, &mut Vec::new(), 1);

        ans
    }

    fn dfs(n: usize, k: usize, ans: &mut Vec<Vec<i32>>, selected: &mut Vec<i32>, cur: usize) {
        if selected.len() + n.wrapping_sub(cur).wrapping_add(1) < k {
            return;
        }

        if selected.len() == k {
            ans.push(selected.to_vec());
            return;
        }

        for i in cur..=n {
            selected.push(i as i32);
            Self::dfs(n, k, ans, selected, i + 1);
            selected.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::combine(4, 2), vec![vec![1,2],vec![1,3],vec![1,4],vec![2,3],vec![2,4],vec![3,4]]);
        assert_eq!(Solution::combine(1, 1), vec![vec![1]]);
    }
}
