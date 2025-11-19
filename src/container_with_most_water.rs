use std::cmp::{max, min};

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = height.len() - 1;
        let mut ans = 0;


        while left < right {
            ans = max(ans, (right - left) as i32 * min(height[left], height[right]));
            if height[left] < height[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }

        ans
    }
}

pub struct Solution;