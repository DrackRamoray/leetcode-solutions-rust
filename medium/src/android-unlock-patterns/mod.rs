pub struct Solution;

impl Solution {
    pub fn number_of_patterns(m: i32, n: i32) -> i32 {
        let mut ans = 0;
        let mut used = vec![vec![false;3];3];

        Self::dfs(&mut used, &mut ans, m as usize, n as usize, 0, None);

        ans
    }

    fn dfs(used: &mut Vec<Vec<bool>>, ans: &mut i32, m: usize, n: usize, begin: usize, last: Option<(usize, usize)>) {
        if begin >= m {
            *ans += 1;
        }

        if begin == n {
            return;
        }

        if let Some((x, y)) = last {
            for i in 0..3 {
                for j in 0..3 {
                    if !used[i][j] {
                        if ((i == x && j+y == 2) || (j == y && i+x == 2) || (i+x == 2 && j+y == 2)) && !used[(i+x)/2][(j+y)/2] {
                            continue;
                        }

                        used[i][j] = true;
                        Self::dfs(used, ans, m, n, begin + 1, Some((i, j)));
                        used[i][j] = false;
                    }
                }
            }
        } else {
            for i in 0..3 {
                for j in 0..3 {
                    used[i][j] = true;
                    Self::dfs(used, ans, m, n, begin + 1, Some((i, j)));
                    used[i][j] = false;
                }
            }
        }
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::number_of_patterns(1, 1), 9);
    assert_eq!(Solution::number_of_patterns(1, 2), 65);
}
