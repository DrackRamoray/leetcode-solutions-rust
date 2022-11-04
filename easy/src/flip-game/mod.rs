pub struct Solution;

impl Solution {
    pub fn generate_possible_next_moves(current_state: String) -> Vec<String> {
        let mut ans = vec![];
        let s = current_state.as_bytes();
        let n = s.len() - 1;

        for i in 0..n {
            if s[i] == b'+' && s[i+1] == b'+' {
                ans.push(current_state[0..i].to_string() + "--" + &current_state[i+2..]);
            }
        }

        ans
    }
}


#[test]
fn it_works() {
    use assist::vec_stringify;

    assert_eq!(Solution::generate_possible_next_moves("++++".into()), vec_stringify!(["--++","+--+","++--"]));
    assert_eq!(Solution::generate_possible_next_moves("+".into()), Vec::new() as Vec<String>);
}
