struct Solution;

impl Solution {
    pub fn is_ugly(num: i32) -> bool {
        if num == 0 {
            return false;
        }

        let mut n = num;

        while n % 2 == 0 {
            n = n / 2;
        }

        while n % 3 == 0 {
            n = n / 3;
        }

        while n % 5 == 0 {
            n = n / 5;
        }

        return n == 1;
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::is_ugly(6), true);
    assert_eq!(Solution::is_ugly(1), true);
    assert_eq!(Solution::is_ugly(14), false);
}
