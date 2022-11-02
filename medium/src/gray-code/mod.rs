struct Solution;

impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        let mut ans = vec![0;1];
        let mut head = 1;

        for _ in 1..=n {
            for j in (0..=ans.len() - 1).rev() {
                ans.push(head + ans[j]);
            }

            head <<= 1;
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::gray_code(2), vec![0,1,3,2]);
        assert_eq!(Solution::gray_code(1), vec![0,1]);
    }
}
