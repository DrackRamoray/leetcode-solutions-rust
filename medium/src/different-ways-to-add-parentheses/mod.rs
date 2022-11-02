struct Solution;

impl Solution {
    pub fn diff_ways_to_compute(expression: String) -> Vec<i32> {
        Self::compute(&expression)
    }

    fn compute(expression: &str) -> Vec<i32> {
        if Self::is_digit(expression) {
            let num = expression.parse::<i32>().unwrap();
            return vec![num];
        }

        let n = expression.len();
        let ss = expression.as_bytes();
        let mut ans = vec![];

        for i in 0..n {
            if ss[i] == b'+' || ss[i] == b'-' || ss[i] == b'*' {
                let left = Self::compute(&expression[0..i]);
                let right = Self::compute(&expression[i+1..]);

                for &left_num in left.iter() {
                    for &right_num in right.iter() {
                        let res = if ss[i] == b'+' {
                            left_num + right_num
                        } else if ss[i] == b'-' {
                            left_num - right_num
                        } else {
                            left_num * right_num
                        };
                        ans.push(res);
                    }
                }
            }
        }

        ans
    }

    fn is_digit(expression: &str) -> bool {
        for &u in expression.as_bytes() {
            if u < b'0' || u > b'9' {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::diff_ways_to_compute("2-1-1".to_string()), vec![2,0]);
        assert_eq!(Solution::diff_ways_to_compute("2*3-4*5".to_string()), vec![-34,-10,-14,-10,10]);
    }
}
