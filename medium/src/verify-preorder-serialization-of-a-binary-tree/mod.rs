pub struct Solution;

impl Solution {
    pub fn is_valid_serialization(preorder: String) -> bool {
        let chars = preorder.split(',').collect::<Vec<&str>>();
        let mut nodes = 1;

        for ch in chars {
            if nodes == 0 {
                return false;
            }

            match ch {
                "#" => nodes -= 1,
                _ => nodes += 1
            }
        }

        nodes == 0
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::is_valid_serialization("9,3,4,#,#,1,#,#,2,#,6,#,#".to_string()), true);
    assert_eq!(Solution::is_valid_serialization("1,#".to_string()), false);
    assert_eq!(Solution::is_valid_serialization("9,#,#,1".to_string()), false);
}
