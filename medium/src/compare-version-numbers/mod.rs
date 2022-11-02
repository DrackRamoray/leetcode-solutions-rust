struct Solution;

impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let v1 = version1.split('.').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let v2 = version2.split('.').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let n = v1.len();
        let m = v2.len();
        let mut i = 0_usize;

        while i < n && i < m {
            if v1[i] > v2[i] {
                return 1;
            }

            if v2[i] > v1[i] {
                return -1;
            }

            i += 1;
        }

        while i < n {
            if v1[i] > 0 {
                return 1;
            }

            i += 1;
        }

        while i < m {
            if v2[i] > 0 {
                return -1;
            }

            i += 1;
        }

        0
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::compare_version("1.01".into(), "1.001".into()), 0);
        assert_eq!(Solution::compare_version("1.0".into(), "1.0.0".into()), 0);
        assert_eq!(Solution::compare_version("0.1".into(), "1.1".into()), -1);
    }
}
