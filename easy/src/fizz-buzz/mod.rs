pub struct Solution;

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut res = vec![];

        for i in 1..=n {
            let d3 = i % 3 == 0;
            let d5 = i % 5 == 0;
            if d3 && d5 {
                res.push("FizzBuzz".to_string());
            } else if d5 {
                res.push("Buzz".to_string());
            } else if d3 {
                res.push("Fizz".to_string())
            } else {
                res.push(i.to_string())
            }
        }

        res
    }
}

#[test]
fn it_works() {
    use assist::vec_stringify;
    assert_eq!(Solution::fizz_buzz(3), vec_stringify!(["1","2","Fizz"]));
    assert_eq!(Solution::fizz_buzz(5), vec_stringify!(["1","2","Fizz","4","Buzz"]));
    assert_eq!(Solution::fizz_buzz(15), vec_stringify!(["1","2","Fizz","4","Buzz","Fizz","7","8","Fizz","Buzz","11","Fizz","13","14","FizzBuzz"]));
}
