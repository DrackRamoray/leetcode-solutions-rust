pub struct Solution;

impl Solution {
    pub fn first_bad_version(&self, n: i32) -> i32 {
        let mut lo = 0;
        let mut hi = n;
        let mut ans = -1;

        while lo <= hi {
            let mid = lo + (hi - lo) / 2;

            if self.isBadVersion(mid) {
                ans = mid;
                hi = mid - 1;
            } else {
                lo = mid + 1;
            }
        }

        ans
    }

    // useless, prevent fault from test
    #[allow(non_snake_case)]
    fn isBadVersion(&self, _: i32) -> bool {
        true
    }
}
