pub struct Solution;

impl Solution {
    pub fn length_longest_path(input: String) -> i32 {
        let mut stack = vec![];
        let inputs = input.split("\n");
        let mut ans = 0;
        let mut prev = -1;

        for item in inputs {
            let len = item.trim_start_matches('\t').len();
            let num = (item.len() - len) as i32;

            if num != prev + 1 {
                stack.drain(num as usize..);
            }

            stack.push(len);

            if item.contains(".") {
                ans = ans.max(stack.len() - 1 + stack.iter().sum::<usize>());
            }

            prev = num;
        }

        ans as i32
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::length_longest_path("dir\n\tsubdir1\n\tsubdir2\n\t\tfile.ext".to_string()), 20);
    assert_eq!(Solution::length_longest_path("dir\n\tsubdir1\n\t\tfile1.ext\n\t\tsubsubdir1\n\tsubdir2\n\t\tsubsubdir2\n\t\t\tfile2.ext".to_string()), 32);
    assert_eq!(Solution::length_longest_path("a".to_string()), 0);
    assert_eq!(Solution::length_longest_path("file1.txt\nfile2.txt\nlongfile.txt".to_string()), 12);
}
