impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        for i in 0..nums.len() {
            let left_sum: i32 = nums.iter().take(i).sum();

            let right_sum: i32 = nums.iter().skip(i + 1).take(nums.len() - i - 1).sum();
            if right_sum == left_sum {
                return i as i32;
            }
        }
        -1
    }
}

pub struct Solution;