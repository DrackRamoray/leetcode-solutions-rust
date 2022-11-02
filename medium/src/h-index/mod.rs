pub struct Solution;

impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let n = citations.len();
        let mut ans = 0;
        let mut cnt = vec![0;n+1];

        for i in 0..n {
            if citations[i] >= n as i32 {
                cnt[n] += 1;
            } else {
                cnt[citations[i] as usize] += 1;
            }
        }

        for i in (0..=n).rev() {
            ans += cnt[i];

            if ans >= i {
                return i as i32;
            }
        }

        0
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::h_index(vec![3,0,6,1,5]), 3);
    assert_eq!(Solution::h_index(vec![1,3,1]), 1);
}
