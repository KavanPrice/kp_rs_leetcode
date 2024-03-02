use crate::models::ListNode;
use std::cmp::max;

mod models;
mod test;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Option<(usize, usize)> {
    for i in 0..nums.len() - 1 {
        for j in i + 1..nums.len() {
            if nums[i] + nums[j] == target {
                return Some((i, j));
            }
        }
    }
    None
}

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    fn node_list_to_vec(head: Box<ListNode>, acc: &mut Vec<i32>) -> Vec<i32> {
        acc.push(head.val);
        match head.next {
            Some(child) => node_list_to_vec(child, acc),
            None => acc.clone(),
        }
    }

    fn vec_to_i32(digits_vec: &[i32]) -> i32 {
        let mut num: i32 = 0;
        for (power, digit) in digits_vec.iter().enumerate() {
            num += digit * 10_i32.pow(power as u32);
        }
        num
    }

    fn i32_to_vec(num: i32) -> Vec<i32> {
        num.to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .rev()
            .collect()
    }

    fn vec_to_node_list(num_vec: &[i32]) -> Option<Box<ListNode>> {
        if let Some(num) = num_vec.first() {
            let mut node = Box::new(ListNode::new(*num));
            node.next = vec_to_node_list(&num_vec[1..]);
            return Some(node);
        }
        None
    }

    let mut l1_vec: Vec<i32> = Vec::new();
    if let Some(l1_head) = l1 {
        l1_vec = node_list_to_vec(l1_head, &mut vec![]);
    }
    let num1 = vec_to_i32(&l1_vec);

    let mut l2_vec: Vec<i32> = Vec::new();
    if let Some(l2_head) = l2 {
        l2_vec = node_list_to_vec(l2_head, &mut vec![]);
    }
    let num2 = vec_to_i32(&l2_vec);

    let sum = num1 + num2;
    let sum_vec = i32_to_vec(sum);
    vec_to_node_list(&sum_vec)
}

pub fn length_of_longest_substring(s: String) -> i32 {
    let chars_vec = s.chars();
    let mut longest_length = 0;
    let mut current_length = 0;
    let mut past_chars: Vec<char> = vec![];

    for char in chars_vec {
        if past_chars.contains(&char) {
            longest_length = max(longest_length, current_length);
            current_length = 1;
        } else {
            current_length += 1;
            past_chars.push(char);
        }
    }
    longest_length
}

pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let mut sorted = nums1.clone();
    let mut nums1_pointer: usize = 0;
    let mut nums2_pointer: usize = 0;

    while nums1_pointer < sorted.len() {
        while nums2_pointer < nums2.len() {
            if sorted[nums1_pointer] < nums2[nums2_pointer] {
                sorted.insert(nums1_pointer + 1, nums2[nums2_pointer]);
                nums2_pointer += 1;
            } else {
                nums1_pointer += 1;
            }
        }
    }

    let l_median = sorted
        .get((sorted.len() as f64 / 2_f64).floor() as usize)
        .unwrap();
    let u_median = sorted
        .get((sorted.len() as f64 / 2_f64).ceil() as usize)
        .unwrap();
    (*l_median as f64 + *u_median as f64) / 2f64
}

pub fn longest_palindrome(s: String) -> String {
    trait PossiblePalindrome {
        fn is_palindrome(&self) -> bool;
    }
    impl PossiblePalindrome for String {
        fn is_palindrome(&self) -> bool {
            self == &self.chars().rev().collect::<String>()
        }
    }

    let mut longest_palindrome: String = "".into();
    for i in 0..s.len()-1 {
        for j in i..s.len() {
            let substring = s[i..j].to_string();
            if substring.is_palindrome() && substring.len() > longest_palindrome.len() {
                longest_palindrome = substring;
            }
        }
    }
    longest_palindrome
}
