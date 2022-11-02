struct Solution;

impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        return Self::solve(n, 0, 0, 0, 0);
    }

    fn solve(n: i32, row: i32, cols: i32, left: i32, right: i32) -> i32 {
        if n == row {
            return 1;
        }

        let mut count = 0;
        let mut pos = ((1 << n) - 1) & (!(cols | left | right));

        while pos != 0 {
            let p = pos & (-pos);
            pos = pos & (pos - 1);
            count += Self::solve(n, row + 1, cols | p, (left | p) << 1, (right | p) >> 1);
        }

        count
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(super::Solution::total_n_queens(4), 2);
        assert_eq!(super::Solution::total_n_queens(1), 1);
    }
}
