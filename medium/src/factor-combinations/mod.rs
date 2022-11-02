pub struct Solution;

impl Solution {
    pub fn get_factors(n: i32) -> Vec<Vec<i32>> {
        Self::dfs(n, 2)
    }

    fn dfs(n: i32, begin: i32) -> Vec<Vec<i32>> {
        let mut ans = vec![];
        let mut i = begin;

        while i * i <= n {
            if n % i == 0 {
                ans.push(vec![n / i, i]);

                for s in Self::dfs(n / i, i).iter_mut() {
                    s.push(i);
                    ans.push(s.to_vec());
                }
            }
            i += 1;
        }

        ans
    }
}

#[test]
fn test() {
    assert_eq!(Solution::get_factors(1), vec![] as Vec<Vec<i32>>);
    assert_eq!(Solution::get_factors(37), vec![] as Vec<Vec<i32>>);
    assert_eq!(Solution::get_factors(12), vec![vec![6,2],vec![3,2,2],vec![4,3]]);
    assert_eq!(Solution::get_factors(32), vec![vec![16,2],vec![8,2,2],vec![4,2,2,2],vec![2,2,2,2,2],vec![4,4,2],vec![8,4]]);
}
