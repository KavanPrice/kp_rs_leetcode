/*
    ======================
             Lib
    ======================
*/
use crate::models::ListNode;

mod models;

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

/*
    ======================
            Tests
    ======================
*/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), Some((0, 1)));
        assert_eq!(two_sum(vec![3, 2, 4], 6), Some((1, 2)));
        assert_eq!(two_sum(vec![3, 3], 6), Some((0, 1)));
    }

    #[test]
    fn test_add_two_numbers() {
        {
            let l1_3 = ListNode { val: 3, next: None };
            let l1_2 = ListNode {
                val: 4,
                next: Some(Box::new(l1_3)),
            };
            let l1 = Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(l1_2)),
            }));

            let l2_3 = ListNode { val: 4, next: None };
            let l2_2 = ListNode {
                val: 6,
                next: Some(Box::new(l2_3)),
            };
            let l2 = Some(Box::new(ListNode {
                val: 5,
                next: Some(Box::new(l2_2)),
            }));

            let lans3 = ListNode { val: 8, next: None };
            let lans2 = ListNode {
                val: 0,
                next: Some(Box::new(lans3)),
            };
            let lans = Some(Box::new(ListNode {
                val: 7,
                next: Some(Box::new(lans2)),
            }));

            assert_eq!(add_two_numbers(l1, l2), lans);
        }

        {
            let l1 = Some(Box::new(ListNode { val: 0, next: None }));
            let l2 = Some(Box::new(ListNode { val: 0, next: None }));
            let lans = Some(Box::new(ListNode { val: 0, next: None }));

            assert_eq!(add_two_numbers(l1, l2), lans)
        }

        {}
    }
}
