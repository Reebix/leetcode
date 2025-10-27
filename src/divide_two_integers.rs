impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        let (val, overflowed) = dividend.overflowing_div(divisor);
        if overflowed { if divisor > 0 { i32::MIN } else { i32::MAX } } else { val }
    }
}

pub struct Solution;