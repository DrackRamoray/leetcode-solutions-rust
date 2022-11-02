pub struct Solution;

impl Solution {
    pub fn is_strobogrammatic(num: String) -> bool {
        let mut i = 0;
        let mut j = num.len() - 1;
        let num = num.chars().collect::<Vec<char>>();

        while i <= j && i < num.len() {
            if num[j] != Self::get_symmetry_num(num[i]) {
                return false;
            }
            i += 1;
            j = j.wrapping_sub(1);
        }

        true
    }

    fn get_symmetry_num(n: char) -> char {
        match n {
            '0' => '0',
            '1' => '1',
            '6' => '9',
            '8' => '8',
            '9' => '6',
            _ => ' ',
        }
    }
}


#[test]
fn it_works() {
    assert_eq!(Solution::is_strobogrammatic("69".into()), true);
    assert_eq!(Solution::is_strobogrammatic("88".into()), true);
    assert_eq!(Solution::is_strobogrammatic("962".into()), false);
    assert_eq!(Solution::is_strobogrammatic("1".into()), true);
}
