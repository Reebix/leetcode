use std::collections::HashMap;

impl Solution {
    pub fn find_lucky(arr: Vec<i32>) -> i32 {
        let mut num_set = arr.iter().fold(HashMap::new(), |mut acc, num| {
            *acc.entry(num).or_insert(0) += 1;
            acc
        });
        let mut nums: Vec<i32> = vec![];

        let mut add = 1;
        while add != -1 {
            add = **num_set.iter().find(move |(key, v)| { **key == *v }).unwrap_or_else(|| { (&&-1, &-1) }).0;
            nums.push(add);
            num_set.remove(&add);
        }
        nums.sort();
        nums.last().unwrap().clone()
    }
}

pub struct Solution;