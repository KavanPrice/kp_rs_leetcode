#[cfg(test)]
mod tests {
    use crate::*;

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

        {
            let l1_7 = ListNode { val: 9, next: None };
            let l1_6 = ListNode {
                val: 9,
                next: Some(Box::new(l1_7)),
            };
            let l1_5 = ListNode {
                val: 9,
                next: Some(Box::new(l1_6)),
            };
            let l1_4 = ListNode {
                val: 9,
                next: Some(Box::new(l1_5)),
            };
            let l1_3 = ListNode {
                val: 9,
                next: Some(Box::new(l1_4)),
            };
            let l1_2 = ListNode {
                val: 9,
                next: Some(Box::new(l1_3)),
            };
            let l1 = Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(l1_2)),
            }));

            let l2_4 = ListNode { val: 9, next: None };
            let l2_3 = ListNode {
                val: 9,
                next: Some(Box::new(l2_4)),
            };
            let l2_2 = ListNode {
                val: 9,
                next: Some(Box::new(l2_3)),
            };
            let l2 = Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(l2_2)),
            }));

            let lans8 = ListNode { val: 1, next: None };
            let lans7 = ListNode {
                val: 0,
                next: Some(Box::new(lans8)),
            };
            let lans6 = ListNode {
                val: 0,
                next: Some(Box::new(lans7)),
            };
            let lans5 = ListNode {
                val: 0,
                next: Some(Box::new(lans6)),
            };
            let lans4 = ListNode {
                val: 9,
                next: Some(Box::new(lans5)),
            };
            let lans3 = ListNode {
                val: 9,
                next: Some(Box::new(lans4)),
            };
            let lans2 = ListNode {
                val: 9,
                next: Some(Box::new(lans3)),
            };
            let lans = Some(Box::new(ListNode {
                val: 8,
                next: Some(Box::new(lans2)),
            }));

            assert_eq!(add_two_numbers(l1, l2), lans);
        }
    }

    #[test]
    fn test_length_of_longest_substring() {
        assert_eq!(length_of_longest_substring("abcabcbb".into()), 3);
        assert_eq!(length_of_longest_substring("bbbbb".into()), 1);
        assert_eq!(length_of_longest_substring("pwwkew".into()), 3);
    }

    #[test]
    fn test_find_median_sorted_arrays() {
        assert_eq!(find_median_sorted_arrays(vec![1, 3], vec![2]), 2f64);
        assert_eq!(find_median_sorted_arrays(vec![1, 2], vec![3, 4]), 2.5);
    }
}
