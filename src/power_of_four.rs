impl Solution {
    pub fn is_power_of_four(n: i32) -> bool {
        let n = n as u32;
        n != 0 &&
            (n & (n - 1)) // power of two
                == 0 && (n & 0x5555_5555) != 0 // odd bit 0x5555_5555 == 0101 ...
    }
}

pub struct Solution;