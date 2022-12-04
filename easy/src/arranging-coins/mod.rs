pub struct Solution;

impl Solution {
    pub fn arrange_coins(n: i32) -> i32 {
        let n = n as i64;
        let mut lo = 1;
        let mut hi = n;
        let mut ans = -1;

        while lo <= hi {
            let mid = lo + (hi - lo) / 2;

            if (1 + mid) * mid <= n * 2 {
                lo = mid + 1;
                ans = mid;
            } else {
                hi = mid - 1;
            }
        }

        ans as i32
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::arrange_coins(5), 2);
    assert_eq!(Solution::arrange_coins(8), 3);
}
