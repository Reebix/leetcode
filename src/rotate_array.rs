impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        if nums.len() == 1 { return; }
        for _ in 0..k as usize {
            let rem = nums.remove(nums.len() - 1);
            nums.insert(0, rem);
        }
    }
}

pub struct Solution;