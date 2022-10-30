pub struct Solution;

impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let n = ratings.len();
        let mut cnt = vec![1;n];

        for i in 1..n {
            if ratings[i] > ratings[i-1] {
                cnt[i] = cnt[i].max(cnt[i-1] + 1);
            }
        }

        for i in (0..n-1).rev() {
            if ratings[i] > ratings[i+1] {
                cnt[i] = cnt[i].max(cnt[i+1] + 1);
            }
        }

        cnt.into_iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::candy(vec![1,0,2]), 5);
        assert_eq!(Solution::candy(vec![1,2,2]), 4);
    }
}
