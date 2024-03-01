/*
    ======================
            Tests
    ======================
*/
use crate::models::ListNode;
use crate::{add_two_numbers, two_sum};

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