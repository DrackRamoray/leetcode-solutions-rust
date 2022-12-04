pub struct Solution;

impl Solution {
    pub fn find_kth_number(n: i32, k: i32) -> i32 {
        let mut ans = 1;
        let mut k = k - 1;

        while k > 0 {
            let steps = Self::get_steps(ans, n as u64);

            if steps <= k {
                k -= steps;
                ans += 1;
            } else {
                ans *= 10;
                k -= 1;
            }
        }

        ans
    }

    fn get_steps(cur: i32, n: u64) -> i32 {
        let mut cnt = 0;
        let mut prev = cur as u64;
        let mut next = cur as u64;

        while prev <= n {
            cnt += next.min(n) - prev + 1;
            prev *= 10;
            next = next * 10 + 9;
        }

        cnt as i32
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::find_kth_number(12, 2), 10);
    assert_eq!(Solution::find_kth_number(1, 1), 1);
}
