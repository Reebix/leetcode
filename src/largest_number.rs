impl Solution {
    pub fn largest_number(mut nums: Vec<i32>) -> String {
        nums.sort_by(|a, b| {
            let ab = format!("{}{}", a, b);
            let ba = format!("{}{}", b, a);
            ba.cmp(&ab)
        });

        if nums[0] == 0 {
            return "0".to_string(); // leading zero means zero
        }
        let mut str = String::new();
        nums.iter().for_each(|n| {
            str.push_str(n.to_string().as_str());
        });
        str
    }
}

pub struct Solution;