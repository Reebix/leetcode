use std::collections::HashMap;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        let num_set = nums.iter().fold(HashMap::new(), |mut acc, num| {
            *acc.entry(num).or_insert(0) += 1;
            acc
        });
        let mut out = vec![];

        num_set.iter().for_each(|(&k, &v)| {
            if nums.len() / 3 < v {
                out.push(*k)
            }
        });

        out
    }
}

pub struct Solution;