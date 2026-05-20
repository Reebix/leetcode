use std::collections::HashSet;

impl Solution {
    pub fn find_the_prefix_common_array(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let mut out = Vec::new();
        let mut unique = HashSet::new();
        let mut current_common = 0;

        for i in 0..a.len() {
            let first = a[i];
            let second = b[i];

            if first == second {
                current_common += 1;
            } else {
                if unique.contains(&first) {
                    current_common += 1;
                    unique.remove(&first);
                } else {
                    unique.insert(first);
                }

                if unique.contains(&second) {
                    current_common += 1;
                    unique.remove(&second);
                } else {
                    unique.insert(second);
                }
            }
            out.push(current_common);
        }

        out
    }
}

pub struct Solution;
