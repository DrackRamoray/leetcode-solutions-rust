pub struct Solution;

impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = heights.len();
        let m = heights[0].len();

        let mut pacific = vec![vec![false;m];n];

        for i in 0..n {
            Self::dfs(&heights, &mut pacific, i, 0);
        }

        for j in 1..m {
            Self::dfs(&heights, &mut pacific, 0, j);
        }

        let mut atlantic = vec![vec![false;m];n];

        for i in 0..n {
            Self::dfs(&heights, &mut atlantic, i, m - 1);
        }

        for j in 0..m-1 {
            Self::dfs(&heights, &mut atlantic, n - 1, j);
        }

        let mut ans = vec![];

        for i in 0..n {
            for j in 0..m {
                if pacific[i][j] && atlantic[i][j] {
                    ans.push(vec![i as i32, j as i32]);
                }
            }
        }

        ans
    }

    fn dfs(heights: &Vec<Vec<i32>>, ocean: &mut Vec<Vec<bool>>, i: usize, j: usize) {
        if ocean[i][j] {
            return;
        }

        let n = heights.len();
        let m = heights[0].len();

        ocean[i][j] = true;

        if i > 0 && heights[i-1][j] >= heights[i][j] {
            Self::dfs(heights, ocean, i-1, j);
        }

        if i < n-1 && heights[i+1][j] >= heights[i][j] {
            Self::dfs(heights, ocean, i+1, j);
        }

        if j > 0 && heights[i][j-1] >= heights[i][j] {
            Self::dfs(heights, ocean, i, j-1);
        }

        if j < m-1 && heights[i][j+1] >= heights[i][j] {
            Self::dfs(heights, ocean, i, j+1);
        }
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::pacific_atlantic(vec![vec![1,2,2,3,5],vec![3,2,3,4,4],vec![2,4,5,3,1],vec![6,7,1,4,5],vec![5,1,1,2,4]]), vec![vec![0,4],vec![1,3],vec![1,4],vec![2,2],vec![3,0],vec![3,1],vec![4,0]]);
    assert_eq!(Solution::pacific_atlantic(vec![vec![2,1],vec![1,2]]), vec![vec![0,0],vec![0,1],vec![1,0],vec![1,1]]);
}
