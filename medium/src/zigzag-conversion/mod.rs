struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let n = s.len();
        let mut ans = vec!["".to_owned();n];

        let mut index: i32 = 0;
        let mut direction: i32 = 1;

        for c in s.chars() {
            ans[index as usize].push(c);

            if index == 0 {
                direction = 1;
            } else if index == (num_rows - 1) {
                direction = -1;
            }

            index = index + direction
        }

        ans.join("")
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::convert("PAYPALISHIRING".to_owned(), 3), "PAHNAPLSIIGYIR".to_owned());
        assert_eq!(Solution::convert("PAYPALISHIRING".to_owned(), 4), "PINALSIGYAHRPI".to_owned());
        assert_eq!(Solution::convert("A".to_owned(), 1), "A".to_owned());
    }
}
