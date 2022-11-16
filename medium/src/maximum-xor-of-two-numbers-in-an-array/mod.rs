#[derive(Default)]
struct Trie {
    children: [Option<Box<Trie>>;2],
}

impl Trie {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn insert(&mut self, num: i32) {
        let mut s = self;

        for i in (0..31).rev() {
            let index = if (num >> i) & 1 == 0 { 0 } else { 1 };
            s = s.children[index].get_or_insert_with(|| Box::new(Trie::new()));
        }
    }

    pub fn check(&self, num: i32) -> i32 {
        let mut s = self;
        let mut res = 0;

        for i in (0..31).rev() {
            if (num >> i) & 1 == 0 {
                if let Some(ref child) = s.children[1] {
                    s = child;
                    res = (res << 1) + 1;
                } else if let Some(ref child) = s.children[0] {
                    s = child;
                    res = res << 1;
                }
            } else {
                if let Some(ref child) = s.children[0] {
                    s = child;
                    res = (res << 1) + 1;
                } else if let Some(ref child) = s.children[1] {
                    s = child;
                    res = res << 1;
                }
            }
        }

        res
    }
}

pub struct Solution;

impl Solution {
    pub fn find_maximum_xor(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut trie = Trie::new();
        for i in 1..nums.len() {
            trie.insert(nums[i - 1]);
            ans = ans.max(trie.check(nums[i]));
        }
        ans
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::find_maximum_xor(vec![3,10,5,25,2,8]), 28);
    assert_eq!(Solution::find_maximum_xor(vec![14,70,53,83,49,91,36,80,92,51,66,70]), 127);
}
