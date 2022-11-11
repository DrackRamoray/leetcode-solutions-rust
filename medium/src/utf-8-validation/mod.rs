pub struct Solution;

impl Solution {
    pub fn valid_utf8(data: Vec<i32>) -> bool {
        let n = data.len();
        let mut i = 0;

        while i < n {
            if (data[i] >> 7) == 0b0 {
                i += 1;
            } else if (data[i] >> 5) == 0b110 {
                if i + 1 >= n {
                    return false;
                }

                if (data[i+1] >> 6) == 0b10 {
                    i += 2;
                } else {
                    return false;
                }
            } else if (data[i] >> 4) == 0b1110 {
                if i + 2 >= n {
                    return false;
                }

                if (data[i+1] >> 6) == 0b10 && (data[i+2] >> 6) == 0b10 {
                    i += 3;
                } else {
                    return false;
                }
            } else if (data[i] >> 3) == 0b11110 {
                if i + 3 >= n {
                    return false;
                }

                if (data[i+1] >> 6) == 0b10 && (data[i+2] >> 6) == 0b10 && (data[i+3] >> 6) == 0b10 {
                    i += 4;
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::valid_utf8(vec![197,130,1]), true);
    assert_eq!(Solution::valid_utf8(vec![235,140,4]), false);
}
