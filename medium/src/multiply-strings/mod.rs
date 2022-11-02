pub struct Solution;

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        let n = num1.len();
        let m = num2.len();
        let mut ans = vec![0;n+m];
        let n1 = num1.as_bytes();
        let n2 = num2.as_bytes();

        for i in (0..n).rev() {
            let a = n1[i] as i32 - 48;

            for j in (0..m).rev() {
                let b = n2[j] as i32 - 48;
                ans[i+j+1] += a * b;
            }
        }

        for i in (1..n+m).rev() {
            ans[i-1] += ans[i] / 10;
            ans[i] = ans[i] % 10;
        }

        let mut i = 0;

        while i < n+m-1 {
            if ans[i] == 0 {
                i += 1;
            } else {
                break;
            }
        }

        ans[i..].into_iter().map(|x| x.to_string()).collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::multiply("2".to_owned(), "3".to_owned()), "6".to_owned());
        assert_eq!(Solution::multiply("123".to_owned(), "456".to_owned()), "56088".to_owned());
    }
}
