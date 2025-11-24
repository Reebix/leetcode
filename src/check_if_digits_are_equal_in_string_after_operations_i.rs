impl Solution {
    pub fn has_same_digits(mut s: String) -> bool {
        let mut s = s.into_bytes();
        for byte in &mut s { *byte -= b'0' }
        for z in (2..s.len()).rev() {
            for i in 0..z {
                s[i] = (s[i] + s[i + 1]) % 10;
            }
        }


        s[0] == s[1]
    }
}

pub struct Solution;