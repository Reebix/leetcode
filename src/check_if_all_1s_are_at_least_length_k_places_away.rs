impl Solution {
    pub fn k_length_apart(nums: Vec<i32>, k: i32) -> bool {
        let mut i = 0;
        let mut first = true;
        for x in nums {
            if x == 1 {
                if i < k && !first { return false; }
                first = false;
                i = 0;
            } else {
                i += 1;
            }
        }

        true
    }
}

pub struct Solution;