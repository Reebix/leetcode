use std::cmp::min;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut prefix: String = strs[0].to_string();
        while prefix != "" {
            if strs.iter().all(|string: &String| { *string.split_at(min(prefix.len(), string.len())).0 == *prefix }) {
                return prefix;
            }

            prefix = prefix.split_at(prefix.len() - 1).0.to_string();
        }

        "".to_string()
    }
}

pub struct Solution;