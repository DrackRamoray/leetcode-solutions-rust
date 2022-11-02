pub struct Solution;

impl Solution {
    pub fn convert_to_title(mut column_number: i32) -> String {
        let mut ans = vec![];

        while column_number > 0 {
            let x = ((column_number - 1) % 26) as u8;
            let c = (x + b'A') as char;
            ans.insert(0, c);
            column_number = (column_number - 1) / 26;
        }

        ans.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::convert_to_title(1), "A".to_string());
        assert_eq!(Solution::convert_to_title(28), "AB".to_string());
        assert_eq!(Solution::convert_to_title(701), "ZY".to_string());
        assert_eq!(Solution::convert_to_title(2147483647), "FXSHRXW".to_string());
    }
}
