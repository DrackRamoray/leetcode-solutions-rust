pub struct Solution;

impl Solution {
    pub fn get_hint(secret: String, guess: String) -> String {
        let n = secret.len();
        let ss = secret.as_bytes();
        let gg = guess.as_bytes();
        let mut cnt_secret = vec![0;10];
        let mut cnt_guess = vec![0;10];
        let mut bulls = 0;
        let mut cows = 0;

        for i in 0..n {
            if ss[i] == gg[i] {
                bulls += 1;
            } else {
                cnt_secret[ss[i] as usize - '0' as usize] += 1;
                cnt_guess[gg[i] as usize - '0' as usize] += 1;
            }
        }

        for i in 0..10 {
            cows += cnt_secret[i].min(cnt_guess[i]);
        }

        bulls.to_string() + "A" + &cows.to_string() + "B"
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::get_hint("1807".to_string(), "7810".to_string()), "1A3B".to_string());
    assert_eq!(Solution::get_hint("1123".to_string(), "0111".to_string()), "1A1B".to_string());
}
