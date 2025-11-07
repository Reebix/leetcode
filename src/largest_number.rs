use std::cmp::Ordering;

impl Solution {
    fn compare_by_digits(x: i32, k: i32) -> Ordering {
        let a: Vec<char> = x.to_string().chars().collect();
        let b: Vec<char> = k.to_string().chars().collect();
        let max_len = a.len().max(b.len());

        for i in 0..max_len {
            let ca = if i < a.len() { a[i] } else { *a.last().unwrap() };
            let cb = if i < b.len() { b[i] } else { *b.last().unwrap() };

            let da = ca.to_digit(10).unwrap();
            let db = cb.to_digit(10).unwrap();

            if da > db {
                return Ordering::Less;
            } else if da < db {
                return Ordering::Greater;
            }
        }
        Ordering::Equal
    }
    pub fn largest_number(mut nums: Vec<i32>) -> String {
        nums.sort_by(|a, b| Self::compare_by_digits(*a, *b));
        let mut str = String::new();
        nums.iter().for_each(|n| {
            str.push_str(n.to_string().as_str());
        });
        println!("{:?}", nums);
        str
    }
}

pub struct Solution;