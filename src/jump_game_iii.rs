use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn can_reach(arr: Vec<i32>, start: i32) -> bool {
        let mut paths: HashSet<i32> = HashSet::new();
        let mut queue: VecDeque<i32> = VecDeque::new();

        queue.push_front(start);

        while queue.len() > 0 {
            let current = queue.pop_front().unwrap();

            let step_size = arr[current as usize];

            if step_size == 0 {
                return true;
            }

            let left = current - step_size;
            let right = current + step_size;

            if left >= 0 && !paths.contains(&left) {
                queue.push_back(left);
                paths.insert(current);
            }

            if right < arr.len() as i32 && !paths.contains(&right) {
                queue.push_back(right);
                paths.insert(right);
            }
        }

        false
    }
}

pub struct Solution;
