struct Solution;

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let row_index = row_index as usize;
        let mut ans = vec![0;row_index + 1];
        ans[0] = 1;

        for i in 1..=row_index {
            ans[i as usize] = ((ans[i-1] as u64) * ((row_index - i + 1) as u64) / (i as u64)) as i32;
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::get_row(3), vec![1,3,3,1]);
        assert_eq!(Solution::get_row(0), vec![1]);
        assert_eq!(Solution::get_row(1), vec![1,1]);
    }
}
