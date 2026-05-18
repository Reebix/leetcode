use std::collections::{HashMap, HashSet, VecDeque};

impl Solution {
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        // <number_in_array, partner_set>
        let mut paths: HashMap<i32, HashSet<usize>> = HashMap::new();
        // <index, min_steps>
        let mut visited: HashMap<usize, i32> = HashMap::new();
        let mut result = (arr.len() - 1) as i32;

        for i in 0..arr.len() {
            paths.entry(arr[i]).or_insert(HashSet::new()).insert(i);
        }

        let mut queue: VecDeque<(usize, i32)> = VecDeque::new();
        queue.push_back((0, 0));

        while !queue.is_empty() {
            let (current_index, steps) = queue.pop_front().unwrap();

            if current_index == arr.len() - 1 {
                if steps < result {
                    result = steps
                }
                continue;
            }

            let right = current_index + 1;

            let steps = steps + 1;

            let known_min = visited
                .get(&current_index)
                .unwrap_or(&(arr.len() as i32))
                .clone();

            if known_min + 1 < steps {
                continue;
            }

            if let Some(path) = paths.remove(&arr[current_index]) {
                path.iter().for_each(|jump_target| {
                    let known_min = visited
                        .get(&jump_target)
                        .unwrap_or(&(arr.len() as i32))
                        .clone();

                    if steps < known_min {
                        queue.push_front((*jump_target, steps));
                        visited.insert(*jump_target, steps);
                    }
                });
            }

            if current_index > 0 && steps < result {
                let left = current_index - 1;

                let known_min = visited.get(&left).unwrap_or(&(arr.len() as i32)).clone();

                if steps < known_min {
                    queue.push_back((left, steps));
                    visited.insert(left, steps);
                }
            }

            if right < arr.len() && steps < result {
                let known_min = visited.get(&right).unwrap_or(&(arr.len() as i32)).clone();

                if steps < known_min {
                    queue.push_back((right, steps));
                    visited.insert(right, steps);
                }
            }
        }

        result
    }
}

pub struct Solution;
