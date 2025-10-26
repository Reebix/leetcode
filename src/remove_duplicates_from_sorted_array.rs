use std::collections::HashSet;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut seen: HashSet<i32> = HashSet::new();
        nums.retain(|num| {
            let contains = seen.contains(num);
            seen.insert(*num);
            !contains
        });

        nums.len() as i32
    }
}

pub struct Solution;