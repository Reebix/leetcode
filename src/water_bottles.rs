impl Solution {
    pub fn num_water_bottles(mut num_bottles: i32, num_exchange: i32) -> i32 {
        let mut total = num_bottles;
        while num_bottles >= num_exchange {
            let full_bottles = num_bottles / num_exchange;
            num_bottles %= num_exchange;
            num_bottles += full_bottles;
            total += full_bottles;
        }
        total
    }
}

pub struct Solution;