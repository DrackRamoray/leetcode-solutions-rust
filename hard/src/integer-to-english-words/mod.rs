pub struct Solution;

impl Solution {
    pub fn number_to_words(num: i32) -> String {
        if num == 0 {
            return "Zero".to_string();
        }

        let nineteen: [&'static str;19] = ["One","Two","Three","Four","Five","Six","Seven","Eight","Nine","Ten","Eleven","Twelve","Thirteen","Fourteen","Fifteen","Sixteen","Seventeen","Eighteen","Nineteen"];
        let tens: [&'static str;8] = ["Twenty","Thirty","Forty","Fifty","Sixty","Seventy","Eighty","Ninety"];
        let units: [&'static str;4] = ["Hundred","Thousand","Million","Billion"];

        Self::transform(num as usize, &nineteen, &tens, &units)
    }

    fn transform (
        num: usize,
        nineteen: &[&'static str],
        tens: &[&'static str],
        units: &[&'static str],
    ) -> String {
        let word = if num < 20 {
            if num > 0 { nineteen[num - 1].to_string() } else { "".to_string() }
        } else if num < 100 {
            format!(
                "{} {}",
                if num / 10 > 1 { tens[num / 10 - 2].to_string() } else { "".to_string() },
                Self::transform(num % 10, nineteen, tens, units),
            )
        } else if num < 1000 {
            format!(
                "{} {} {}",
                Self::transform(num / 100, nineteen, tens, units),
                units[0],
                Self::transform(num % 100, nineteen, tens, units),
            )
        } else if num < 1_000_000 {
            format!(
                "{} {} {}",
                Self::transform(num / 1000, nineteen, tens, units),
                units[1],
                Self::transform(num % 1000, nineteen, tens, units),
            )
        } else if num < 1_000_000_000 {
            format!(
                "{} {} {}",
                Self::transform(num / 1_000_000, nineteen, tens, units),
                units[2],
                Self::transform(num % 1_000_000, nineteen, tens, units),
            )
        } else {
            format!(
                "{} {} {}",
                Self::transform(num / 1_000_000_000, nineteen, tens, units),
                units[3],
                Self::transform(num % 1_000_000_000, nineteen, tens, units),
            )
        };

        word.trim_end().into()
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::number_to_words(123), "One Hundred Twenty Three".to_string());
    assert_eq!(Solution::number_to_words(12345), "Twelve Thousand Three Hundred Forty Five".to_string());
    assert_eq!(Solution::number_to_words(1234567), "One Million Two Hundred Thirty Four Thousand Five Hundred Sixty Seven".to_string());
}
