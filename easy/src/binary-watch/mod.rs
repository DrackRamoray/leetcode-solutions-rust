pub struct Solution;

impl Solution {
    pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
        let mut ans: Vec<String> = vec![];

        for i in 0..12 {
            for j in 0..60 {
                if i32::count_ones(i) + i32::count_ones(j) == turned_on as u32 {
                    ans.push(format!("{}:{:02}", i, j));
                }
            }
        }

        ans
    }
}

#[test]
fn it_works() {
    use assist::vec_stringify;
    assert_eq!(Solution::read_binary_watch(1), vec_stringify!(["0:01","0:02","0:04","0:08","0:16","0:32","1:00","2:00","4:00","8:00"]));
    assert_eq!(Solution::read_binary_watch(9), vec![] as Vec<String>);
}
