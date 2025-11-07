impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        n != 0 && (n & (n - 1)) == 0 && n != -2147483648
    }
}

pub struct Solution;