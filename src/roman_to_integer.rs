pub struct Solution;

impl Solution {
    pub fn roman_to_int(mut s: String) -> i32 {
        fn val(c: char) -> i32 {
            match c {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => 0,
            }
        }

        s = s.replace("IV", "IIII").replace("IX", "VIIII");
        s = s.replace("XL", "XXXX").replace("XC", "LXXXX");
        s = s.replace("CD", "CCCC").replace("CM", "DCCCC");

        let chars: Vec<char> = s.chars().collect();
        let mut sum = 0;
        for i in 0..chars.len() {
            let v = val(chars[i]);
            sum += v;
        }
        sum
    }
}