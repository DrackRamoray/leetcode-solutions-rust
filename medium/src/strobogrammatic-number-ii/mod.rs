pub struct Solution;

impl Solution {
    pub fn find_strobogrammatic(n: i32) -> Vec<String> {
        Self::helper(n, n)
    }

    fn helper(n: i32, m: i32) -> Vec<String> {
        if n == 0 {
            return vec!["".into()];
        }

        if n == 1 {
            return vec!["0".into(), "1".into(), "8".into()];
        }

        let ans = Self::helper(n - 2, m);
        let mut res = vec![];

        for s in ans {
            if n != m {
                res.push(format!("{}{}{}", 0, s, 0));
            }

            res.push(format!("{}{}{}", 1, s, 1));
            res.push(format!("{}{}{}", 6, s, 9));
            res.push(format!("{}{}{}", 8, s, 8));
            res.push(format!("{}{}{}", 9, s, 6));
        }

        res
    }
}

#[test]
fn it_works() {
    use assist::vec_stringify;

    let n = 1;
    let mut res = vec_stringify!(vec!["0", "1", "8"]);
    let mut ans = Solution::find_strobogrammatic(n);
    res.sort();
    ans.sort();
    assert_eq!(ans, res);
    let n = 2;
    let mut res = vec_stringify!(vec!["11", "69", "88", "96"]);
    let mut ans = Solution::find_strobogrammatic(n);
    res.sort();
    ans.sort();
    assert_eq!(ans, res);
}
