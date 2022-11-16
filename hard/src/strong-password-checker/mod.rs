pub struct Solution;

impl Solution {
    pub fn strong_password_checker(password: String) -> i32 {
        let mut miss_types = 3;

        if password.chars().any(|c| c.is_lowercase()) {
            miss_types -= 1;
        }

        if password.chars().any(|c| c.is_uppercase()) {
            miss_types -= 1;
        }

        if password.chars().any(|c| c.is_digit(10)) {
            miss_types -= 1;
        }

        let n = password.len();
        let s = password.as_bytes();
        let (mut a, mut b, mut r) = (0, 0, 0);
        let mut i = 2;

        while i < n {
            if s[i] == s[i-1] && s[i] == s[i-2] {
                let mut tmp = 3;

                while i + 1 < n && s[i] == s[i+1] {
                    tmp += 1;
                    i += 1;
                }

                if tmp % 3 == 0 {
                    a += 1;
                } else if tmp % 3 == 1 {
                    b += 1;
                }

                r += tmp / 3;
            }

            i += 1;
        }

        if n < 6 {
            return miss_types.max(6 - n as i32);
        }

        if n <= 20 {
            return miss_types.max(r);
        }

        let d = n as i32 - 20;
        r -= a.min(d);

        if d > a {
            r -= b.min((d - a) / 2);
        }

        if d > a + 2 * b {
            r -= (d - a - 2 * b) / 3;
        }

        d + r.max(miss_types)
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::strong_password_checker("a".to_string()), 5);
    assert_eq!(Solution::strong_password_checker("aA1".to_string()), 3);
    assert_eq!(Solution::strong_password_checker("1337C0d3".to_string()), 0);
}
