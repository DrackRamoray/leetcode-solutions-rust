pub struct Solution;

impl Solution {
    pub fn lexical_order(n: i32) -> Vec<i32> {
        let mut ans = vec![];
        let mut i = 1;

        for _ in 0..n {
            ans.push(i);

            if i * 10 <= n {
                i *= 10;
            } else {
                while i % 10 == 9 || i + 1 > n {
                    i /= 10;
                }

                i += 1;
            }
        }

        ans
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::lexical_order(13), vec![1,10,11,12,13,2,3,4,5,6,7,8,9]);
    assert_eq!(Solution::lexical_order(2), vec![1,2]);
}
