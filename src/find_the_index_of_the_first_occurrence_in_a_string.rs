impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if let Some(some) = haystack.find(&needle) {
            return some as i32;
        }
        -1
    }
}

pub struct Solution;