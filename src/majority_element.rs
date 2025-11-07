use std::collections::HashMap;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let num_set = nums.iter().fold(HashMap::new(), |mut acc, num| {
            *acc.entry(num).or_insert(0) += 1;
            acc
        });
        *num_set.into_iter().max_by_key(|&(_, cnt)| cnt).map(|(k, _)| k).unwrap()
    }
}

pub struct Solution;