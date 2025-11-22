impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        nums.iter().fold(0, |mut acc, &x| {
            let r = x % 3;
            acc += (r != 0) as i32;
            acc
        })
    }
}

pub struct Solution;