pub struct Solution;

impl Solution {
    pub fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
        g.sort();
        s.sort();

        let n = g.len();
        let m = s.len();
        let mut i = 0;
        let mut j = 0;
        let mut count = 0;

        while i < n && j < m {
            while j < m && s[j] < g[i] {
                j += 1;
            }

            if j < m {
                count += 1
            }

            i += 1;
            j += 1;
        }

        count
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::find_content_children(vec![1,2,3], vec![1,1]), 1);
    assert_eq!(Solution::find_content_children(vec![1,2], vec![1,2,3]), 2);
}
