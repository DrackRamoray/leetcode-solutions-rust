pub struct Solution;

impl Solution {
    fn min_abbreviation(target: String, dictionary: Vec<String>) -> String {
        let n = target.len();
        let target = target.chars().collect::<Vec<char>>();
        let dictionary = dictionary.into_iter().filter(|s| s.len() == n).map(|s| s.chars().collect()).collect::<Vec<Vec<char>>>();
        let mut mask: u32 = 0;
        let mut ans: u32 = !0;

        Self::dfs(&dictionary, &mut mask, &mut ans, &target, 0);
        Self::abbr(&target, ans)
    }

    fn dfs(dictionary: &[Vec<char>], mask: &mut u32, ans: &mut u32, target: &[char], begin: usize) {
        let n = target.len();

        if begin == n {
            for word in dictionary.iter() {
                if Self::check(target, word, *mask, n) {
                    return;
                }
            }

            if Self::get_size(*mask, n) < Self::get_size(*ans, n) {
                *ans = *mask;
            }
        } else {
            Self::dfs(dictionary, mask, ans, target, begin + 1);
            *mask |= 1 << begin;
            Self::dfs(dictionary, mask, ans, target, begin + 1);
            *mask &= !(1 << begin);
        }
    }

    fn check(target: &[char], word: &[char], mask: u32, n: usize) -> bool {
        (0..n).all(|i| mask & 1 << i == 0 || target[i] == word[i])
    }

    fn abbr(target: &[char], mask: u32) -> String {
        let n = target.len();
        let mut i = 0;
        let mut ans = String::new();

        while i < n {
            if mask & 1 << i == 0 {
                let mut cnt = 1;

                while i + 1 < n && mask & 1 << (i + 1) == 0 {
                    cnt += 1;
                    i += 1;
                }

                ans.push_str(&cnt.to_string());
            } else {
                ans.push(target[i]);
            }

            i += 1;
        }

        ans
    }

    fn get_size(mask: u32, n: usize) -> usize {
        let mut i = 0;
        let mut ans = 0;

        while i < n {
            if mask & 1 << i == 0 {
                while i + 1 < n && mask & 1 << (i + 1) == 0 {
                    i += 1;
                }
            }

            ans += 1;
            i += 1;
        }
        ans
    }
}

#[test]
fn it_works() {
    use assist::vec_stringify;
    assert_eq!(Solution::min_abbreviation("apple".to_string(), vec_stringify!(["blade"])), "a4".to_string());
    assert_eq!(Solution::min_abbreviation("apple".to_string(), vec_stringify!(["blade","plain","amber"])), "3l1".to_string());
}
