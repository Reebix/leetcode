impl Solution {
    pub fn max_bottles_drunk(mut num_bottles: i32, mut num_exchange: i32) -> i32 {
        let mut total = num_bottles;
        while num_bottles >= num_exchange {
            num_bottles -= num_exchange;
            num_bottles += 1;
            total += 1;
            num_exchange += 1;
        }
        total
    }
}

pub struct Solution;