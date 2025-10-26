mod roman_to_integer;
mod longest_common_prefix;
mod valid_parentheses;

fn main() {
    println!("{}", roman_to_integer::Solution::roman_to_int("MCMXCIV".to_string()));

    println!("{}", longest_common_prefix::Solution::longest_common_prefix(vec!["flower".parse().unwrap(), "flow".parse().unwrap(), "flight".parse().unwrap()]));
    println!("{}", longest_common_prefix::Solution::longest_common_prefix(vec!["a".parse().unwrap()]));

    println!("{} == true", valid_parentheses::Solution::is_valid("()".to_string()));
    println!("{} == true", valid_parentheses::Solution::is_valid("()[]{}".to_string()));
    println!("{} == true", valid_parentheses::Solution::is_valid("([])".to_string()));
    println!("{} == false", valid_parentheses::Solution::is_valid("([)]".to_string()));
}
