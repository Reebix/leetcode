use crate::merge_two_sorted_lists::ListNode;

mod roman_to_integer;
mod longest_common_prefix;
mod valid_parentheses;
mod merge_two_sorted_lists;

fn main() {
    println!("{}", roman_to_integer::Solution::roman_to_int("MCMXCIV".to_string()));

    println!("{}", longest_common_prefix::Solution::longest_common_prefix(vec!["flower".parse().unwrap(), "flow".parse().unwrap(), "flight".parse().unwrap()]));
    println!("{}", longest_common_prefix::Solution::longest_common_prefix(vec!["a".parse().unwrap()]));

    println!("{} == true", valid_parentheses::Solution::is_valid("()".to_string()));
    println!("{} == true", valid_parentheses::Solution::is_valid("()[]{}".to_string()));
    println!("{} == true", valid_parentheses::Solution::is_valid("([])".to_string()));
    println!("{} == false", valid_parentheses::Solution::is_valid("([)]".to_string()));

    let list1 = Some(Box::new(ListNode { val: 1, next: Some(Box::new(ListNode { val: 2, next: Some(Box::new(ListNode { val: 4, next: None })) })) }));
    let list2 = Some(Box::new(ListNode { val: 1, next: Some(Box::new(ListNode { val: 3, next: Some(Box::new(ListNode { val: 4, next: None })) })) }));

    println!("{:#?}", merge_two_sorted_lists::Solution::merge_two_lists(list1, list2));
}
