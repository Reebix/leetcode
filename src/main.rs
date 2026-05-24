mod binary_prefix_divisible_by_5;
mod check_if_all_1s_are_at_least_length_k_places_away;
mod check_if_digits_are_equal_in_string_after_operations_i;
mod climbing_stairs;
mod container_with_most_water;
mod count_integers_in_intervals;
mod delete_nodes_from_linked_list_present_in_array;
mod divide_two_integers;
mod final_value_of_variable_after_performing_operations;
mod find_lucky_integer_in_an_array;
mod find_minimum_in_rotated_sorted_array_ii;
mod find_minimum_operations_to_make_all_elements_divisible_by_three;
mod find_pivot_index;
mod find_the_index_of_the_first_occurrence_in_a_string;
mod find_the_length_of_the_longest_common_prefix;
mod find_the_prefix_common_array_of_two_arrays;
mod group_anagrams;
mod integer_to_roman;
mod jump_game_iii;
mod jump_game_iv;
mod keep_multiplying_found_values_by_two;
mod largest_number;
mod legth_of_last_word;
mod license_key_formatting;
mod longest_common_prefix;
mod longest_substring_without_repeating_characters;
mod lru_cache;
mod majority_element;
mod majority_element_ii;
mod merge_two_sorted_lists;
mod minimum_common_value;
mod plus_one;
mod power_of_four;
mod power_of_two;
mod powx_n;
mod remove_duplicates_from_sorted_array;
mod remove_element;
mod reverse_integer;
mod reverse_string;
mod reverse_words_in_a_string;
mod reverse_words_in_a_string_iii;
mod roman_to_integer;
mod rotate_array;
mod rotate_image;
mod search_in_rotated_sorted_array;
mod single_number;
mod sort_colors;
mod uses_cache;
mod valid_parentheses;
mod water_bottles;
mod water_bottles_ii;
mod zigzag_conversion;
mod check_if_array_is_sorted_and_rotated;
mod jump_game_v;

fn main() {
    println!(
        "{}",
        roman_to_integer::Solution::roman_to_int("MCMXCIV".to_string())
    );

    println!(
        "{}",
        longest_common_prefix::Solution::longest_common_prefix(vec![
            "flower".parse().unwrap(),
            "flow".parse().unwrap(),
            "flight".parse().unwrap()
        ])
    );
    println!(
        "{}",
        longest_common_prefix::Solution::longest_common_prefix(vec!["a".parse().unwrap()])
    );

    println!(
        "{} == true",
        valid_parentheses::Solution::is_valid("()".to_string())
    );
    println!(
        "{} == true",
        valid_parentheses::Solution::is_valid("()[]{}".to_string())
    );
    println!(
        "{} == true",
        valid_parentheses::Solution::is_valid("([])".to_string())
    );
    println!(
        "{} == false",
        valid_parentheses::Solution::is_valid("([)]".to_string())
    );

    // let list1 = Some(Box::new(ListNode { val: 1, next: Some(Box::new(ListNode { val: 2, next: Some(Box::new(ListNode { val: 4, next: None })) })) }));
    // let list2 = Some(Box::new(ListNode { val: 1, next: Some(Box::new(ListNode { val: 3, next: Some(Box::new(ListNode { val: 4, next: None })) })) }));

    // println!("{:#?}", merge_two_sorted_lists::Solution::merge_two_lists(list1, list2));

    println!(
        "{} == 5",
        remove_duplicates_from_sorted_array::Solution::remove_duplicates(&mut vec![
            0, 0, 1, 1, 1, 2, 2, 3, 3, 4
        ])
    );

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

    println!(
        "{} == 5F3Z-2E9W",
        license_key_formatting::Solution::license_key_formatting("5F3Z-2e-9-w".to_string(), 4)
    );

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

    println!(
        "{} == PAHNAPLSIIGYIR",
        zigzag_conversion::Solution::convert("PAYPALISHIRING".to_string(), 3)
    );

    println!(
        "{}==9534330",
        largest_number::Solution::largest_number(vec![3, 30, 34, 5, 9])
    );

    println!(
        "{}==MMMDCCXLIX",
        integer_to_roman::Solution::int_to_roman(3749)
    );

    println!(
        "{}==2",
        majority_element::Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2])
    );

    println!("{}==13", water_bottles::Solution::num_water_bottles(9, 3));

    println!(
        "{}==2",
        find_lucky_integer_in_an_array::Solution::find_lucky(vec![2, 2, 3, 4])
    );
    println!(
        "{}==3",
        find_lucky_integer_in_an_array::Solution::find_lucky(vec![1, 2, 2, 3, 3, 3])
    );

    println!("{}==3", find_minimum_operations_to_make_all_elements_divisible_by_three::Solution::minimum_operations(vec![1, 2, 3, 4]));
    println!("{}==0", find_minimum_operations_to_make_all_elements_divisible_by_three::Solution::minimum_operations(vec![3, 6, 9]));

    println!(
        "{}==1",
        find_minimum_in_rotated_sorted_array_ii::Solution::find_min(vec![1, 3, 5])
    );

    println!(
        "{}==3",
        find_pivot_index::Solution::pivot_index(vec![1, 7, 3, 6, 5, 6])
    );

    println!(
        "{:?}==[['bat'],['nat','tan'],['ate','eat','tea']]",
        group_anagrams::Solution::group_anagrams(vec![
            "eat".parse().unwrap(),
            "tea".parse().unwrap(),
            "tan".parse().unwrap(),
            "ate".parse().unwrap(),
            "nat".parse().unwrap(),
            "bat".parse().unwrap()
        ])
    );

    println!(
        "{:?}==true",
        jump_game_iii::Solution::can_reach(vec![4, 2, 3, 0, 3, 1, 2], 5)
    );

    println!(
        "{:?}==3",
        jump_game_iv::Solution::min_jumps(vec![100, -23, -23, 404, 100, 23, 23, 23, 3, 404])
    );

    println!(
        "{:?}==1",
        jump_game_iv::Solution::min_jumps(vec![7, 6, 9, 6, 9, 6, 9, 7])
    );

    println!(
        "{:?}==2",
        minimum_common_value::Solution::get_common(vec![1, 2, 3], vec![2, 4])
    );

    println!(
        "{:?}==[0,2,3,4]",
        find_the_prefix_common_array_of_two_arrays::Solution::find_the_prefix_common_array(
            vec![1, 3, 2, 4],
            vec![3, 1, 2, 4]
        )
    );

    println!(
        "{:?}==3",
        find_the_length_of_the_longest_common_prefix::Solution::longest_common_prefix(
            vec![1, 10, 100],
            vec![1000]
        )
    );

    println!(
        "{:?}==3",
        search_in_rotated_sorted_array::Solution::search(vec![4, 5, 6, 7, 8, 9, 1, 2, 3], 1)
    );

    println!(
        "{:?}==true",
        check_if_array_is_sorted_and_rotated::Solution::check(vec![3, 4, 5, 1, 2])
    );

    println!(
        "{:?}==4",
        jump_game_v::Solution::max_jumps(vec![6, 4, 14, 6, 8, 13, 9, 7, 10, 6, 12], 2)
    );
    println!(
        "{:?}==7",
        jump_game_v::Solution::max_jumps(vec![1, 2, 3, 4, 5, 6, 7], 1)
    );
    println!(
        "{:?}==1",
        jump_game_v::Solution::max_jumps(vec![3, 3, 3, 3], 1)
    );
    println!(
        "{:?}==11",
        jump_game_v::Solution::max_jumps(vec![10, 86, 10, 1, 41, 35, 42, 83, 14, 89, 48, 26, 44, 19, 92, 47, 63, 42, 58, 92, 11, 4, 38, 11, 50, 48, 33, 61, 54, 81, 7, 16, 95, 25, 54, 24, 70, 76, 60, 96, 25, 67, 71, 20, 10, 36, 26, 22, 9, 31, 32, 56, 21, 96, 98, 55, 84, 67, 60, 49, 22, 88, 39, 97, 95, 99, 11, 16, 6, 99, 28, 15, 67, 41, 80, 35, 27, 27, 80, 72, 42, 71, 82, 81, 13, 53, 86, 60, 2, 97, 86, 96, 25, 36, 71, 17, 95, 81, 45], 16)
    );
}
