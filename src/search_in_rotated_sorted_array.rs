pub struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() {
            return -1;
        }

        let mut low: usize = 0;
        let mut high: usize = nums.len() - 1;

        while low <= high {
            let mid = low + (high - low) / 2;
            if nums[mid] == target {
                return mid as i32;
            }

            if nums[low] == nums[mid] && nums[mid] == nums[high] {
                low = low.saturating_add(1);
                if high == 0 {
                    break;
                }
                high = high.saturating_sub(1);
                continue;
            }

            if nums[low] <= nums[mid] {
                if nums[low] <= target && target < nums[mid] {
                    if mid == 0 {
                        break;
                    }
                    high = mid - 1;
                } else {
                    low = mid + 1;
                }
            } else {
                if nums[mid] < target && target <= nums[high] {
                    low = mid + 1;
                } else {
                    if mid == 0 {
                        break;
                    }
                    high = mid - 1;
                }
            }
        }

        -1
    }
}
