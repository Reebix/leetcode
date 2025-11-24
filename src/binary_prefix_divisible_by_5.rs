impl Solution {
    pub fn prefixes_div_by5(nums: Vec<i32>) -> Vec<bool> {
        let mut result = Vec::new();
        let mut curr = 0;
        nums.iter().for_each(|&bit| {
            curr = ((curr << 1) + bit) % 5;
            result.push(curr == 0);
        });

        result
    }
}

pub struct Solution {}