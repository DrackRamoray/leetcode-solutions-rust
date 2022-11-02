struct Solution;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut v1 = vec![128;128];
        let mut v2 = vec![128;128];
        let ss = s.as_bytes();
        let tt = t.as_bytes();
        let n = ss.len();

        for n in 0..n {
            let i = (ss[n]) as usize;
            let j = (tt[n]) as usize;

            if (v1[i] != 128 && v1[i] != j) || (v2[j] != 128 && v2[j] != i) {
                return false;
            }

            v1[i] = j;
            v2[j] = i;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::is_isomorphic("egg".into(), "add".into()), true);
        assert_eq!(Solution::is_isomorphic("foo".into(), "bar".into()), false);
        assert_eq!(Solution::is_isomorphic("paper".into(), "title".into()), true);
    }
}
