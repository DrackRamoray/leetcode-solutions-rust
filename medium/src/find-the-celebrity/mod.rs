pub struct Solution {}

impl Solution {
    pub fn find_celebrity(&self, n: i32) -> i32 {
        let mut ans = 0;

        for i in 0..n {
            if self.knows(ans, i) {
                ans = i;
            }
        }

        for i in 0..n {
            if ans != i {
                if !self.knows(i, ans) || self.knows(ans,i) {
                    return -1;
                }
            }
        }

        ans
    }

    // useless, prevent fault from testing
    fn knows(&self, _: i32, _: i32) -> bool {
        true
    }
}
