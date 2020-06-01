use crate::Solution;

use itertools::Itertools;

impl Solution {
    /// NOTE: The solution algorithm is as follows:
    /// 1. Coalesce same elements into (value, frequency).
    /// 2. Perform adjacent difference, yielding (key, (left frequency, right frequency)).
    ///    Key here is the (min(left value, right value), max(left value, right value)).
    /// 3. Coalesce again using the key to match
    /// 4. Fold over the groups, summing the left frequency.
    ///    Except for the last element in each group where we have to add the right frequency too.
    /// 5. Return max.
    pub fn total_fruit(vec: Vec<i32>) -> i32 {
        let _coalesce = vec.into_iter().map(|x| (x, 1)).coalesce(|x, y| {
            if x.0 == y.0 {
                Ok((x.0, x.1 + 1))
            } else {
                Err((x, y))
            }
        });
        0
    }
}

#[test]
fn problem_904_test_1() {
    assert_eq!(Solution::total_fruit(vec![1, 2, 1]), 3);
}

#[test]
fn problem_904_test_2() {
    assert_eq!(Solution::total_fruit(vec![0, 1, 2, 2]), 3);
}
#[test]
fn problem_904_test_3() {
    assert_eq!(Solution::total_fruit(vec![1, 2, 3, 2, 2]), 4);
}
#[test]
fn problem_904_test_4() {
    assert_eq!(
        Solution::total_fruit(vec![3, 3, 3, 1, 2, 1, 1, 2, 3, 3, 4]),
        5
    );
}
#[test]
fn problem_904_test_5() {
    assert_eq!(Solution::total_fruit(vec![1, 0, 1, 4, 1, 4, 1, 2, 3]), 5);
}
