use std::collections::HashSet;

impl Solution {
    pub fn longest_common_prefix(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        let mut prefixes: HashSet<i32> = HashSet::new();
        let mut longest_common_prefix = 0;

        for i in 0..arr1.len() {
            let curr = arr1[i];
            let curr_len = curr.to_string().len();

            for j in 0..curr_len {
                prefixes.insert(curr / 10_i32.pow((curr_len - 1 - j) as u32));
            }
        }

        for i in 0..arr2.len() {
            let curr = arr2[i];
            let curr_len = curr.to_string().len();

            if prefixes.contains(&curr) {
                longest_common_prefix = longest_common_prefix.max(curr_len);
            }

            for j in 0..curr_len {
                let pref = curr / 10_i32.pow((curr_len - 1 - j) as u32);
                if !prefixes.contains(&pref) {
                    let len = pref.to_string().len();
                    if len > longest_common_prefix {
                        longest_common_prefix = len - 1;
                    }
                    break;
                }
            }
        }

        longest_common_prefix as i32
    }
}

pub struct Solution;
