impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        // not quite binary but still works?
        nums.iter().min().unwrap_or(&-1).clone()
    }
}

pub struct Solution;