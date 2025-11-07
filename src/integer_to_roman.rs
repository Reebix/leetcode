impl Solution {
    pub fn int_to_roman(mut num: i32) -> String {
        let mut ans = String::new();
        let instruction_set = vec![("M", 1000), ("CM", 900), ("D", 500), ("CD", 400), ("C", 100), ("XC", 90), ("L", 50), ("XL", 40), ("X", 10), ("IX", 9), ("V", 5), ("IV", 4), ("I", 1)];
        instruction_set.iter().for_each(|&(symbol, amount)| {
            while num >= amount {
                num -= amount;
                ans.push_str(symbol);
            }
        });

        ans
    }
}

pub struct Solution;