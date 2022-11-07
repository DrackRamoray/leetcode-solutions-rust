pub struct Solution;

impl Solution {
    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        let n = matrix.len();
        let m = matrix[0].len();
        let mut visited = vec![vec![0;m];n];
        let mut ans = 0;

        for i in 0..n {
            for j in 0..m {
                ans = ans.max(Self::search(&matrix, &mut visited, i, j));
            }
        }

        ans as i32
    }

    fn search (matrix: &Vec<Vec<i32>>, visited: &mut Vec<Vec<usize>>, i: usize, j: usize) -> usize {
        if visited[i][j] != 0 {
            return visited[i][j];
        }

        let n = matrix.len();
        let m = matrix[0].len();

        visited[i][j] += 1;

        if i > 0 && matrix[i-1][j] > matrix[i][j] {
            visited[i][j] = visited[i][j].max(Self::search(matrix, visited, i-1, j)+1);
        }

        if i < n - 1 && matrix[i+1][j] > matrix[i][j] {
            visited[i][j] = visited[i][j].max(Self::search(matrix, visited, i+1, j)+1);
        }

        if j > 0 && matrix[i][j-1] > matrix[i][j] {
            visited[i][j] = visited[i][j].max(Self::search(matrix, visited, i, j-1)+1);
        }

        if j < m - 1 && matrix[i][j+1] > matrix[i][j] {
            visited[i][j] = visited[i][j].max(Self::search(matrix, visited, i, j+1)+1);
        }


        return visited[i][j];
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::longest_increasing_path(vec![vec![9,9,4],vec![6,6,8],vec![2,1,1]]), 4);
    assert_eq!(Solution::longest_increasing_path(vec![vec![3,4,5],vec![3,2,6],vec![2,2,1]]), 4);
    assert_eq!(Solution::longest_increasing_path(vec![vec![1]]), 1);
}
