pub struct Solution;

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut solutions: Vec<Vec<String>> = Vec::new();
        let mut queens = vec![-1;n as usize];

        Self::solve(&mut solutions, &mut queens, n, 0, 0, 0, 0);

        return solutions;
    }

    fn solve(solutions: &mut Vec<Vec<String>>, queens: &mut Vec<i32>, n: i32, row: i32, cols: i32, left: i32, right: i32) {
        if row == n {
            solutions.push(Self::make_solution(queens, n));
            return;
        }

        let mut positions = ((1 << n) - 1) & (!(cols | left | right));

        while positions != 0 {
            let pos = positions & (-positions);
            positions = positions & (positions - 1);
            let col = Self::count_bits(pos - 1);
            queens[row as usize] = col;
            Self::solve(solutions, queens, n, row + 1, cols | pos, (left | pos) << 1, (right | pos) >> 1);
            queens[row as usize] = -1;
        }
    }

    fn make_solution(queens: &Vec<i32>, n: i32) -> Vec<String> {
        let mut solution = Vec::new();

        for i in 0..n as usize {
            let mut row = vec![".".to_string();n as usize];
            row[queens[i] as usize] = "Q".to_string();
            solution.push(row.join(""));
        }

        solution
    }

    fn count_bits(mut n: i32) -> i32 {
        let mut count = 0;

        while n > 0 {
            n = n & (n - 1);
            count += 1;
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::solve_n_queens(4), vec![vec![".Q..".to_owned(),"...Q".to_owned(),"Q...".to_owned(),"..Q.".to_owned()],vec!["..Q.".to_owned(),"Q...".to_owned(),"...Q".to_owned(),".Q..".to_owned()]]);
        assert_eq!(Solution::solve_n_queens(1), vec![vec!["Q".to_owned()]])
    }
}
