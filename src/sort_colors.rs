impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        nums.sort_by(|a, b| a.cmp(b));
    }
}

pub struct Solution;