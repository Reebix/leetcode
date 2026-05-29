impl Solution {
    pub fn min_element(nums: Vec<i32>) -> i32 {
        nums.iter().map(|x| {
            sum_digits(*x)
        }).min().unwrap()
    }
}


fn sum_digits(x: i32) -> i32 {
    (x % 10) + (0..)
        .scan(x, |num, _| {
            *num /= 10;
            Some(*num)
        })
        .take_while(|num| *num > 0)
        .map(|num| num % 10)
        .sum::<i32>()
}
pub struct Solution;