pub struct Solution;

impl Solution {
    pub fn add_operators(num: String, target: i32) -> Vec<String> {
        let mut ans = vec![];

        Self::dfs(&num, &mut ans, "".to_owned(), 0, target as i64, 0, 0);

        ans
    }

    fn dfs(s: &str, ans: &mut Vec<String>, selected: String, index: usize, target: i64, prev: i64, curr: i64) {
        if index == s.len() {
            if curr == target {
                ans.push(selected);
            }
            return;
        }

        let ss = s.as_bytes();

        for i in index..s.len() {
            if i != index && ss[index] == b'0' {
                break;
            }

            let val = (&s[index..i+1]).parse::<i64>().unwrap();

            if index == 0 {
                Self::dfs(&s, ans, (&s[index..i+1]).to_string(), i + 1, target, val, val);
            } else {
                Self::dfs(&s, ans, selected.to_owned() + "+" + &s[index..i+1], i + 1, target, val, curr + val);
                Self::dfs(&s, ans, selected.to_owned() + "-" + &s[index..i+1], i + 1, target, -val, curr - val);
                Self::dfs(&s, ans, selected.to_owned() + "*" + &s[index..i+1], i + 1, target, prev * val, curr - prev + prev * val);
            }
        }
    }
}

#[test]
fn it_works() {
    use assist::vec_stringify;
    assert_eq!(Solution::add_operators("123".to_string(), 6), vec_stringify!(vec!["1+2+3","1*2*3"]));
    assert_eq!(Solution::add_operators("232".to_string(), 8), vec_stringify!(vec!["2+3*2","2*3+2"]));
    assert_eq!(Solution::add_operators("3456237490".to_string(), 9191), vec![] as Vec<String>);
}
