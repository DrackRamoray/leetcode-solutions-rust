struct Solution;

impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut stack = Vec::new();
        let path_arr = path.split('/');

        for p in path_arr {
            match p {
                "." | "" => (),
                ".." => {
                    if stack.len() > 0 {
                        stack.pop();
                    }
                },
                _ => stack.push(p)
            }
        }

        "/".to_string() + &stack.join("/")
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(super::Solution::simplify_path("/home/".to_string()), "/home".to_string());
        assert_eq!(super::Solution::simplify_path("/../".to_string()), "/".to_string());
        assert_eq!(super::Solution::simplify_path("/home//foo/".to_string()), "/home/foo".to_string());
        assert_eq!(super::Solution::simplify_path("/a/./b/../../c/".to_string()), "/c".to_string());
    }
}
