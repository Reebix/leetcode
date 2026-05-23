impl Solution {
    pub fn check(nums: Vec<i32>) -> bool {
        let mut was_nr = nums[0] < nums[nums.len() - 1];
        for i in 1..nums.len() {
            if nums[i] >= nums[i - 1] {
                continue;
            }
            if was_nr {
                return false;
            }
            was_nr = true;
        }
        true
    }
}

pub struct Solution;