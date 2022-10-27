pub struct Solution;

impl Solution {
    pub fn get_permutation(n: i32, k: i32) -> String {
        let n = n as usize;
        let mut fact = vec![0;n];
        fact[0] = 1;

        for i in 1..n {
            fact[i] = fact[i-1] * i;
        }

        let mut k = k as usize - 1;
        let mut ans = vec![];
        let mut valid = vec![1;n + 1];

        for i in 1..=n {
            let mut order = k / fact[n - i] + 1;

            for j in 1..=n {
                order -= valid[j];

                if order == 0 {
                    ans.push(j);
                    valid[j] = 0;
                    break;
                }
            }

            k %= fact[n-i];
        }

        ans.into_iter().map(|v| (v as u8 + b'0') as char).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::get_permutation(3, 3), "213".to_owned());
        assert_eq!(Solution::get_permutation(4, 9), "2314".to_owned());
        assert_eq!(Solution::get_permutation(3, 1), "123".to_owned());
    }
}
