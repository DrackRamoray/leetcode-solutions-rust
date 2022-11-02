struct Solution;

impl Solution {
    pub fn title_to_number(column_title: String) -> i32 {
        let mut ans = 0_i32;

        for ch in column_title.as_bytes().iter() {
            ans = ans * 26 + *ch as i32 - 'A' as u8 as i32 + 1;
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::title_to_number("A".into()), 1);
        assert_eq!(Solution::title_to_number("AB".into()), 28);
        assert_eq!(Solution::title_to_number("ZY".into()), 701);
    }
}
