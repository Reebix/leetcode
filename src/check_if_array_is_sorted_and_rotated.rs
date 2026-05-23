impl Solution {
    pub fn check(mut nums: Vec<i32>) -> bool {
        if nums.len() < 3 {
            return true;
        }

        if nums[0] < nums[nums.len() - 1] {
            if nums[1] < nums[0] {
                return false;
            }
            nums[0] = 1;
        } else {
            if nums[1] < nums[0] {
                nums[0] = 1;
            } else {
                nums[0] = 0;
            }
        }

        for i in 2..nums.len() {
            if nums[i] >= nums[i - 1] {
                continue;
            }
            if nums[0] == 2 {
                return false;
            }
            nums[0] += 1;
        }
        return nums[0] != 2;
    }
}

pub struct Solution;