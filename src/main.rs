mod roman_to_integer;
mod longest_common_prefix;
mod valid_parentheses;
mod merge_two_sorted_lists;
mod remove_duplicates_from_sorted_array;
mod remove_element;
mod find_the_index_of_the_first_occurrence_in_a_string;
mod divide_two_integers;
mod longest_substring_without_repeating_characters;
mod count_integers_in_intervals;
mod reverse_integer;
mod legth_of_last_word;
mod license_key_formatting;
mod plus_one;
mod climbing_stairs;
mod uses_cache;
mod lru_cache;
mod rotate_array;
mod zigzag_conversion;
mod rotate_image;
mod powx_n;
mod delete_nodes_from_linked_list_present_in_array;
mod power_of_four;
mod single_number;
mod largest_number;
mod power_of_two;
mod integer_to_roman;
mod sort_colors;
mod majority_element;
mod water_bottles;
mod water_bottles_ii;

fn main() {
    println!("{}", roman_to_integer::Solution::roman_to_int("MCMXCIV".to_string()));

    println!("{}", longest_common_prefix::Solution::longest_common_prefix(vec!["flower".parse().unwrap(), "flow".parse().unwrap(), "flight".parse().unwrap()]));
    println!("{}", longest_common_prefix::Solution::longest_common_prefix(vec!["a".parse().unwrap()]));

    println!("{} == true", valid_parentheses::Solution::is_valid("()".to_string()));
    println!("{} == true", valid_parentheses::Solution::is_valid("()[]{}".to_string()));
    println!("{} == true", valid_parentheses::Solution::is_valid("([])".to_string()));
    println!("{} == false", valid_parentheses::Solution::is_valid("([)]".to_string()));

    // let list1 = Some(Box::new(ListNode { val: 1, next: Some(Box::new(ListNode { val: 2, next: Some(Box::new(ListNode { val: 4, next: None })) })) }));
    // let list2 = Some(Box::new(ListNode { val: 1, next: Some(Box::new(ListNode { val: 3, next: Some(Box::new(ListNode { val: 4, next: None })) })) }));

    // println!("{:#?}", merge_two_sorted_lists::Solution::merge_two_lists(list1, list2));

    println!("{} == 5", remove_duplicates_from_sorted_array::Solution::remove_duplicates(&mut vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4]));

    let mut count_intervals = count_integers_in_intervals::CountIntervals::new();
    count_intervals.add(2, 3);
    count_intervals.add(7, 10);
    println!("{} == 6", count_intervals.count());
    count_intervals.add(5, 8);
    println!("{} == 8", count_intervals.count());
    let mut count_intervals = count_integers_in_intervals::CountIntervals::new();
    println!("{} == 0", count_intervals.count());
    count_intervals.add(1, 1000000000);
    println!("{} == 1000000000", count_intervals.count());

    println!("{} == 321", reverse_integer::Solution::reverse(123));
    println!("{} == 0", reverse_integer::Solution::reverse(1534236469));

    println!("{} == 5F3Z-2E9W", license_key_formatting::Solution::license_key_formatting("5F3Z-2e-9-w".to_string(), 4));

    println!("{:?} == 1,0,0", plus_one::Solution::plus_one(vec![9, 9]));


    println!("{} == 1", climbing_stairs::Solution::climb_stairs(1));
    println!("{} == 2", climbing_stairs::Solution::climb_stairs(2));
    println!("{} == 3", climbing_stairs::Solution::climb_stairs(3));

    let mut lru_cache = lru_cache::LRUCache::new(2);
    lru_cache.put(1, 1);
    lru_cache.put(2, 2);
    println!("{} == 1", lru_cache.get(1));
    lru_cache.put(3, 3);
    println!("{} == -1", lru_cache.get(2));
    lru_cache.put(4, 4);
    println!("{} == -1", lru_cache.get(1));
    println!("{} == 3", lru_cache.get(3));
    println!("{} == 4", lru_cache.get(4));

    println!("{} == PAHNAPLSIIGYIR", zigzag_conversion::Solution::convert("PAYPALISHIRING".to_string(), 3));

    println!("{}==9534330", largest_number::Solution::largest_number(vec![3, 30, 34, 5, 9]));

    println!("{}==MMMDCCXLIX", integer_to_roman::Solution::int_to_roman(3749));

    println!("{}==2", majority_element::Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]));

    println!("{}==13", water_bottles::Solution::num_water_bottles(9, 3));
}
